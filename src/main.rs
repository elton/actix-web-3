use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello)) // curl http://localhost:8088/hey
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
