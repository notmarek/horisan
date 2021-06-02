use crate::overdrive::GoogleDrive;

use crate::helpers::*;

use rocket::response::content;
use rocket::State;

use std::path::PathBuf;

use crate::structs::*;

#[get("/GoogleDrive/<path..>", rank = 0)]
pub async fn google_drive(
    path: Option<PathBuf>,
    config: State<'_, Config>,
) -> content::Json<String> {
    let new_path;
    let drive = config.drive.as_ref().unwrap();
    if path.as_ref().unwrap_or(&PathBuf::new()) == &PathBuf::new() {
        new_path = "root".to_string();
    } else {
        new_path = path
            .unwrap()
            .components()
            .last()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string();
    }
    let google_files = drive.get_files_in_folder(&new_path).await.files.unwrap();
    let mut files = parse_google_files(google_files, drive).await;
    files.sort_by(|a, b| file_sort(a, b));
    content::Json(vector_to_json_array(structs_to_json(files)))
}
