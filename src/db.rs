use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::models::{User, FileData, InAppPurchase};

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let pool = PgPool::connect(database_url).await?;
        
        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;
        
        Ok(Self { pool })
    }

    // User operations
    pub async fn create_user(&self, email: &str, password_hash: &str) -> anyhow::Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, email, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, email, password_hash, dongle, created_at, updated_at
            "#,
            Uuid::new_v4(),
            email,
            password_hash,
            Utc::now(),
            Utc::now()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, password_hash, dongle, created_at, updated_at FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update_user_password(&self, user_id: Uuid, password_hash: &str) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE users SET password_hash = $1, updated_at = $2 WHERE id = $3",
            password_hash,
            Utc::now(),
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn set_user_dongle(&self, user_id: Uuid, dongle: &str) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE users SET dongle = $1, updated_at = $2 WHERE id = $3",
            dongle,
            Utc::now(),
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // File operations
    pub async fn create_file(&self, user_id: Uuid, path: &str, content: &str) -> anyhow::Result<FileData> {
        let file = sqlx::query_as!(
            FileData,
            r#"
            INSERT INTO files (id, user_id, path, content, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, user_id, path, content, created_at, updated_at
            "#,
            Uuid::new_v4(),
            user_id,
            path,
            content,
            Utc::now(),
            Utc::now()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(file)
    }

    pub async fn get_file(&self, user_id: Uuid, path: &str) -> anyhow::Result<Option<FileData>> {
        let file = sqlx::query_as!(
            FileData,
            "SELECT id, user_id, path, content, created_at, updated_at FROM files WHERE user_id = $1 AND path = $2",
            user_id,
            path
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(file)
    }

    pub async fn update_file(&self, user_id: Uuid, path: &str, content: &str) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE files SET content = $1, updated_at = $2 WHERE user_id = $3 AND path = $4",
            content,
            Utc::now(),
            user_id,
            path
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_file(&self, user_id: Uuid, path: &str) -> anyhow::Result<()> {
        sqlx::query!(
            "DELETE FROM files WHERE user_id = $1 AND path = $2",
            user_id,
            path
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_files(&self, user_id: Uuid, path_prefix: &str) -> anyhow::Result<Vec<FileData>> {
        let files = sqlx::query_as!(
            FileData,
            "SELECT id, user_id, path, content, created_at, updated_at FROM files WHERE user_id = $1 AND path LIKE $2",
            user_id,
            format!("{}%", path_prefix)
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(files)
    }

    // In-app purchase operations
    pub async fn get_or_create_purchase(&self, user_id: Uuid, app_name: &str) -> anyhow::Result<InAppPurchase> {
        // Try to get existing purchase
        if let Some(purchase) = sqlx::query_as!(
            InAppPurchase,
            "SELECT id, user_id, app_name, owned, consumed, created_at, updated_at FROM in_app_purchases WHERE user_id = $1 AND app_name = $2",
            user_id,
            app_name
        )
        .fetch_optional(&self.pool)
        .await?
        {
            return Ok(purchase);
        }

        // Create new purchase
        let purchase = sqlx::query_as!(
            InAppPurchase,
            r#"
            INSERT INTO in_app_purchases (id, user_id, app_name, owned, consumed, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, user_id, app_name, owned, consumed, created_at, updated_at
            "#,
            Uuid::new_v4(),
            user_id,
            app_name,
            5, // Default 5 saves
            0, // Default 0 consumed
            Utc::now(),
            Utc::now()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(purchase)
    }

    pub async fn update_purchase(&self, user_id: Uuid, app_name: &str, owned: i32, consumed: i32) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE in_app_purchases SET owned = $1, consumed = $2, updated_at = $3 WHERE user_id = $4 AND app_name = $5",
            owned,
            consumed,
            Utc::now(),
            user_id,
            app_name
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}