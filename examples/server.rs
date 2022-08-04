use ci_insights::{ AppOptions, GenericResult};
use structopt::StructOpt;

fn main() -> GenericResult<()> {
    let _opts = AppOptions::from_args();

    Ok(())
}
