//! This script takes a Typst project as input. It identifies any local packages referenced in the project and adds them to the project folder. The idea is to make the project portable.
//!
//! The general flow goes as follows:
//! 1. Identify all files in folder (up to ~5 levels of recursion)
//! 2. Create a copy of the folder structure
//! 3. For each file with .typ extension, review the file. For each reference to a local Typst package:
//!   + Point reference to a relative folder called `pckgs` (make name parameterizable).
//!   + Copy content of package into `pkcgs/package-name`

use inquire::Confirm;
use std::fs;
use std::io::prelude::*;
use std::io::{BufRead, LineWriter};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use clap::Parser;
use toml::Table;
use tracing::{Level, debug, error, info, span};

#[derive(Debug)]
struct TypstImport<'a> {
    package_name: &'a str,
    version: &'a str,
    imports: &'a str,
}

trait TypstConversion {
    fn format_import_relative(self) -> String;
}

impl<'a> TypstConversion for TypstImport<'a> {
    // Expected output format: "#import "pckgs/<package_name>/<version>: <imports>" "
    fn format_import_relative(self) -> String {
        let entrypoint = {
            let typst_toml = get_package_location()
                .join(self.package_name)
                .join(self.version)
                .join("typst.toml");
            debug!(?typst_toml);

            let mut file = fs::File::open(typst_toml).expect("Failed to open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Could not find typst.toml");

            let package_config = contents.parse::<Table>().unwrap();
            debug!(?package_config);

            match package_config["package"]["entrypoint"].as_str() {
                Some(entrypoint) => String::from(entrypoint),
                None => panic!("Could not find `entrypoint` in typst.toml"),
            }
        };
        format!(
            r#"#import "/pckgs/{}/{}/{}": {}"#,
            self.package_name,
            self.version,
            entrypoint,
            self.imports.trim()
        )
    }
}

fn copy_folder(source_folder: &Path, destination_folder: &Path) -> Result<(), std::io::Error> {
    let d_span = span!(Level::DEBUG, "COPYING");
    let _guard = d_span.enter();

    if !destination_folder.exists() {
        fs::create_dir(destination_folder)?;
    }

    debug!(?source_folder, ?destination_folder);

    for node in WalkDir::new(source_folder) {
        let entry = node?;
        let source_entry_path = entry.path();
        let relative_target_path = source_entry_path
            .strip_prefix(source_folder)
            .expect("The prefix should always match");

        let full_target_path = PathBuf::from(destination_folder).join(relative_target_path);

        if source_entry_path.is_dir() && !full_target_path.exists() {
            fs::create_dir(&full_target_path)?;
            debug!(new_folder=?full_target_path, "Created new directory");
        } else if source_entry_path.is_file() {
            fs::copy(&source_entry_path, &full_target_path)?;
            debug!(src=?source_entry_path, dst=?full_target_path, "COPIED");
        }
    }

    Ok(())
}

/// From https://github.com/typst/packages:
/// - $XDG_DATA_HOME or ~/.local/share on Linux
/// - ~/Library/Application Support on macOS
/// - %APPDATA% on Windows
fn get_package_location() -> PathBuf {
    let package_base_location = match std::env::consts::OS {
        "linux" => match std::env::var("XDG_DATA_HOME") {
            Ok(env_variable) => PathBuf::from(env_variable),
            Err(_) => {
                let home_path =
                    std::env::var("HOME").unwrap_or(String::from("Could not resolve $HOME"));
                PathBuf::from(home_path).join(".local/share")
            }
        },
        "windows" => {
            let home_path =
                std::env::var("APPDATA").unwrap_or(String::from("Could not resolve $HOME"));
            PathBuf::from(home_path)
        }
        "macos" => {
            let home_path =
                std::env::var("HOME").unwrap_or(String::from("Could not resolve $HOME"));
            PathBuf::from(home_path).join("Library/Application Support")
        }
        unrecognized_os_name => panic!(
            "The os `{}` is not supported by Typst",
            unrecognized_os_name
        ),
    };

    package_base_location.join("typst/packages/local")
}

