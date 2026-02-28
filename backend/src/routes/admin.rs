use axum::{
    extract::{Path, Query, State, Multipart},
    routing::{get, post, put, patch},
    Json, Router,
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    error::{ApiError, ApiResult},
    models::*,
    services::{auth::{verify_password, generate_jwt}, s3},
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/products", get(get_admin_products).post(create_product))
        .route("/products/:id", put(update_product).delete(delete_product))
        .route("/products/:id/toggle", patch(toggle_product))
        .route("/categories", get(get_admin_categories).post(create_category))
        .route("/categories/:id", put(update_category).delete(delete_category))
        .route("/quotes", get(get_quotes))
        .route("/quotes/:id", get(get_quote_by_id))
        .route("/quotes/:id/status", patch(update_quote_status))
        .route("/upload", post(upload_image))
        // El middleware de autenticación debe aplicarse en main.rs o handlers individuales
        // por el momento los handlers verificarán la autenticación manualmente
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> ApiResult<Json<LoginResponse>> {
    // Validar entrada
    payload.validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;
    
    // Buscar admin por email
    let admin = sqlx::query_as::<_, Admin>(
        "SELECT * FROM admins WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::Auth("Credenciales inválidas".to_string()))?;
    
    // Verificar contraseña
    let is_valid = verify_password(&payload.password, &admin.password_hash)?;
    if !is_valid {
        return Err(ApiError::Auth("Credenciales inválidas".to_string()));
    }
    
    // Actualizar último login
    sqlx::query("UPDATE admins SET last_login = NOW() WHERE id = $1")
        .bind(admin.id)
        .execute(&state.db)
        .await?;
    
    // Generar JWT
    let token = generate_jwt(&admin.email, &state.config.jwt_secret)?;
    
    Ok(Json(LoginResponse {
        token,
        admin: AdminInfo {
            id: admin.id,
            email: admin.email,
            name: admin.name,
        },
    }))
}

#[derive(Debug, Deserialize)]
pub struct AdminProductQuery {
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub active: Option<bool>,
}

async fn get_admin_products(
    State(state): State<AppState>,
    Query(params): Query<AdminProductQuery>,
) -> ApiResult<Json<ProductListResponse>> {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = (page - 1) * limit;
    
    let mut query = String::from("SELECT * FROM products");
    
    if let Some(active) = params.active {
        query.push_str(&format!(" WHERE is_active = {}", active));
    }
    
    query.push_str(" ORDER BY created_at DESC");
    query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));
    
    let products: Vec<Product> = sqlx::query_as(&query)
        .fetch_all(&state.db)
        .await?;
    
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products")
        .fetch_one(&state.db)
        .await?;
    
    Ok(Json(ProductListResponse {
        products,
        total: total.0,
        page,
        limit,
    }))
}

async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<CreateProductRequest>,
) -> ApiResult<Json<Product>> {
    let product = sqlx::query_as::<_, Product>(
        r#"
        INSERT INTO products (
            name, slug, description, category_id, specifications, regulatory_info
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#
    )
    .bind(&payload.name)
    .bind(&payload.slug)
    .bind(&payload.description)
    .bind(&payload.category_id)
    .bind(payload.specifications.unwrap_or(serde_json::json!({})))
    .bind(payload.regulatory_info.unwrap_or(serde_json::json!({})))
    .fetch_one(&state.db)
    .await?;
    
    Ok(Json(product))
}

async fn update_product(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateProductRequest>,
) -> ApiResult<Json<Product>> {
    let product = sqlx::query_as::<_, Product>(
        r#"
        UPDATE products 
        SET name = COALESCE($1, name),
            slug = COALESCE($2, slug),
            description = COALESCE($3, description),
            is_active = COALESCE($4, is_active),
            updated_at = NOW()
        WHERE id = $5
        RETURNING *
        "#
    )
    .bind(&payload.name)
    .bind(&payload.slug)
    .bind(&payload.description)
    .bind(&payload.is_active)
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Product not found".to_string()))?;
    
    Ok(Json(product))
}

async fn delete_product(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> ApiResult<Json<serde_json::Value>> {
    let result = sqlx::query("DELETE FROM products WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Product not found".to_string()));
    }
    
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Product deleted successfully"
    })))
}

