extern crate dotenv;

use actix_web::{
    post, 
    get, 
    put,
    delete,
    web,
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

#[derive(Serialize)]
struct User {
    user_id: u32,
    email: String,
    first_name: String,
    last_name: String,
    biography: String,
    avatar_link: String,
    followers: u32,
    following: u32
}

#[derive(Serialize)]
struct Event {
    event_id: u32,
    user_id: u32,
    username: String,
    location: String,
    description: String
}

#[post("")]
async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("Create user")
}

#[get("")]
async fn get_user(req: HttpRequest) -> impl Responder {
    let user_id: u32 = req.match_info().query("user_id").parse().unwrap();
    println!("user_id {}", user_id);

    HttpResponse::Ok().body("Get user")
}

#[put("")]
async fn update_user() -> impl Responder {
    HttpResponse::Ok().body("Update user")
}

#[delete("")]
async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("Delete User")
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
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/users")
                            .service(create_user)
                            .service(
                                web::scope("/{user_id}")
                                    .service(get_user)
                                    .service(update_user)
                                    .service(delete_user)
                            )
                        // web::scope("/events")
                    )
            )
    })
    .bind((host, port))?
    .run()
    .await
}
