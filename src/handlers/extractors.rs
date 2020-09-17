use actix_web::{get, post, web, HttpRequest, Result};
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

#[derive(Deserialize)]
struct FormData {
    username: String,
}

/// extract form data using serde 通过serde来提取提交的表单中的内容
/// this handler gets called only if the content type is *x-www-form-urlencoded*
/// and the content of the request could be deserialized to a `FormData` struct

#[post("/user")]
async fn form(form: web::Form<FormData>) -> Result<String> {
    Ok(format!("Welcome {}!", form.username))
}
