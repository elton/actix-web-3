use actix_web::{get, web, HttpRequest, Result};
use serde::Deserialize;

/// extract path info from "/users/{userid}/{friend}" url
/// {userid} -  - deserializes to a u32
/// {friend} - deserializes to a String

#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn user_info(web::Path((user_id, friend)): web::Path<(u32, String)>) -> Result<String> {
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

/// extract path info using serde
#[get("/users2/{user_id}/{friend}")] // <- define path parameters
async fn user_info_serde(info: web::Path<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}!",
        info.friend, info.user_id
    ))
}

/// It is also possible to get or query the request for path parameters by name
#[get("/users3/{userid}/{friend}")] // <- define path parameters
async fn user_info_request(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().query("userid").parse().unwrap();

    Ok(format!("Welcome {}, userid {}!", name, userid))
}
