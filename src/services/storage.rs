use aws_sdk_s3::{Client, Error};
use aws_sdk_s3::primitives::ByteStream;
use crate::config::AppConfig;

pub struct StorageService {
    client: Client,
    bucket: String,
}

impl StorageService {
    pub fn new(config: &AppConfig) -> Self {
        let aws_config = aws_config::from_env()
            .region(aws_config::Region::new(config.aws_region.clone()));
        
        // Note: In a real implementation, you'd want to properly configure AWS credentials
        let client = Client::new(&aws_config::load_from_env().await);
        
        Self {
            client,
            bucket: config.s3_bucket.clone(),
        }
    }

    pub async fn put_object(&self, key: &str, data: &[u8]) -> Result<(), Error> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(ByteStream::from(data.to_vec()))
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_object(&self, key: &str) -> Result<Vec<u8>, Error> {
        let resp = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;

        let data = resp.body.collect().await?;
        Ok(data.into_bytes().to_vec())
    }

    pub async fn delete_object(&self, key: &str) -> Result<(), Error> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;

        Ok(())
    }

    pub async fn object_exists(&self, key: &str) -> Result<bool, Error> {
        match self.client
            .head_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(err) => {
                // Check if it's a "not found" error
                if let Some(service_err) = err.as_service_error() {
                    if service_err.code() == Some("NotFound") {
                        return Ok(false);
                    }
                }
                Err(err)
            }
        }
    }
}