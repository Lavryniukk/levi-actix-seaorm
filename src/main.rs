use actix_web::{ App, HttpServer };
use db::{establish_connection, DatabaseContext};
use handlers::register;
use std::io;
use actix_web::web::Data;
use migration::{ Migrator, MigratorTrait };
mod entity;
mod handlers;
mod db;
mod services;
#[actix_web::main]
async fn main() -> Result<(), io::Error> {
    let connection = establish_connection().await.unwrap();

    Migrator::up(&connection, None).await.expect("Error running migrations");

    let context = DatabaseContext {
        db: connection.to_owned()
    };

    HttpServer::new(move || { App::new().app_data(Data::new(context.clone())).configure(register) })
        .bind(("127.0.0.1", 8080))?
        .run().await
}
