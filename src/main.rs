use argparse::{ArgumentParser, List, Print, Store};
use glob::glob;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{fs::rename, path::PathBuf, process::exit};

fn generate_random_name(length: usize) -> String {
    thread_rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn rename_file(current_name: &PathBuf, new_name: &PathBuf) {
    rename(current_name, new_name).expect("Failed to rename the file");
}

fn main() {
    let mut file: Vec<String> = Vec::new();
    let mut length: usize = 20;

    {
        let mut parser = ArgumentParser::new();

        parser.set_description("Renaming files with random names");
        parser
            .refer(&mut file)
            .add_argument("<FILE>", List, "Current name of the file(s)")
            .required();
        parser
            .refer(&mut length)
            .add_option(
                &["-l", "--length"],
                Store,
                "Length of the random name (default: 20)",
            )
            .metavar("<LENGTH>");
        parser.add_option(
            &["-v", "--version"],
            Print(env!("CARGO_PKG_VERSION").to_string()),
            "Show version",
        );

        match parser.parse_args() {
            Ok(()) => {}
            Err(x) => {
                exit(x);
            }
        }
    }

    let mut files: Vec<PathBuf> = Vec::new();

    for pattern in &file {
        let mut paths: Vec<PathBuf> = glob(pattern)
            .expect("Failed to read glob pattern")
            .filter_map(Result::ok)
            .collect();
        files.append(&mut paths)
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
