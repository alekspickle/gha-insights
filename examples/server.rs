use ci_insights::{AppOptions, GenericResult};
use clap::Parser;

fn main() -> GenericResult<()> {
    let _opts = AppOptions::parse();

    // TODO: setup server

    Ok(())
}
