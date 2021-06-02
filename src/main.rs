#![feature(proc_macro_hygiene, decl_macro, once_cell)]
#[macro_use]
extern crate rocket;

mod helpers;
mod overdrive;
mod structs;
mod routes;

use overdrive::{Drive, GoogleDrive};

use rocket::Rocket;
use routes::myanimelist::*;
use routes::core::*;
use routes::googledrive::*;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

use std::env;
use std::path::PathBuf;

use structs::*;

pub struct CORS();
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        res.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[options("/<_path..>")]
async fn options(_path: Option<PathBuf>) -> String {
    // just a catchall endpoint to add correct CORS to all option requests
    String::new()
}

#[launch]
async fn rocket() -> Rocket {
    dotenv::dotenv().expect("Failed to load .env file");

    let mut routes = routes![options, files, files_y];

    let mut config: Config = Config { drive: None, mal_secret: None, mal_client_id: None };

    let drive_enabled: String = env::var("ENABLE_GDRIVE").expect("ENABLE_GDRIVE expected");
    if drive_enabled.to_lowercase() == "true" || drive_enabled.to_lowercase() == "yes" {
        let drive_api_key: String = env::var("GDRIVE_API_KEY").expect("GDRIVE_API_KEY not found.");
        let drive_secret_file: String = env::var("GDRIVE_APP_SECRET").expect("GDRIVE_APP_SECRET not found.");
        let drive: Drive = Drive::init(&drive_secret_file, &drive_api_key, "drive").await;
        config.drive = Some(drive);
        routes.extend(routes![google_drive]);
    }

    let mal_enabled: String = env::var("ENABLE_MAL").expect("ENABLE_MAL expected");
    if mal_enabled.to_lowercase() == "true" || mal_enabled.to_lowercase() == "yes" {
        let mal_secret: String = env::var("MAL_SECRET").expect("MAL_SECRET not found.");
        let mal_client_id: String = env::var("MAL_CLIENT_ID").expect("MAL_CLIENT_ID not found.");
        config.mal_client_id = Some(mal_client_id);
        config.mal_secret = Some(mal_secret);
        routes.extend(routes![malauth, malurl, maluser, malanime, malupdateanimelist, map]);
    }
    
    let base_path: String = env::var("BASE_PATH").unwrap_or("/".to_string());
    rocket::ignite()
        .manage(config)
        .mount(
            &base_path,
            routes,
        )
        .attach(CORS())
}
