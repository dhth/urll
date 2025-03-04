use clap::Parser;

/// urll lets you browse URLs in a webpage in a recursive manner
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(value_name = "URL")]
    pub url: String,
    // whether to open up results in a TUI
    #[arg(short = 't', long = "tui")]
    pub tui: bool,
}
