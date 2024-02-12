use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub created_at: String, 
    pub updated_at: String 
}