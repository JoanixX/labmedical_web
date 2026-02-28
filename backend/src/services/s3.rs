use aws_config::BehaviorVersion;
use aws_sdk_s3::{Client, primitives::ByteStream};
use crate::{config::Config, error::{ApiError, ApiResult}};
use uuid::Uuid;

pub async fn create_client(config: &Config) -> Client {
    let aws_config = aws_config::defaults(BehaviorVersion::latest())
        .region(aws_config::Region::new(config.aws_region.clone()))
        .load()
        .await;
    
    Client::new(&aws_config)
}

pub async fn upload_image(
    client: &Client,
    bucket: &str,
    file_data: Vec<u8>,
    content_type: &str,
) -> ApiResult<String> {
    // generamos el nombre de archivo único
    let extension = match content_type {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/webp" => "webp",
        _ => return Err(ApiError::BadRequest("Tipo de imagen inválido".to_string())),
    };
    
    let filename = format!("{}.{}", Uuid::new_v4(), extension);
    let key = format!("products/{}", filename);
    
    // Subir a S3
    client
        .put_object()
        .bucket(bucket)
        .key(&key)
        .body(ByteStream::from(file_data))
        .content_type(content_type)
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("Error al subir a S3: {}", e)))?;
    
    // Retornar URL pública
    let url = format!("https://{}.s3.amazonaws.com/{}", bucket, key);
    Ok(url)
}

#[allow(dead_code)]
pub async fn delete_image(
    client: &Client,
    bucket: &str,
    url: &str,
) -> ApiResult<()> {
    // se extrae la key de la URL
    let key = url
        .split(&format!("{}.s3.amazonaws.com/", bucket))
        .nth(1)
        .ok_or_else(|| ApiError::BadRequest("URL de S3 inválida".to_string()))?;
    
    client
        .delete_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("Error al eliminar de S3: {}", e)))?;
    
    Ok(())
}
