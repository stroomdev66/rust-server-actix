// use actix_web::{get, post, middleware, web, App, HttpResponse, HttpServer, Responder};
use actix_web::{post, App, HttpResponse, HttpServer, Responder};

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

#[post("/stroom/noauth/datafeed")]
async fn echo() -> impl Responder {
    HttpResponse::Ok()
}

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok))).workers(4);

    HttpServer::new(|| {
        App::new()
            // .wrap(middleware::DefaultHeaders::new().add("X-Version", "0.2"))
            // .wrap(middleware::Compress::default())
            // .wrap(middleware::Logger::default())
            // .service(hello)
            .service(echo)
            // .route("/hey", web::get().to(manual_hello))
    })
        // .workers(24)
        // .max_connections(1024)
        // .workers(1)
        // .backlog(1024)
        // .max_connection_rate(1024)
        // .worker_max_blocking_threads(2048)
        .bind(("127.0.0.1", 8090))?
        .run()
        .await
}