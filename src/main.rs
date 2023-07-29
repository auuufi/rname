use argparse::{ArgumentParser, List, Print, Store};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{
    fs::rename,
    path::{Path, PathBuf},
    process::exit,
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
        exit(1);
    }
}

fn main() {
    let mut files: Vec<PathBuf> = Vec::new();
    let mut length: usize = 20;

    {
        let mut parser = ArgumentParser::new();

        parser.set_description("Rename files with random names");
        parser
            .refer(&mut files)
            .add_argument("<FILES>", List, "List of files to be renamed")
            .required();
        parser
            .refer(&mut length)
            .add_option(
                &["-l", "--length"],
                Store,
                "Random name length (default: 20)",
            )
            .metavar("<LENGTH>");
        parser.add_option(
            &["-v", "--version"],
            Print(env!("CARGO_PKG_VERSION").to_string()),
            "Print version information",
        );

        match parser.parse_args() {
            Ok(()) => {}
            Err(err) => {
                exit(err);
            }
        }
    }

    if files.is_empty() {
        eprintln!("No files provided for renaming");
        exit(1);
    }

    for current_name in files {
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
