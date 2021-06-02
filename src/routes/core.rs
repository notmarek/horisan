use crate::helpers::*;

use rocket::response::content;

use std::fs;
use std::path::PathBuf;

use crate::structs::*;

use chrono::offset::Utc;
use chrono::DateTime;

#[get("/")]
pub async fn files() -> content::Json<String> {
    let mut files: Vec<File> = Vec::new();
    let paths: fs::ReadDir = fs::read_dir("/home/pi/Y/Animu/").unwrap();
    for path in paths {
        let metadata = path.as_ref().unwrap().metadata().unwrap();
        let modified: DateTime<Utc> = metadata.modified().unwrap().into();
        let file: File = File {
            name: Some(format!(
                "{}",
                path.as_ref().unwrap().file_name().into_string().unwrap()
            )),
            r#type: match metadata.is_dir() {
                true => Some("directory".to_string()),
                false => Some("file".to_string()),
            },
            mtime: Some(modified.format("%a, %d %b %Y %T %Z").to_string()),
            size: Some(metadata.len()),
        };
        files.push(file);
    }
    let mut parsed_files: Vec<ParsedFile> = parse_files(files);
    parsed_files.sort_by(|a, b| file_sort(a, b));
    content::Json(vector_to_json_array(structs_to_json(parsed_files)))
}

#[get("/<path..>", rank = 4)]
pub async fn files_y(path: Option<PathBuf>) -> content::Json<String> {
    let mut files: Vec<File> = Vec::new();
    let paths: fs::ReadDir = fs::read_dir(format!(
        "/home/pi/Y/Animu/{}",
        path.unwrap_or(PathBuf::new()).to_str().unwrap_or("")
    ))
    .unwrap();
    for path in paths {
        let metadata = path.as_ref().unwrap().metadata().unwrap();
        let modified: DateTime<Utc> = metadata.modified().unwrap().into();
        let file: File = File {
            name: Some(format!(
                "{}",
                path.as_ref().unwrap().file_name().into_string().unwrap()
            )),
            r#type: match metadata.is_dir() {
                true => Some("directory".to_string()),
                false => Some("file".to_string()),
            },
            mtime: Some(modified.format("%a, %d %b %Y %T %Z").to_string()),
            size: Some(metadata.len()),
        };
        files.push(file);
    }
    let mut parsed_files: Vec<ParsedFile> = parse_files(files);
    parsed_files.sort_by(|a, b| file_sort(a, b));
    content::Json(vector_to_json_array(structs_to_json(parsed_files)))
}
