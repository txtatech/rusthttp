use actix_web::HttpServer;

use actix_files as files;

// server.rs
use actix_web::{get, post, web, App, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(r#"<html>
    <head>
        <meta http-equiv="refresh" content="5; URL='index.html'">
        <a href="http://127.0.0.1:3000/index.html">localhost:3000</a>
        <a href="http://127.0.0.1:8080/index.html">localhost:8080</a>
        <a href="http://127.0.0.1:8888/index.html">localhost:8888</a>
    </head>
    <body>
        Choose or be redirected to default host in 5 seconds.
    </body>
</html>"#)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub struct Server {
    // struct fields and methods
}

impl Server {
    pub fn new() -> Server {
        Server { /* initialize struct fields */ }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .default_service(files::Files::new("/", "static").index_file("index.html"))
                .service(hello)
                .route("/hey", web::get().to(manual_hello))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    }
}
