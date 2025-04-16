use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name = "UserRole", rename_all = "lowercase")]

pub enum UserRole {
Admin,
User,
}


impl UserRole {
  pub fn to_string(&self) -> String {
    match self {
      UserRole::Admin => "admin".to_string(),
      UserRole::User => "user".to_string(),
    }
  }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type, Clone)]
#[sqlx(type_name = "User", rename_all = "camelCase")]
pub struct User {
  pub id:: uuid:Uuid,
  pub name::String,
  pub email::String,
  pub password::String,
  pub role::UserRole,
  pub verified::bool,
  pub verification_token: Option<String>,
  pub token_expiry: Option<DateTime<Utc>>,
  pub created_at::DateTime<Utc>,
  pub updated_at::DateTime<Utc>,
}


