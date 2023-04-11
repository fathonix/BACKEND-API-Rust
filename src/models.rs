use crate::schema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub price: u32,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::products)]
pub struct NewProduct {
    pub name: String,
    pub price: u32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct InputProduct {
    pub name: String,
    pub price: u32,
}

#[derive(Serialize, Deserialize)]
pub struct OutputProduct<'a> {
    pub status: bool,
    pub message: &'a str,
    pub data: Option<Product>
}

#[derive(Serialize, Deserialize)]
pub struct OutputProducts<'a> {
    pub status: bool,
    pub message: &'a str,
    pub data: Option<Vec<Product>>
}

#[derive(Serialize, Deserialize)]
pub struct OutputError<'a> {
    pub status: bool,
    pub message: &'a str
}