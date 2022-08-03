use octocrab::models::{
    workflows::{Job, Run, Step},
    JobId, RunId,
};

/// Struct aggregating useful job info
#[derive(Debug)]
pub struct RunInfo {
    pub id: RunId,
    pub name: String,
    pub head_branch: String,
    pub status: String,
    pub jobs_url: String,
    pub conclusion: Option<String>,
}

impl From<Run> for RunInfo {
    fn from(run: Run) -> Self {
        let Run {
            id,
            name,
            status,
            head_branch,
            conclusion,
            ..
        } = run;
        Self {
            id,
            name,
            status,
            conclusion,
            head_branch,
            jobs_url: run.jobs_url.to_string(),
        }
    }
}

/// Struct aggregating useful job info
#[derive(Debug)]
pub struct JobInfo {
    pub id: JobId,
    pub run_url: String,
    pub steps: Vec<StepInfo>,
    pub conclusion: Option<String>,
}

impl From<Job> for JobInfo {
    fn from(job: Job) -> Self {
        Self {
            id: job.id,
            run_url: job.run_url.to_string(),
            steps: job.steps.into_iter().map(Into::into).collect(),
            conclusion: job.conclusion,
        }
    }
}

#[derive(Debug)]
pub struct StepInfo {
    pub name: String,
    pub status: String,
    pub conclusion: Option<String>,
}

impl From<Step> for StepInfo {
    fn from(s: Step) -> Self {
        Self {
            name: s.name,
            status: s.status,
            conclusion: s.conclusion,
        }
    }
}
