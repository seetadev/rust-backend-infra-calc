use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_region: String,
    pub s3_bucket: String,
    pub ses_from_email: String,
    pub cookie_secret: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://localhost/aspiring_investments".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "11oETzKXQAGaYdkL5gEmGeJJFuYh7EQnp2XdTP1o/Vo=".to_string()),
            aws_access_key_id: env::var("AWS_ACCESS_KEY_ID").unwrap_or_default(),
            aws_secret_access_key: env::var("AWS_SECRET_ACCESS_KEY").unwrap_or_default(),
            aws_region: env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
            s3_bucket: env::var("S3_BUCKET").unwrap_or_else(|_| "aspiring-investments".to_string()),
            ses_from_email: env::var("SES_FROM_EMAIL")
                .unwrap_or_else(|_| "aspiring.investments@gmail.com".to_string()),
            cookie_secret: env::var("COOKIE_SECRET")
                .unwrap_or_else(|_| "11oETzKXQAGaYdkL5gEmGeJJFuYh7EQnp2XdTP1o/Vo=".to_string()),
        })
    }
}