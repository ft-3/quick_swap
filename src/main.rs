use actix_files::Files;
use actix_web::{error, middleware, get, Error, App, web, HttpRequest, HttpResponse, HttpServer, Responder};
use tera::Tera;
use env_logger;

#[get("/")]
async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", "Test123");
    let s = tmpl.render("index.html", &ctx).map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/swap/{swapid}")]
async fn swap(req: HttpRequest) -> impl Responder {
    let swapid = req.match_info().get("swapid").unwrap();
    HttpResponse::Ok().body(format!("Info about {}", swapid))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        let tera =
            Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        App::new()
            .data(tera)
            .wrap(middleware::Logger::default()) // enable logger
            .service(index)
            .service(swap)
            .service(Files::new("/static", "static"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
