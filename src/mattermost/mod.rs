use chrono::{TimeZone, Utc};
use common::post;
use github::PullRequest;
use std::collections::HashMap;
use std::io::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MattermostResponse {
    pub id: String,
}

pub trait Mattermost<T> {
    fn format_for_mattermost(items_to_post: Vec<T>) -> String;
}

impl Mattermost<PullRequest> for PullRequest {
    fn format_for_mattermost(items_to_post: Vec<PullRequest>) -> String {
        let state = "Low Activity Pull Requests \n";
        let format = items_to_post
            .into_iter()
            .map(|v| vec![v.clone().html_url, map_exclamation_marks(&v)].join(" "))
            .collect::<Vec<_>>()
            .join("\n");
        vec![state, &*format].join("")
    }
}

fn map_exclamation_marks(pull_request: &PullRequest) -> String {
    pull_request
        .clone()
        .updated_at
        .map(|update_duration| {
            let datetime = Utc
                .datetime_from_str(&*update_duration, "%Y-%m-%dT%H:%M:%SZ")
                .unwrap();
            let duration_elapsed = Utc::now() - datetime;
            (0..(duration_elapsed.num_days() / 2))
                .collect::<Vec<i64>>()
                .iter()
                .map(|_| "!")
                .collect::<String>()
        })
        .unwrap_or("".to_string())
}

pub fn post_to_mattermost(
    web_hook: &str,
    to_mattermost: &str,
) -> Result<MattermostResponse, Error> {
    let mut payload_map = HashMap::new();
    payload_map.insert("channel", "devcore");
    payload_map.insert("username", "Github Doctor");
    payload_map.insert(
        "icon_url",
        "https://png.icons8.com/metro/1600/medical-doctor.png",
    );
    payload_map.insert("text", to_mattermost);
    post::<MattermostResponse>(web_hook, &payload_map)
}
