use chrono::Utc;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{ ColumnTrait, EntityTrait, QueryFilter, Set, TryIntoModel };
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use crate::entity::user::{self, Model as User};

use crate::entity::user::Entity as UserEntity;
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct UsersService {}

impl UsersService {
    pub async fn create_user(
        connection: &DbConn,
        create_user_dto: CreateUserInput
    ) -> Result<User, DbErr> {
        let existing_user = UserEntity::find()
            .filter(user::Column::Email.contains(&create_user_dto.email))
            .one(connection).await
            .expect("Error finding user");

        if let Some(_) = existing_user {
            return Err(DbErr::Custom("User already exists".to_string()));
        }

        let active_user = user::ActiveModel {
            id: NotSet,
            name: Set(create_user_dto.name),
            email: Set(create_user_dto.email),
            role: Set("user".to_string()),
            password: Set(create_user_dto.password),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
            last_signed_in_at: Set(Utc::now().naive_utc()),
        };

        active_user.save(connection).await.unwrap().try_into_model()
    }
    pub async fn get_user_by_id(connection: &DbConn, id: i32) -> Result<Option<User>, DbErr> {
        UserEntity::find_by_id(id).one(connection).await
    }

    pub async fn get_all_users(connection: &DbConn) -> Result<Vec<User>, DbErr> {
        UserEntity::find().all(connection).await
    }

    pub async fn _update_user(
        connection: &DbConn,
        id: i32,
        name: String,
        password: String
    ) -> Result<User, DbErr> {
        let user = UserEntity::find_by_id(id).one(connection).await?;
        if let Some(user) = user {
            let mut active_user: user::ActiveModel = user.into();
            active_user.name = Set(name);
            active_user.password = Set(password);
            return active_user.save(connection).await.unwrap().try_into_model();
        } else {
            return Err(DbErr::RecordNotFound("User not found".to_string()));
        }
    }
}
