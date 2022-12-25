use ci_insights::{App, AppOptions, GenericResult, JobInfo};
use futures::TryStreamExt;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> GenericResult<()> {
    let opts = AppOptions::from_args();
    let gh = opts.clone().octo().await?;

    let app = App::new(gh, opts);

    let runs = app.get_runs().await?;
    let jobs: Vec<JobInfo> = futures::stream::iter(runs.into_iter().map(Result::Ok))
        .filter_map(move |run| {
            let id = run.id.clone();
            async move { app.get_jobs(run.id).await }
        })
        .try_collect()
        .await?;

    Ok(())
}
