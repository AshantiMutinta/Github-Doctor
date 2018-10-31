#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate serde;
extern crate serde_json;

pub mod common;
pub mod github;
pub mod mattermost;
use chrono::{TimeZone, Utc};
use github::{get_pull_requests, PullRequest};
use mattermost::{post_to_mattermost, Mattermost};
use std::env;
use std::io::Error;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let hook = args.get(1).expect("webhook not provided");
    let pull_requests = get_pull_requests("https://api.github.com", "holochain", "holochain-rust")
        .unwrap_or(Vec::new())
        .into_iter()
        .filter(|e| e.state == "open")
        .filter(|f| evaluate_low_activity(f))
        .collect::<Vec<_>>();
    post_to_mattermost(hook, &*PullRequest::format_for_mattermost(pull_requests));
}

fn evaluate_low_activity(pull_request: &PullRequest) -> bool {
    pull_request
        .clone()
        .updated_at
        .map(|update_duration| {
            Utc.datetime_from_str(&*update_duration, "%Y-%m-%dT%H:%M:%SZ")
                .map(|datetime| {
                    let duration_elapsed = Utc::now() - datetime;
                    duration_elapsed.num_days() > 1
                })
                .unwrap_or(false)
        })
        .unwrap_or(false)
}
