use actix_web::{ get, post, web::{self,  Data}, HttpResponse, Responder };

use crate::{db::DatabaseContext,  services::{users_service::CreateUserInput, UsersService}};


#[get("/users")]
async fn hello(context: Data<DatabaseContext>) -> impl Responder {
    let users = UsersService::get_all_users(&context.db).await.unwrap();
    HttpResponse::Ok().json(users)
}

#[post("/users")]
async fn create_user(user: web::Json<CreateUserInput>, context: Data<DatabaseContext>) -> impl Responder {
    let created_user = UsersService::create_user(&context.db, user.into_inner()).await.unwrap();
    HttpResponse::Ok().json(created_user)
}

#[get("/users/{id}")]
async fn get_user_by_id(path: web::Path<(i32,)>, context: Data<DatabaseContext>) -> impl Responder {
    let user_id = path.0;
    let user = UsersService::get_user_by_id(&context.db, user_id).await.unwrap();
    HttpResponse::Ok().json(user)
}

pub fn register(config: &mut web::ServiceConfig) {
    config.service(hello).service(create_user).service(get_user_by_id);
}
