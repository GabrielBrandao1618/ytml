mod ast;
mod cli;
mod file_input;
mod file_output;
mod html;
mod ytml;

use clap::Parser;

use file_input::read_file_into_ast;
use file_output::write_html_to_file;

use cli::{Cli, Command};

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Parse {
            input_file,
            output_file,
            indent
        } => {
            let file_ast = read_file_into_ast(&input_file);
            write_html_to_file(&output_file, file_ast, indent.into());
        }
    }
}
