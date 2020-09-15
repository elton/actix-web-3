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
use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Mutex;

struct AppState {
    app_name: String,
}

async fn state(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name; // get app name
    format!("Hello {}!", app_name)
}

pub struct AppStateWithCounter {
    pub counter: Mutex<i32>, // <- 互斥锁对于跨线程安全地进行变异是必要的
}

#[get("/appstate")]
pub async fn app_state(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap(); // <- 获取counter的互斥锁
    *counter += 1; // <- MutexGuard内的访问计数器

    format!("Request number: {}", counter)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    // curl https://localhost:8443/application/apps
    cfg.service(
        web::resource("/apps")
            .route(web::get().to(|| HttpResponse::Ok().body("apps")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
    // curl https://localhost:8443/application/test
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok().body("test")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
    // Application state is shared with all routes and resources within the same scope
    // curl https://localhost:8443/application/state
    cfg.service(
        web::resource("/state")
            .data(AppState {
                // 每个线程建立的state是独立的
                app_name: String::from("Actix-web"),
            })
            .route(web::get().to(state)),
    );
}