/// 1. Find where the local package is located in the filesystem
/// 2. Copy the local package into the project
fn create_relative_package(
    package_details: &TypstImport,
    relative_package_folder: &Path,
) -> Result<(), std::io::Error> {
    let local_packages_folder = get_package_location();
    debug!(?local_packages_folder);

    let origin_package_folder = local_packages_folder.join(package_details.package_name);
    let destination_package_folder = relative_package_folder.join(package_details.package_name);

    if !destination_package_folder.exists() {
        debug!(?destination_package_folder);
        fs::create_dir_all(&destination_package_folder).expect("Could not create the folder");
    }

    copy_folder(
        origin_package_folder
            .join(package_details.version)
            .as_path(),
        destination_package_folder
            .join(package_details.version)
            .as_path(),
    )?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum TypstFileParserError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    RegexError(#[from] regex::Error),
}

fn parse_line(line: &String, dest_pckg_path: &Path) -> String {
    let re = regex::Regex::new(r#"\s*#import\s+"@local/([^\s:]+):(\w+.\w+.\w+)"\s?(:\s*(.+))?"#)
        .unwrap();
    match re.captures(&line) {
        Some(matched_line) => {
            let package_name = matched_line
                .get(1)
                .expect("Match must have content")
                .as_str();
            let version = matched_line
                .get(2)
                .expect("Match must have content")
                .as_str();
            let imports = match matched_line.get(4) {
                Some(match_group) => match_group.as_str(),
                None => "*",
            };

            let typst_package_details = TypstImport {
                package_name: &package_name,
                version: &version,
                imports: &imports,
            };
            match create_relative_package(&typst_package_details, &dest_pckg_path) {
                Ok(_) => typst_package_details.format_import_relative(),
                Err(_) => String::from(r#"#panic!("The package could not be moved!")"#),
            }
        }
        None => line.to_owned(),
    }
}

/// Typst files may contain references to local (external) packages. To wrap them in:
/// - Identify any calls to local packages. For each such call:
///   - Replace the call so that it points to a subdirectory rather than a local package
///   - Copy the local package into that subdirectory
/// - Copy the (possibly modified) Typst file in the destination path replicating the original file structure.
fn process_typst_file(
    source_file_path: &Path,
    dest_file_path: &Path,
    dest_pckg_path: &Path,
) -> Result<(), TypstFileParserError> {
    let i_span = span!(Level::INFO, "TYP conversion");
    let _guard = i_span.enter();

    //Read file line by line
    let source_file = std::fs::File::open(source_file_path)
        .expect("File should exist unless someone is messing with the filesystem");
    let reader = std::io::BufReader::new(&source_file);

    let dest_file = std::fs::File::create(dest_file_path)?;
    let mut writer = LineWriter::new(dest_file);
    //MARKER
    // Sample import line:
    // #import "@local/package:2025.1.1": *
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| parse_line(&line, &dest_pckg_path))
        .for_each(|line| write!(writer, "{}\n", line).expect("Error copying file"));

    Ok(())
}

/// Process each path.
///   - If the file is a folder, ensure it exists in the destination project.
///   - If the file is not a Typst file, copy it in the destination path replicating the original file structure.
///   - If the file is a Typst file: `process_typst_file` the file
fn process_path(source_path: &Path, target_path: &Path, args: &Cli) {
    // If Typst: process further
    let span = span!(Level::DEBUG, "PROCESS", source = source_path.to_str());
    let _guard = span.enter();

    let target_root = &args.target_folder;

    // todo!("Consider: make local package folder name customizable")
    let packages_path = target_root.join("pckgs");

    match fs::metadata(source_path) {
        Ok(src_metadata) => {
            if src_metadata.is_dir() {
                // Case 1: file is dir
                if !target_path.exists() {
                    match fs::create_dir(target_path) {
                        Ok(_) => {
                            info!(name = ?target_path, "Folder created")
                        }
                        Err(e) => {
                            error!(?e, name = ?target_path, "Failed to create folder")
                        }
                    }
                } else {
                    if !args.overwrite_target {
                        match Confirm::new("The target folder already exists. Overwrite?")
                            .with_default(false)
                            .prompt()
                        {
                            Ok(true) => {} // go ahead
                            Ok(false) => panic!("Can't continue because the folder already exists"),
                            Err(e) => error!(?e, "Error while parsing the input"),
                        }
                    }
                }
            } else if src_metadata.is_file() {
                if source_path.extension().and_then(std::ffi::OsStr::to_str) == Some("typ") {
                    // Case 2: Typst file
                    match process_typst_file(source_path, target_path, &packages_path) {
                        Ok(_) => debug!("Typst file processed"),
                        Err(e) => match e {
                            TypstFileParserError::IOError(inner_e) => {
                                error!(?inner_e, "IOError: The file could not be processed")
                            }
                            TypstFileParserError::RegexError(inner_e) => {
                                error!(
                                    ?inner_e,
                                    "RegexError: Error while looking for imports in Typst file"
                                )
                            }
                        },
                    }
                } else {
                    match fs::copy(source_path, target_path) {
                        // Case 3: any other file
                        Ok(_) => {}
                        Err(e) => {
                            error!(?e, ?source_path, ?target_path, "Error copying file")
                        }
                    }
                }
            }
        }
        Err(e) => error!(?e, "Could not get file metadata"),
    }
}

fn package_folder_to_go(args: &Cli) {
    let source_folder = args.source_folder.as_path();
    let target_folder = args.target_folder.as_path();

    let original_file_structure = WalkDir::new(&source_folder).max_depth(5);
    original_file_structure
        .into_iter()
        .map(|entry_res| {
            let entry = entry_res
                .unwrap_or_else(|_| panic!("Error while browsing the source folder. Ensure the path is valid and accessible."));
            let relative_target_path = entry
                .path()
                .strip_prefix(&source_folder)
                .expect("The prefix should always be removable");
            let mut full_new_path = PathBuf::new();
            full_new_path.push(&target_folder);
            full_new_path.push(relative_target_path);

            (entry, full_new_path)
        })
        .for_each(|entry| process_path(entry.0.path(), entry.1.as_path(), &args));
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    source_folder: PathBuf,

    target_folder: PathBuf,

    /// If the target exists, overwrite content
    #[arg(short)]
    overwrite_target: bool,
}

/// Explore the input project and process each file on it. (`process` defines what to do with
/// each file)
fn main() {
    tracing_subscriber::fmt::init();

    let args = Cli::parse();
    info!(?args, "Starting");

    package_folder_to_go(&args);

    info!("✔️ Finished successfully");
    // todo!("Test on Windows");
    // todo!(
    // "Generic packages for testing (e.g. single file, multiple versions, complex up to 5 levels)"
    // );
    // todo!("Unit tests");
    // todo!("Compile");
}

#[cfg(test)]
mod tests {
    use sha2::{Digest, Sha256};
    use std::fs;
    use std::io::{BufReader, Read};
    use std::path::{Path, PathBuf};

    use tracing::debug;

    use crate::{Cli, package_folder_to_go, parse_line};

    #[test]
    /// Basic test: different files on a single level (no nested folders).
    /// There is one Typst file but with no imports (it should be identical to source).
    fn basic() {
        let source_folder = PathBuf::from("./test-data/1-basic");
        let target_folder = PathBuf::from("./test-output/1-basic");
        let file_digests = vec![
            (
                "single.typ",
                "e902ec256d1f5b5f7f6ccb9aba5596d641726dd180997ead28c8b8943977aef8",
            ),
            (
                "mime-fencing.jpg",
                "7f569bef477ec154405ab76d3a422e015f0a9db5e9a4cf7ad61537396ebfbebc",
            ),
            (
                "SourceSans3-BoldIt.otf",
                "05e97d9adc01059607e97b167a68afb25680db904db7aba831dc2510f14b6515",
            ),
        ];
        // let target_single_file = target_folder.join("single.typ");

        if target_folder.exists() {
            fs::remove_dir_all(&target_folder).expect("The file should be removable.");
        }

        fs::create_dir(&target_folder)
            .expect("Could not create target directory. Check permissions and rerun the tests");
        // if target_single_file.exists() {
        //     fs::remove_file(&target_single_file).expect("The file should be removable.");
        // }
        let args = Cli {
            source_folder: source_folder,
            target_folder: target_folder,
            overwrite_target: true,
        };

        package_folder_to_go(&args);

        for file_digest in file_digests {
            assert_eq!(
                sha256_digest(&args.target_folder.join(file_digest.0)).unwrap_or(String::from(
                    "It should be possible to get the digest of that file"
                )),
                String::from(file_digest.1)
            );
        }
    }

    #[test]
    fn line_parsing() {
        // For this test, there must be packages with the right names and configuration in the Typst package folder. A copy of the packages expected is located in the folder `test-local-packages`
        let dest_pckg_path = Path::new(".");
        let inputs = vec![
            "test",
            "A longer line with unexpected characters:⚡😅🍣ラメン",
            r#"#import "lettertemp.typ": *"#,
            r#"#import "@local/simple-package:2025.1.0": *"#,
            r#"         #import "@local/simple-package:2025.1.0": *       "#,
            r#"#import "@local/simple-package:2025.1.0": i1"#,
            r#"#import "@local/simple-package:2025.1.0": i1, i2"#,
            // r#"#import "@local/a-package:2025.1.0": *"#,
        ];
        let expected_outputs = vec![
            "test",                                                           // same as input
            "A longer line with unexpected characters:⚡😅🍣ラメン",          // same as input
            r#"#import "lettertemp.typ": *"#, // same as input (project-specific import)
            r#"#import "/pckgs/simple-package/2025.1.0/entrypoint.typ": *"#, // Local package with universal import
            r#"#import "/pckgs/simple-package/2025.1.0/entrypoint.typ": *"#, // Same as previous (spaces need trimming)
            r#"#import "/pckgs/simple-package/2025.1.0/entrypoint.typ": i1"#, // Same local package as previous, one concrete import
            r#"#import "/pckgs/simple-package/2025.1.0/entrypoint.typ": i1, i2"#, // Same local package as previous, two imports
                                                                                  // r#"#import "/pckgs/a-package/2025.1.0/CV-template.typ": *"#, // Local package with universal import
        ];
        inputs
            .iter()
            .map(|s| s.to_string())
            .zip(expected_outputs.iter().map(|s| s.to_string()))
            .for_each(|(input, expected_output)| {
                let parsed = parse_line(&input, &dest_pckg_path);
                assert_eq!(parsed, expected_output);

                // println!("{} --> {}", input, parsed);
            });
        // parse_line(&"test".to_string(), dest_pckg_path);
    }
    // #[test]
    /// Flat structure: different files in a single level (no nested folders).
    // fn flat() {}

    fn sha256_digest(path: &PathBuf) -> Result<String, std::io::Error> {
        let input = fs::File::open(path)?;
        let mut reader = BufReader::new(input);

        let digest = {
            let mut hasher = Sha256::new();
            let mut buffer = [0; 1024];
            loop {
                let count = reader.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                debug!(count);
                hasher.update(&buffer[..count]);
            }
            hasher.finalize()
        };
        Ok(format!("{:x}", digest))
    }
}
