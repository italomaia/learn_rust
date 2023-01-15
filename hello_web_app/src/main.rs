use actix_web::{web, App, HttpResponse, HttpServer, Responder};


async fn index_view() -> impl Responder {
    HttpResponse::Ok().body("<h1>This is the index page</h1>")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(||{
                let mut resp =
                        // redirect and preserve the http method used; also, no caching please
                        HttpResponse::TemporaryRedirect();
                    resp.insert_header(("Location", "/root_path/"));
                    resp
            }))
            .service(
                web::scope("/root_path")
                    .route("/", web::get().to(index_view))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// go here http://127.0.0.1:8080/root_path/index to see our page
