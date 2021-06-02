use crate::overdrive::Drive;

use rocket::data::{Data, FromData, Limits, Outcome};
use rocket::outcome::Outcome::Success;
use rocket::Request;

use serde::{Deserialize, Serialize};

pub struct Config {
    pub mal_secret: Option<String>,
    pub drive: Option<Drive>,
    pub mal_client_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnimeInfo {
    pub name: Option<String>,
    pub mal: Option<u32>,
    pub episode_offset: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub r#type: Option<String>,
    pub name: Option<String>,
    pub mtime: Option<String>,
    pub size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedFile {
    pub name: Option<String>,
    pub anime: Option<String>,
    pub group: Option<String>,
    pub episode: Option<String>,
    pub r#type: Option<String>,
    pub mtime: Option<String>,
    pub size: Option<u64>,
    pub mal_id: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MALReply {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MALUser {
    pub user: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MALAnime {
    pub anime_id: u32,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MALAnimeUpdate {
    pub anime_id: u32,
    pub token: String,
    pub status: String,
    pub num_watched_episodes: u32,
}

macro_rules! from_json {
    ($($t:ty),+) => {
        $(#[rocket::async_trait]
            impl<'r> FromData<'r> for $t {
                type Error = String;
                async fn from_data(req: &'r Request<'_>, data: Data) -> Outcome<Self, Self::Error> {
                    let limit = req.limits().get("json").unwrap_or(Limits::STRING);
                    let result: $t =
                        serde_json::from_str(&data.open(limit).into_string().await.unwrap()).unwrap();
                    Success(result)
                }
            })+
    }
}
from_json!(MALReply, MALUser, MALAnime, MALAnimeUpdate);
