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
            write_html_to_file(&output_file, file_ast, indent.into());
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
            file_watcher.watch(input_file_path, RecursiveMode::NonRecursive).unwrap();
            println!("Watching for file changes...");

            for data in rx {
                match data {
                    Ok(event) => {
                        match &event.kind {
                            notify::EventKind::Modify(_) => {
                                let file_ast = read_file_into_ast(&input_file);
                                write_html_to_file(&output_file, file_ast, indent.into());
                                println!("Compiled {in} into {out}", in = input_file, out = output_file);
                            },
                            _ => (),

                        }
                    },
                    Err(e) => println!("{e}"),
                }
            }

            Ok(())
        }
    }
}
