use ci_insights::{App, AppOptions, GenericResult, JobInfo};
use clap::Parser;
use futures_util::{stream, StreamExt};

#[tokio::main]
async fn main() -> GenericResult<()> {
    let opts = AppOptions::parse();
    let gh = opts.clone().octo().await?;
    let app = App::new(gh, opts);

    let runs = app.get_runs().await?;

    // Goddammit lifetimes and borrowing does not make this stuff easy...
    let mut jobs = stream::iter(runs.into_iter())
        .map(move |run| {
            let app = app.clone();
            async move { app.get_jobs(run.id.clone()).await }
        })
        .buffer_unordered(10);

    let mut result = Vec::new();
    while let Some(res) = jobs.next().await {
        match res {
            Ok(s) => result.push(s),
            Err(e) => eprintln!("Error processing line: {}", e),
        }
    }
    let ids: Vec<String> = result.iter().flatten().map(|j| j.id.to_string()).collect();

    println!("Ids found: {:?}", ids);

    Ok(())
}
