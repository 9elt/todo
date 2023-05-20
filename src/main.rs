// @todo @high {implement some cache}
// @todo @low {refactor main.rs mess}

mod parser;
mod util;

use clap::Parser as ClapParser;
use parser::Parser;
use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};
use util::logger::ResultLogger;
use walkdir::WalkDir;

fn main() {
    let args = util::args::Args::parse();

    let max_depth = match args.recursion {
        Some(n) => n,
        None => 5,
    };

    let mut cd = current_dir().unwrap();

    if let Some(dir) = args.dir {
        cd.push(dir);

        if !cd.exists() {
            return println!("{} does not exist", cd.to_string_lossy());
        }

        if !cd.is_dir() {
            return println!("{} is not a directory", cd.to_string_lossy());
        }
    }

    let cdd = cd.as_path().to_owned();

    let walker = WalkDir::new(cd).max_depth(max_depth);

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if args.extensions.len() > 0 {
            let ext = match path.extension() {
                Some(ext) => ext.to_string_lossy().to_string(),
                None => continue,
            };

            if !args.extensions.contains(&ext) {
                continue;
            }
        }

        let filestr = match fs::read_to_string(path) {
            Ok(str) => str,
            Err(_) => continue,
        };

        if !filestr.contains("@todo") {
            continue;
        }

        let file = filestr.as_bytes();

        let mut result = Parser::new(file.iter()).parse();

        if result.len() == 0 {
            continue;
        }

        result.sort_by_key(|(_, _, p)| p.to_u8());

        if args.important_only {
            result = result
                .iter()
                .filter(|(_, _, p)| p.is_high())
                .map(|v| v.to_owned())
                .collect();
        }

        ResultLogger::new(result)
            .filname(relative_path(&cdd, path))
            .res()
            .line();
    }
}

fn relative_path(cd: &PathBuf, path: &Path) -> String {
    path.to_string_lossy()
        .replace(cd.to_string_lossy().as_ref(), ".")
}