async fn toggle_product(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> ApiResult<Json<Product>> {
    let product = sqlx::query_as::<_, Product>(
        "UPDATE products SET is_active = NOT is_active WHERE id = $1 RETURNING *"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Product not found".to_string()))?;
    
    Ok(Json(product))
}

async fn get_admin_categories(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<Category>>> {
    let categories = sqlx::query_as::<_, Category>(
        "SELECT * FROM categories ORDER BY name ASC"
    )
    .fetch_all(&state.db)
    .await?;
    
    Ok(Json(categories))
}

async fn create_category(
    State(state): State<AppState>,
    Json(payload): Json<CreateCategoryRequest>,
) -> ApiResult<Json<Category>> {
    let category = sqlx::query_as::<_, Category>(
        "INSERT INTO categories (name, slug, description) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.slug)
    .bind(&payload.description)
    .fetch_one(&state.db)
    .await?;
    
    Ok(Json(category))
}

async fn update_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> ApiResult<Json<Category>> {
    let category = sqlx::query_as::<_, Category>(
        r#"
        UPDATE categories 
        SET name = COALESCE($1, name),
            slug = COALESCE($2, slug),
            description = COALESCE($3, description)
        WHERE id = $4
        RETURNING *
        "#
    )
    .bind(&payload.name)
    .bind(&payload.slug)
    .bind(&payload.description)
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Category not found".to_string()))?;
    
    Ok(Json(category))
}

async fn delete_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> ApiResult<Json<serde_json::Value>> {
    let result = sqlx::query("DELETE FROM categories WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Category not found".to_string()));
    }
    
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Category deleted successfully"
    })))
}

#[derive(Debug, Deserialize)]
pub struct QuoteQuery {
    pub status: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

async fn get_quotes(
    State(state): State<AppState>,
    Query(params): Query<QuoteQuery>,
) -> ApiResult<Json<QuoteListResponse>> {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = (page - 1) * limit;
    
    let mut query = String::from("SELECT * FROM quotes");
    
    if let Some(status) = &params.status {
        query.push_str(&format!(" WHERE status = '{}'", status));
    }
    
    query.push_str(" ORDER BY created_at DESC");
    query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));
    
    let quotes: Vec<Quote> = sqlx::query_as(&query)
        .fetch_all(&state.db)
        .await?;
    
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM quotes")
        .fetch_one(&state.db)
        .await?;
    
    Ok(Json(QuoteListResponse {
        quotes,
        total: total.0,
        page,
        limit,
    }))
}

async fn get_quote_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> ApiResult<Json<Quote>> {
    let quote = sqlx::query_as::<_, Quote>(
        "SELECT * FROM quotes WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Quote not found".to_string()))?;
    
    Ok(Json(quote))
}

async fn update_quote_status(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateQuoteStatusRequest>,
) -> ApiResult<Json<Quote>> {
    let quote = sqlx::query_as::<_, Quote>(
        r#"
        UPDATE quotes 
        SET status = $1, 
            notes = COALESCE($2, notes),
            contacted_at = CASE WHEN $1 = 'contacted' THEN NOW() ELSE contacted_at END
        WHERE id = $3
        RETURNING *
        "#
    )
    .bind(&payload.status)
    .bind(&payload.notes)
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Quote not found".to_string()))?;
    
    Ok(Json(quote))
}

async fn upload_image(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> ApiResult<Json<serde_json::Value>> {
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        ApiError::BadRequest(format!("Failed to read multipart field: {}", e))
    })? {
        let name = field.name().unwrap_or("").to_string();
        
        if name == "file" {
            let content_type = field.content_type()
                .ok_or_else(|| ApiError::BadRequest("Missing content type".to_string()))?
                .to_string();
            
            let data = field.bytes().await.map_err(|e| {
                ApiError::BadRequest(format!("Error al leer datos del archivo: {}", e))
            })?;
            
            // Validar tamaño del archivo (máx 5MB)
            if data.len() > 5 * 1024 * 1024 {
                return Err(ApiError::BadRequest("Archivo muy grande (máx 5MB)".to_string()));
            }
            
            let url = s3::upload_image(
                &state.s3,
                &state.config.aws_s3_bucket,
                data.to_vec(),
                &content_type,
            ).await?;
            
            return Ok(Json(serde_json::json!({
                "url": url
            })));
        }
    }
    
    Err(ApiError::BadRequest("No file provided".to_string()))
}
