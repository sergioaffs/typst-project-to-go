# Typst-project-to-go

This is a utility for [Typst CLI](https://github.com/typst/typst) users.

Users who develop Typst templates and functions for personal use benefit greatly from local packages, that is, packages located in their own computer. [Local packages](https://github.com/typst/packages) are stored in a specific location of the user's operating system and, from there, can be reused in different projects. This is great for single users, but makes collaboration somewhat harder because new collaborators would need to create a copy of the local package repository before starting to collaborate. This may not be too complex, but it does add a bit of friction compared to just telling the would-be collaborators "run `typst c filename.typ`".

Enter `typst-project-to-go`.

This simple CLI tool works on Typst projects: it identifies all local packages used by a given project, creates a copy of said packages inside the project and modifies the references. The new version of the project is portable: the new user only needs to run `typst` to start working.

## How to use

For now, I'm not shipping any compiled artifacts, so you'll need Rust's `cargo` on your device. That should be the only tricky part for now. 

If you have `cargo`installed, run

```bash
cargo run -- <origin-folder> <target-folder>
```

where `<origin-folder>` is the location of the Typst file you want to package to go, and `<target-folder>` is the location where the packaged version will be created.
