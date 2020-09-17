// Copyright 2020 Elton Zheng
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[get("/str")]
pub async fn responder_str() -> &'static str {
    "Responder &'static str"
}

#[get("/string")]
pub async fn responder_string() -> String {
    "Responder_string".to_owned()
}

#[get("/impl_responder")]
pub async fn responder_impl_responder() -> impl Responder {
    web::Bytes::from_static(b"responder_impl_responder")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyObj {
    name: String,
    number: i32,
}

// 自定义 Response
#[derive(Serialize)]
pub struct ResponseWrapper<T> {
    code: i32,
    msg: String,
    data: Option<T>,
}

// 实现 Responder trait
impl<T> Responder for ResponseWrapper<T>
where
    T: Serialize,
{
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

#[get("/impl_responder")]
pub async fn responder_custom_responder() -> impl Responder {
    ResponseWrapper {
        code: 0,
        msg: "success".to_string(),
        data: Some("custom_responder".to_string()),
    }
}

/// This handler uses json extractor
#[post("/extractor")]
pub async fn extractor(item: web::Json<MyObj>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}

/// This handler uses json extractor with limit
#[post("/extractor2")]
pub async fn extract_item(item: web::Json<MyObj>, req: HttpRequest) -> HttpResponse {
    println!("request: {:?}", req); //捕获HttpRequset对象内容
    println!("model: {:?}", item);

    HttpResponse::Ok().json(item.0) // <- send json response
}
