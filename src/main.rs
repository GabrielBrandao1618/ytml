mod ast;
mod cli;
mod file_input;
mod file_output;
mod html;
mod ytml;

use clap::Parser;

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;

use file_input::read_file_into_ast;
use file_output::write_html_to_file;

use cli::{Cli, Command};

fn main() -> notify::Result<()> {
    let args = Cli::parse();
    match args.command {
        Command::Parse {
            input_file,
            output_file,
            indent,
        } => {
            let file_ast = read_file_into_ast(&input_file);
            let output_file_path = output_file.unwrap_or(
                Path::new(&input_file)
                    .with_extension("html")
                    .to_str()
                    .unwrap()
                    .to_owned(),
            );
            write_html_to_file(&output_file_path, file_ast, indent.into());
            Ok(())
        }

        Command::Watch {
            input_file,
            output_file,
            indent,
        } => {
            let (tx, rx) = channel();

            let input_file_path = Path::new(&input_file);
            let actual_output = output_file.unwrap_or(
                input_file_path
                    .with_extension("html")
                    .to_str()
                    .unwrap()
                    .to_owned(),
            );
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
                                let file_ast = read_file_into_ast(&input_file);
                                write_html_to_file(&actual_output, file_ast, indent.into());
                                println!("Compiled {in} into {out}", in = input_file, out = actual_output);
                            } else if input_file_path.is_dir() {
                                let input_file_paths = event.paths;
                                for path in input_file_paths {
                                    if path.extension().unwrap() == "ytml" {
                                        let file_ast = read_file_into_ast(path.to_str().unwrap());
                                        let output_path = path.with_extension("html");
                                        write_html_to_file(
                                            output_path.to_str().unwrap(),
                                            file_ast,
                                            indent.into(),
                                        );
                                        println!("Compiled {in} into {out}", in = path.to_str().unwrap(), out = output_path.to_str().unwrap());
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
