extern crate dotenv;

use actix_web::{
    body::BoxBody, 
    http::header::ContentType, 
    get, 
    post, 
    App, 
    HttpRequest, 
    HttpResponse, 
    HttpServer, 
    Responder
};
use dotenv::dotenv;
use serde::Serialize;
use std::{
    env
};


// Routes
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/json")]
async fn return_json() -> impl Responder {
    MyObj { name: "user" }
}

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

impl Responder for MyObj {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let host_key: String = String::from("HOST");
    let host: String = match env::var_os(host_key.clone()) {
        Some(v) => v.into_string().unwrap(),
        None => { 
            println!("{} is not defined in .env file.", host_key.clone());
            String::from("127.0.0.1")
        }
    };

    let port_key: String = String::from("PORT");
    let port: u16 = String::from(
        match env::var_os(port_key.clone()) {
            Some(v) => v.into_string().unwrap(),
            None => {
                println!("{} is not defined in .env file.", port_key.clone());
                String::from("8080")
            }
        }
    )
    .parse().unwrap();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(return_json)
    })
    .bind((host, port))?
    .run()
    .await
}
