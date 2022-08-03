use gh_rs::{App, AppOptions, GenericResult};
use structopt::StructOpt;

fn main() -> GenericResult<()> {
    let app = AppOptions::from_args();

    Ok(())
}
