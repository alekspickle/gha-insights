use ci_insights::{App, AppOptions, GenericResult, JobInfo};
use futures_utils::TryStreamExt;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> GenericResult<()> {
    let opts = AppOptions::from_args();
    let gh = opts.clone().octo().await?;

    let app = App::new(gh, opts);

    let runs = app.get_runs().await?;
    let jobs: Vec<JobInfo> = runs
        .into_iter()
        .map(|run| app.get_jobs(run.id))
        .try_collect();

    Ok(())
}
