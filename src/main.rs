use actix_web::{middleware, web, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::sync::Mutex;

mod application;
mod handlers;
mod hello;

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
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::scope("hello").configure(hello::routes))
            // Note: using app_data instead of data,跨线程间的数据同步
            // curl https://localhost:8443/appstate
            // 跨线程的数据不能放到模块里
            .app_data(counter.clone())
            .service(application::app_state)
            .service(web::scope("/application").configure(application::routes))
            .service(web::scope("/handlers").configure(handlers::routes))
            .service(web::scope("/extractors").configure(handlers::routes))
    })
    .bind_openssl("127.0.0.1:8443", builder)?
    .run()
    .await
}
