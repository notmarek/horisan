use crate::overdrive::{Drive, GoogleDrive};
use crate::structs::*;
use anitomy::{Anitomy, ElementCategory};
use google_drive3;
use serde::Serialize;
use std::cmp::Ordering;
use std::fs;
use std::lazy::SyncLazy;

static ANIME: SyncLazy<Vec<AnimeInfo>> = SyncLazy::new(|| {
    let contents = fs::read_to_string("map.json");
    let mal_info: Vec<AnimeInfo> = serde_json::from_str(&contents.unwrap()).unwrap();
    mal_info
});

pub fn get_anime_map() -> String {
    vector_to_json_array(structs_to_json(ANIME.get(..).unwrap().to_vec()))
}

pub fn structs_to_json<T>(data: Vec<T>) -> Vec<String>
where
    T: Serialize,
{
    let mut items: Vec<String> = Vec::new();
    for item in data {
        items.push(serde_json::to_string(&item).unwrap());
    }
    items
}

pub async fn parse_google_file(file: google_drive3::api::File, drive: &Drive) -> ParsedFile {
    let anime_info: Vec<AnimeInfo> = ANIME.get(..).unwrap().to_vec();
    let parsed_file: ParsedFile;
    let url;
    let file_type = match file
        .mime_type
        .as_ref()
        .unwrap_or(&"file".to_string())
        .contains("folder")
    {
        true => {
            url = file.id.clone().unwrap();
            "directory".to_string()
        }

        _ => {
            url = drive.get_direct_link(&file).await;
            "file".to_string()
        }
    };

    if file_type == "file"
        && !(file.name.as_ref().unwrap().contains(".mkv")
            || file.name.as_ref().unwrap().contains(".mp4"))
    {
        parsed_file = ParsedFile {
            name: Some(url),
            anime: file.name,
            group: Some(String::new()),
            episode: Some(String::new()),
            r#type: Some(file_type),
            mtime: None,
            size: None,
            mal_id: Some(0),
        };
    } else {
        let mut anitomy: Anitomy = Anitomy::new();
        match anitomy.parse(file.name.as_ref().unwrap()) {
            Ok(ref e) | Err(ref e) => {
                let mal = &anime_info
                    .into_iter()
                    .filter(|ye| {
                        ye.name.as_ref().unwrap()
                            == &e.get(ElementCategory::AnimeTitle).unwrap_or("").to_string()
                    })
                    .collect::<Vec<AnimeInfo>>();
                parsed_file = ParsedFile {
                    name: Some(url),

                    anime: Some(e.get(ElementCategory::AnimeTitle).unwrap_or("").to_string()),
                    group: Some(
                        e.get(ElementCategory::ReleaseGroup)
                            .unwrap_or("")
                            .to_string(),
                    ),
                    episode: Some(
                        e.get(ElementCategory::EpisodeNumber)
                            .unwrap_or("")
                            .to_string(),
                    ),
                    r#type: Some(file_type),
                    mtime: None,
                    size: None,
                    mal_id: {
                        if mal.len() < 1 {
                            Some(0)
                        } else {
                            mal[0].mal
                        }
                    },
                }
            }
        }
    }
    parsed_file
}

pub fn parse_file(file: File) -> ParsedFile {
    let anime_info: Vec<AnimeInfo> = ANIME.get(..).unwrap().to_vec();
    let parsed_file: ParsedFile;

    if file.r#type.as_ref().unwrap() == "file"
        && !(file.name.as_ref().unwrap().contains(".mkv")
            || file.name.as_ref().unwrap().contains(".mp4"))
    {
        parsed_file = ParsedFile {
            name: file.name.clone(),
            anime: file.name,
            group: Some(String::new()),
            episode: Some(String::new()),
            r#type: file.r#type,
            mtime: file.mtime,
            size: file.size,
            mal_id: Some(0),
        };
    } else {
        let mut anitomy: Anitomy = Anitomy::new();
        match anitomy.parse(file.name.as_ref().unwrap()) {
            Ok(ref e) | Err(ref e) => {
                let mal = &anime_info
                    .into_iter()
                    .filter(|ye| {
                        ye.name.as_ref().unwrap()
                            == &e.get(ElementCategory::AnimeTitle).unwrap_or("").to_string()
                    })
                    .collect::<Vec<AnimeInfo>>();
                parsed_file = ParsedFile {
                    name: file.name,
                    anime: Some(e.get(ElementCategory::AnimeTitle).unwrap_or("").to_string()),
                    group: Some(
                        e.get(ElementCategory::ReleaseGroup)
                            .unwrap_or("")
                            .to_string(),
                    ),
                    episode: Some(
                        e.get(ElementCategory::EpisodeNumber)
                            .unwrap_or("")
                            .to_string(),
                    ),
                    r#type: file.r#type,
                    mtime: file.mtime,
                    size: file.size,
                    mal_id: {
                        if mal.len() < 1 {
                            Some(0)
                        } else {
                            mal[0].mal
                        }
                    },
                }
            }
        }
    }
    parsed_file
}

pub fn parse_files(files: Vec<File>) -> Vec<ParsedFile> {
    let mut parsed_files: Vec<ParsedFile> = Vec::new();
    for file in files {
        parsed_files.push(parse_file(file))
    }
    parsed_files
}

pub async fn parse_google_files(
    files: Vec<google_drive3::api::File>,
    drive: &Drive,
) -> Vec<ParsedFile> {
    let mut parsed_files: Vec<ParsedFile> = Vec::new();
    for file in files {
        parsed_files.push(parse_google_file(file, &drive).await)
    }
    parsed_files
}

pub fn vector_to_json_array(vec: Vec<String>) -> String {
    format!("[{}]", vec.join(", "))
}

pub fn file_sort(a: &ParsedFile, b: &ParsedFile) -> Ordering {
    if a.r#type.as_ref().unwrap() == &"file".to_string()
        && b.r#type.as_ref().unwrap() == &"file".to_string()
        && a.anime.as_ref().unwrap_or(&"~~~".to_string())
            == b.anime.as_ref().unwrap_or(&"~~~".to_string())
    {
        a.episode
            .as_ref()
            .unwrap_or(&"~~~".to_string())
            .to_lowercase()
            .cmp(
                &b.episode
                    .as_ref()
                    .unwrap_or(&"~~~".to_string())
                    .to_lowercase(),
            )
    } else {
        a.anime
            .as_ref()
            .unwrap_or(&"~~~".to_string())
            .to_lowercase()
            .cmp(
                &b.anime
                    .as_ref()
                    .unwrap_or(&"~~~".to_string())
                    .to_lowercase(),
            )
    }
}
