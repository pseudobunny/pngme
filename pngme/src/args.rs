use clap::Parser;
use crate::commands::Action;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    action: Action
}