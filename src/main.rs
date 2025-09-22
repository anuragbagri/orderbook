use actix_web::{
    delete, get, post, web::{self, post}, App, HttpServer, Responder
};

#[actix_web::main]

async  fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service()
        .service()
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}


#[get("/depth")]


#[post("/order")]


#[delete("/delete")]



