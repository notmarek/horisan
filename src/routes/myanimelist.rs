use crate::helpers::*;

use rocket::response::content;
use rocket::response::Redirect;
use rocket::State;

use std::collections::HashMap;

use crate::structs::*;

#[post("/mal/list/update/anime", format = "json", data = "<data>")]
pub async fn malupdateanimelist(data: MALAnimeUpdate) -> content::Json<String> {
    let mut form = HashMap::new();
    form.insert("status", data.status);
    form.insert(
        "num_watched_episodes",
        data.num_watched_episodes.to_string(),
    );
    let client = reqwest::Client::new();
    let r = client
        .patch(&format!(
            "https://api.myanimelist.net/v2/anime/{}/my_list_status",
            data.anime_id
        ))
        .header("Authorization", format!("Bearer {}", data.token))
        .form(&form)
        .send()
        .await
        .unwrap();
    content::Json(r.text().await.unwrap())
}

#[post("/mal/oauth2", format = "json", data = "<auth>")]
pub async fn malauth(auth: MALReply, config: State<'_, Config>) -> content::Json<String> {
    let mut form = HashMap::new();
    form.insert("client_id", config.mal_client_id.as_ref().unwrap().as_str());
    form.insert(
        "client_secret",
        config.mal_secret.as_ref().unwrap().as_str(),
    );
    form.insert("code", &auth.code);
    form.insert("code_verifier", &auth.state);
    form.insert("grant_type", "authorization_code");
    let client = reqwest::Client::new();
    let r = client
        .post("https://myanimelist.net/v1/oauth2/token")
        .form(&form)
        .send()
        .await
        .unwrap();
    content::Json(r.text().await.unwrap())
}

#[post("/mal/user", format = "json", data = "<data>")]
pub async fn maluser(data: MALUser) -> content::Json<String> {
    let client = reqwest::Client::new();
    let r = client
        .get(format!(
            "https://api.myanimelist.net/v2/users/{}",
            data.user
        ))
        .header("Authorization", format!("Bearer {}", data.token))
        .send()
        .await
        .unwrap();
    content::Json(r.text().await.unwrap())
}

#[post("/mal/anime", format = "json", data = "<data>")]
pub async fn malanime(data: MALAnime) -> content::Json<String> {
    let client = reqwest::Client::new();
    let r = client
        .get(format!(
            "https://api.myanimelist.net/v2/anime/{}?fields=my_list_status,num_episodes",
            data.anime_id
        ))
        .header("Authorization", format!("Bearer {}", data.token))
        .send()
        .await
        .unwrap();
    content::Json(r.text().await.unwrap())
}

#[get("/mal/link")]
pub async fn malurl() -> Redirect {
    let code_verify = pkce::code_verifier(128);
    let code_challenge = pkce::code_challenge(&code_verify);
    Redirect::to(format!("https://myanimelist.net/v1/oauth2/authorize?response_type=code&client_id=0e16733a4d9bbf1152fa9cb2ada84048&code_challenge={}&state={}", code_challenge, code_challenge))
}

#[get("/map")]
pub async fn map() -> content::Json<String> {
    content::Json(get_anime_map())
}
