use common::get;
use std::io::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequest {
    pub id: u32,
    pub html_url: String,
    pub state: String,
    pub title: String,
    pub number: u32,
    pub updated_at: Option<String>,
}

pub fn get_pull_requests(
    base_api: &str,
    owner: &str,
    repository: &str,
) -> Result<Vec<PullRequest>, Error> {
    let api_call = vec![base_api, "repos", owner, repository, "pulls"].join("/");
    get::<Vec<PullRequest>>(&api_call)
}
