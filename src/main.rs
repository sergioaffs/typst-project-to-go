//! This script takes a Typst project as input. It identifies any local packages referenced in the project and adds them to the project folder. The idea is to make the project portable.
//!
//! The general flow goes as follows:
//! 1. Identify all files in folder (up to ~5 levels of recursion)
//! 2. Create a copy of the folder structure
//! 3. For each file with .typ extension, review the file. For each reference to a local Typst package:
//!   + Point reference to a relative folder called `pckgs` (make name parameterizable).
//!   + Copy content of package into `pkcgs/package-name`

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
            r#"#import "pckgs/{}/{}/{}": {}"#,
            self.package_name, self.version, entrypoint, self.imports
        )
    }
}

fn copy_folder(source_folder: &Path, destination_folder: &Path) {
    let d_span = span!(Level::DEBUG, "COPYING");
    let _guard = d_span.enter();

    if !destination_folder.exists() {
        fs::create_dir(destination_folder).expect("The folder should have been created");
    }

    debug!(?source_folder, ?destination_folder);

    for node in WalkDir::new(source_folder) {
        match node {
            Ok(source_entry) => {
                let relative_target_path = source_entry
                    .path()
                    .strip_prefix(source_folder)
                    .expect("The prefix should always match");

                let full_target_path = PathBuf::from(destination_folder).join(relative_target_path);

                let source_entry_path = source_entry.path();

                if source_entry_path.is_dir() && !source_entry_path.exists() {
                    match fs::create_dir(&full_target_path) {
                        Ok(_) => debug!(new_folder=?full_target_path, "Created new file"),
                        Err(e) => error!(new_folder=?full_target_path, ?e, "Could not create file"),
                    }
                } else if source_entry_path.is_file() {
                    match fs::copy(&source_entry_path, &full_target_path) {
                        Ok(_) => debug!(src=?source_entry_path, dst=?full_target_path, "COPIED"),
                        Err(e) => error!(?e, "Could not copy file"),
                    }
                }
            }
            Err(e) => error!(?e),
        }
    }
}

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
        "windows" => PathBuf::from("%APPDATA%"),
        "macos" => PathBuf::from("~/Library/Application Support"),
        unrecognized_os_name => panic!(
            "The os `{}` is not supported by Typst",
            unrecognized_os_name
        ),
    };

    package_base_location.join("typst/packages/local")
}

/// 1. Find folder of local package
/// 2. Copy folder
fn create_relative_package(package_details: &TypstImport, relative_package_folder: &Path) {
    let local_packages_folder = get_package_location();
    info!(?local_packages_folder);

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
    )
}

fn process_typst_file(source_file_path: &Path, dest_file_path: &Path, dest_pckg_path: &Path) {
    let i_span = span!(Level::INFO, "TYP conversion");
    let _guard = i_span.enter();
    info!("Starting parsing");
    //Read file line by line
    let source_file = std::fs::File::open(source_file_path)
        .expect("File should exist unless someone is messing with the filesystem");
    let reader = std::io::BufReader::new(&source_file);

    let dest_file = match std::fs::File::create(dest_file_path) {
        Ok(created_file) => created_file,
        Err(e) => {
            error!("Failed to create destination file: {}", e);
            return;
        }
    };
    let mut writer = LineWriter::new(dest_file);

    // Sample import line:
    // #import "@local/package:2025.1.1": *
    let re = regex::Regex::new(r#"\s*#import\s+"@local/([^\s:]+):(\w+.\w+.\w+)"\s?(:\s*(.+))?"#)
        .unwrap();
    let lines: Vec<String> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| match re.captures(&line) {
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
                create_relative_package(&typst_package_details, &dest_pckg_path);
                typst_package_details.format_import_relative()
            }
            None => line,
        })
        .collect();

    for line in lines {
        write!(writer, "{}\r\n", line).expect("Error copying file");
    }

    info!("Ended parsing");
}

/// Process each path.
///   - If the file is a folder, ensure it exists in the destination project.
///   - If the file is not a Typst file, copy it in the destination path replicating the original file structure.
///   - If the file is a Typst file:
///      - Identify any calls to local packages. For each such call:
///        - Replace the call so that it points to a subdirectory rather than a local package
///        - Copy the local package into that subdirectory
///      - Copy the (possibly modified) Typst file in the destination path replicating the original file structure.
fn process(source_path: &Path, target_path: &Path, target_root: &Path) {
    // If Typst: process further
    let span = span!(Level::DEBUG, "PROCESS", source = source_path.to_str());
    let _guard = span.enter();

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
                }
            } else if src_metadata.is_file() {
                if source_path.extension().and_then(std::ffi::OsStr::to_str) == Some("typ") {
                    // Case 2: Typst file
                    process_typst_file(source_path, target_path, &packages_path);
                } else {
                    match fs::copy(source_path, target_path) {
                        Ok(_) => {}
                        Err(e) => {
                            error!(?e, ?source_path, ?target_path, "Error copying file")
                        }
                    }
                }
            }
        }
        Err(e) => error!(?e, "Could not open file"),
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    source_folder: PathBuf,

    target_folder: PathBuf,
}

/// Explore the input project and process each file on it. (`process` defines what to do with
/// each file)
fn main() {
    tracing_subscriber::fmt::init();

    let args = Cli::parse();
    info!(?args, "Starting");

    let original_file_structure = WalkDir::new(&args.source_folder).max_depth(5);
    original_file_structure
        .into_iter()
        .map(|entry_res| {
            let entry = entry_res
                .unwrap_or_else(|_| panic!("Unexpected error while exploring source directory"));
            let relative_target_path = entry
                .path()
                .strip_prefix(&args.source_folder)
                .expect("The prefix should always be removable");
            let mut full_new_path = PathBuf::new();
            full_new_path.push(&args.target_folder);
            full_new_path.push(relative_target_path);

            (entry, full_new_path)
        })
        .for_each(|entry| process(entry.0.path(), entry.1.as_path(), &args.target_folder));
}
