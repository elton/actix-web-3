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
use actix_web::{web, HttpResponse, Responder};

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world.")
}

async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn index() -> impl Responder {
    "Hello World from index page."
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    // curl https://localhost:8443/hello/
    cfg.service(web::resource("/").route(web::get().to(hello)));
    // curl -d 'hello' -X POST https://localhost:8443/hello/echo
    cfg.service(web::resource("/echo").route(web::post().to(echo)));
    // curl https://localhost:8443/hello/index.html
    cfg.service(web::resource("/index.html").route(web::get().to(index)));
}
