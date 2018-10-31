extern crate reqwest;

use self::reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::io::Error;
use std::io::ErrorKind;

pub struct Authentication {
    pub token: String,
}

pub type MatterPullResult<T> = Result<T, Error>;
pub fn get<T>(url: &str) -> MatterPullResult<T>
where
    for<'a> T: Deserialize<'a>,
{
    reqwest::get(url)
        .and_then(|mut response| response.json::<T>())
        .map_err(|_| Error::new(ErrorKind::InvalidData, "Could not serialize"))
}

pub fn post<T>(url: &str, arguments: &HashMap<&str, &str>) -> MatterPullResult<T>
where
    for<'a> T: Deserialize<'a> + Debug,
{
    let client = reqwest::Client::new();
    client
        .post(url)
        .json(arguments)
        .send()
        .and_then(|mut send_result| send_result.json::<T>())
        .map_err(|_| Error::new(ErrorKind::InvalidData, "Could not serialize"))
}
