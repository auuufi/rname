use argparse::{ArgumentParser, List, Print, Store};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{
    fs::rename,
    path::{Path, PathBuf},
};

fn generate_random_name(length: usize) -> String {
    thread_rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn rename_file(current_name: &Path, new_name: &Path) {
    if let Err(err) = rename(current_name, new_name) {
        eprintln!("Error when renaming a file: {}", err);
    }
}

fn main() {
    let mut files: Vec<PathBuf> = Vec::new();
    let mut length: usize = 20;

    {
        let mut parser = ArgumentParser::new();

        parser.set_description("A command-line utility to rename files with random names");
        parser
            .refer(&mut files)
            .add_argument("<files>", List, "Specify one or more files to be renamed")
            .required();
        parser
            .refer(&mut length)
            .add_option(
                &["-l", "--length"],
                Store,
                "Set random name length (default: 20)",
            )
            .metavar("<length>");
        parser.add_option(
            &["-v", "--version"],
            Print(env!("CARGO_PKG_VERSION").to_string()),
            "Show version information and exit",
        );
        parser.parse_args_or_exit();
    }

    for current_name in &files {
        if current_name.is_file() {
            let mut new_path = current_name.parent().unwrap().to_path_buf();
            let new_name = generate_random_name(length);
            let file_extension = current_name.extension().and_then(|ext| ext.to_str());

            if let Some(ext) = file_extension {
                new_path.push(format!("{}.{}", new_name, ext));
            } else {
                new_path.push(&new_name);
            }

            rename_file(&current_name, &new_path);
        }
    }
}
