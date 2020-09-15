use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::sync::Mutex;

mod application;

// curl https://localhost:8443/
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world.")
}

// curl -d 'hello' -X POST https://localhost:8443/echo
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
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    println!("Started http server: 127.0.0.1:8443");
    // 使用mkcert创建本地证书（https://github.com/FiloSottile/mkcert）
    // brew install mkcert
    // mkcert localhost 127.0.0.1 ::1
    // load ssl keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let counter = web::Data::new(application::AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        // 把counter变量move到闭包里，避免悬空引用
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello)) // curl https://localhost:8443/hey
            // curl https://localhost:8443/app/index.html scope添加统一的前缀
            .service(web::scope("app").route("/index.html", web::get().to(index)))
            // Application state is shared with all routes and resources within the same scope.
            .data(application::AppState {
                // 每个线程建立的state是独立的
                app_name: String::from("Actix-web"),
            })
            .service(application::state) // curl https://localhost:8443/state
            // Note: using app_data instead of data,跨线程间的数据同步
            .app_data(counter.clone())
            .service(application::app_state)
            // 通过配置，把资源定义放在其他模块中
            // curl https://localhost:8443/apps
            .configure(application::config)
            // curl https://localhost:8443/api/test
            .service(web::scope("/api").configure(application::scope_config))
    })
    .bind_openssl("127.0.0.1:8443", builder)?
    .run()
    .await
}
