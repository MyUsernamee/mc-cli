use clap::*;
use setup::*;

mod setup;

#[derive(Parser)]
#[command()]
struct Cli {

    dir: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,

}

#[derive(Subcommand)]
pub enum Commands {



}

fn main() {
    
    setup::setup().unwrap();

}
