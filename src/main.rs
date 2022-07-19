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
    println!("GET user_id {}", user_id);

    HttpResponse::Ok().body("Get user")
}

#[put("")]
async fn update_user(req: HttpRequest) -> impl Responder {
    let user_id: u32 = req.match_info().query("user_id").parse().unwrap();
    println!("UPDATE user_id {}", user_id);

    HttpResponse::Ok().body("Update user")
}

#[delete("")]
async fn delete_user(req: HttpRequest) -> impl Responder {
    let user_id: u32 = req.match_info().query("user_id").parse().unwrap();
    println!("DELETE user_id {}", user_id);

    HttpResponse::Ok().body("Delete user")
}

#[post("")]
async fn create_event() -> impl Responder {
    HttpResponse::Ok().body("Create user")
}

#[get("")]
async fn get_event(req: HttpRequest) -> impl Responder {
    let event_id: u32 = req.match_info().query("event_id").parse().unwrap();
    println!("GET event_id {}", event_id);

    HttpResponse::Ok().body("Get event")
}

#[put("")]
async fn update_event(req: HttpRequest) -> impl Responder {
    let event_id: u32 = req.match_info().query("event_id").parse().unwrap();
    println!("UPDATE event_id {}", event_id);

    HttpResponse::Ok().body("Update event")
}

#[delete("")]
async fn delete_event(req: HttpRequest) -> impl Responder {
    let event_id: u32 = req.match_info().query("event_id").parse().unwrap();
    println!("DELETE event_id {}", event_id);

    HttpResponse::Ok().body("Delete event")
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
                    )
                    .service(
                        web::scope("/events")
                            .service(create_event)
                            .service(
                                web::scope("/{event_id}")
                                    .service(get_event)
                                    .service(update_event)
                                    .service(delete_event)
                            )
                    )
            )
    })
    .bind((host, port))?
    .run()
    .await
}
