use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Command {
    #[clap(about = "parse a ytml file into a html file")]
    Parse {
        #[arg(help = "Path to .ytml file")]
        input_file: String,
        #[arg(help = "Path to .html file")]
        output_file: Option<String>,
        #[arg(default_value_t = 2, long)]
        indent: u8,
    },
    Watch {
        #[arg(help = "Path to .ytml file")]
        input_file: String,
        #[arg(help = "Path to .html file")]
        output_file: Option<String>,
        #[arg(default_value_t = 2, long)]
        indent: u8,
    },
}

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
