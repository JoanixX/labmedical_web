use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub category_id: Option<i32>,
    pub specifications: serde_json::Value,
    pub image_url: Option<String>,
    pub additional_images: serde_json::Value,
    pub regulatory_info: serde_json::Value,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub category_id: Option<i32>,
    pub specifications: Option<serde_json::Value>,
    pub regulatory_info: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<i32>,
    pub specifications: Option<serde_json::Value>,
    pub regulatory_info: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ProductListResponse {
    pub products: Vec<Product>,
    pub total: i64,
    pub page: i32,
    pub limit: i32,
}