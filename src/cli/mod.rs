use clap::{Parser,Subcommand};

#[derive(Subcommand)]
pub enum Command {
    #[clap(about = "parse a ytml file into a html file")]
    Parse {
        #[arg(help = "Path to .ytml file")]
        input_file: String,
        #[arg(help = "Path to .html file")]
        output_file: String,
    }
}

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command, 
}