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
            compile_ytml_file(input_file, output_file, indent.into());
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
                                // Watch for all files in the directory
                                let input_file_paths = event.paths;
                                for path in input_file_paths {
                                    if path.extension().unwrap() == "ytml" {
                                        let out = compile_ytml_file(
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

fn compile_ytml_file(input_path: String, output_path: Option<String>, indent: usize) -> String {
    let actual_output_path = output_path.unwrap_or(
        Path::new(&input_path)
            .with_extension("html")
            .to_str()
            .unwrap()
            .to_owned(),
    );
    let ast = read_file_into_ast(&input_path);
    write_html_to_file(&actual_output_path, ast, indent);
    actual_output_path
}
