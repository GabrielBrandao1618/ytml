mod ast;
mod cli;
mod file_handling;
mod html;
mod ytml;

use clap::Parser;

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;

use file_handling::compile_ytml_file;

use cli::{Cli, Command};

fn main() -> notify::Result<()> {
    let args = Cli::parse();
    match args.command {
        Command::Parse {
            input_file,
            output_file,
            indent,
        } => {
            let (input, out) = compile_ytml_file(input_file, output_file, indent.into());
            println!("Compiled {in} into {out}", in = input, out = out);
            Ok(())
        }

        Command::Watch {
            input_file,
            output_file,
            indent,
        } => {
            let (tx, rx) = channel();

            let input_file_path = Path::new(&input_file);
            let mut file_watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
            file_watcher
                .watch(input_file_path, RecursiveMode::NonRecursive)
                .unwrap();
            println!("Watching for file changes...");

            for data in rx {
                match data {
                    Ok(event) => match &event.kind {
                        notify::EventKind::Modify(_) => {
                            if input_file_path.is_file() {
                                let (_, output) = compile_ytml_file(
                                    input_file_path.to_str().unwrap().to_owned(),
                                    output_file.clone(),
                                    indent.into(),
                                );
                                println!("Compiled {in} into {out}", in = input_file, out = output);
                            } else if input_file_path.is_dir() {
                                // Watch for all files in the directory
                                let input_file_paths = event.paths;
                                for path in input_file_paths {
                                    if path.extension().unwrap() == "ytml" {
                                        let (_, out) = compile_ytml_file(
                                            path.to_str().unwrap().to_owned(),
                                            None,
                                            indent.into(),
                                        );
                                        println!("Compiled {in} into {out}", in = path.to_str().unwrap(), out = out);
                                    }
                                }
                            }
                        }
                        _ => (),
                    },
                    Err(e) => println!("{e}"),
                }
            }

            Ok(())
        }
    }
}
