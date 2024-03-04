use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewTodo {
    pub description: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}
#[derive(Deserialize, Debug)]
pub struct Login {
    pub username: String,
    pub password: String,
}
#[derive(Deserialize, Debug)]
pub struct Register {
    pub username: String,
    pub email: String,
    pub password: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

