use actix_web::{body::BoxBody, http::header::ContentType, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Serialize;


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

// Response for a custom type that serializes into an `application/json` response
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
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(return_json)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
