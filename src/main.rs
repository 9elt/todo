// @todo {
//    you can reference lines: [74] will print line 74
// }

// @todo {
//    or ranges: [97:100] to print lines 97 to 100
// }

// @todo @low {reference the next 2 lines [+1:+2]}
mod parser;
mod util;

use clap::Parser as ClapParser;
use parser::Parser;
use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};
use util::logger::ResultLogger;
use walkdir::WalkDir;

use crate::util::logger;

fn main() {
    parser::test();
    return;

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

        let file = match fs::read_to_string(path) {
            Ok(str) => str,
            Err(_) => continue,
        };

        if !file.contains("@todo") {
            continue;
        }

        let mut result = Parser::new(&file).parse();

        if result.len() == 0 {
            if !args.silent {
                logger::missing_todo(&relative_path(&cdd, path));
            }
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
