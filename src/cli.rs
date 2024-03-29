use crate::{
    defs::{JobInfo, RunInfo},
    error::Error,
    Result,
};
use clap::{command, Parser};
use octocrab::{
    models::{InstallationToken, RunId},
    params::apps::CreateInstallationAccessToken,
    Octocrab,
};

const NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, name = NAME)]
pub struct AppOptions {
    /// Choose a name for your app
    #[arg(long, short, env = "NAME", default_value = NAME)]
    pub name: String,

    // Github token authorized to do what you want to do.
    #[arg(long, env)]
    pub token: Option<String>,

    // User name, usually it's your GitHub handle
    #[arg(long, short, env, default_value = NAME)]
    pub gh_user: String,

    // Repository you want to analyze
    #[arg(long, short, env, default_value = NAME)]
    pub repo_name: String,

    /// App id. Override for more than one instance usage with JWT tokens
    #[arg(long, env, conflicts_with = "token")]
    pub app_id: Option<u64>,

    /// App private key. Override for more than one instance usage with JWT tokens
    #[arg(long, env, conflicts_with = "token")]
    pub app_private_key: Option<String>,
}

impl AppOptions {
    /// Create an instance of octocrab service with JWT app authentication
    pub async fn octo(&self) -> Result<Octocrab> {
        if let Some(token) = &self.token {
            return octocrab::Octocrab::builder()
                .personal_token(token.clone())
                .build()
                .map_err(Error::Octocrab);
        }

        // Generate private app key for this purpose:
        // https://docs.github.com/en/developers/apps/building-github-apps/authenticating-with-github-apps
        let pem_bytes = self
            .app_private_key
            .clone()
            .map(|s| s.as_bytes().to_owned())
            .expect("invalid RSA key");
        let key = jsonwebtoken::EncodingKey::from_rsa_pem(&pem_bytes).map_err(Error::JWT)?;
        let token = octocrab::auth::create_jwt(self.app_id.unwrap().into(), &key)?;

        let gh = octocrab::Octocrab::builder()
            .personal_token(token)
            .build()
            .map_err(Error::Octocrab)?;

        let installations = gh
            .apps()
            .installations()
            .send()
            .await
            .map_err(Error::Octocrab)?
            .take_items();

        let mut create_access_token = CreateInstallationAccessToken::default();
        create_access_token.repositories = vec![self.repo_name.clone()];

        let access: InstallationToken = gh
            .post(
                installations[0].access_tokens_url.as_ref().unwrap(),
                Some(&create_access_token),
            )
            .await
            .unwrap();

        octocrab::OctocrabBuilder::new()
            .personal_token(access.token)
            .build()
            .map_err(Error::Octocrab)
    }
}

#[derive(Debug, Clone)]
pub struct App {
    pub gh: Octocrab,
    pub opts: AppOptions,
    #[cfg(feature = "server")]
    pub rocket: Option<Rocket<Build>>,
}

impl App {
    pub fn new(gh: Octocrab, opts: AppOptions) -> Self {
        Self { gh, opts }
    }

    #[cfg(feature = "server")]
    pub fn rocket(&self) -> Rocket<Build> {
        self.rocket.clone().unwrap()
    }

    pub async fn get_jobs(&self, id: RunId) -> Result<Vec<JobInfo>> {
        let wf = self.gh.workflows(&self.opts.gh_user, &self.opts.repo_name);
        let mut page = wf
            .list_jobs(id)
            .per_page(50)
            .page(1u32)
            .send()
            .await
            .unwrap();
        let jobs: Vec<JobInfo> = page.take_items().into_iter().map(JobInfo::from).collect();

        Ok(jobs)
    }

    pub async fn get_runs(&self) -> Result<Vec<RunInfo>> {
        let wf = self.gh.workflows(&self.opts.gh_user, &self.opts.repo_name);
        let mut page = wf.list_all_runs().per_page(50).page(1u32).send().await?;
        let runs: Vec<RunInfo> = page.take_items().into_iter().map(RunInfo::from).collect();

        Ok(runs)
    }
}
