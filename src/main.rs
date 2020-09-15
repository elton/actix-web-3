use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

mod application;

// curl http://localhost:8088/
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world.")
}

// curl -d 'hello' -X POST http://localhost:8088/echo
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey, there.")
}

// Application
async fn index() -> impl Responder {
    "Hello World from the scope `app`."
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(application::AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        // 把counter变量move到闭包里，避免悬空引用
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello)) // curl http://localhost:8088/hey
            // curl http://localhost:8088/app/index.html scope添加统一的前缀
            .service(web::scope("app").route("/index.html", web::get().to(index)))
            // Application state is shared with all routes and resources within the same scope.
            .data(application::AppState {
                // 每个线程建立的state是独立的
                app_name: String::from("Actix-web"),
            })
            .service(application::state) // curl http://localhost:8088/state
            // Note: using app_data instead of data,跨线程间的数据同步
            .app_data(counter.clone())
            .service(application::app_state)
            // 通过配置，把资源定义放在其他模块中
            // curl http://localhost:8088/apps
            .configure(application::config)
            // curl http://localhost:8088/api/test
            .service(web::scope("/api").configure(application::scope_config))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
