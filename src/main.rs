use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("¡Hola desde Rust con Actix-web!")
}

#[get("/test")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("¡Test!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Iniciando servidor en http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(test)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}