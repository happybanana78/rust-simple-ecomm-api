use crate::errors::error::AppError;
use crate::traits::UseStorage;
use actix_multipart::form::tempfile::TempFile;
use bytes::Bytes;
use futures_util::TryFutureExt;
use tokio::fs;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, copy};

pub struct LocalStorage {
    pub base_path: String,
}

pub struct S3Storage {
    pub bucket: String,
    pub region: String,
}

impl LocalStorage {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }
}

impl UseStorage for LocalStorage {
    async fn upload(&self, path: &str, ext: &str, bytes: Bytes) -> Result<String, AppError> {
        let full_path = format!("{}/{}.{}", self.base_path, path, ext);

        fs::create_dir_all(&self.base_path)
            .map_err(|e| AppError::Internal(e.to_string()))
            .await?;

        let mut file = File::create(&full_path)
            .map_err(|e| AppError::Internal(e.to_string()))
            .await?;

        file.write_all(&bytes)
            .map_err(|e| AppError::Internal(e.to_string()))
            .await?;

        Ok(full_path)
    }

    async fn upload_from_temp(&self, path: &str, temp_file: TempFile) -> Result<String, AppError> {
        let full_path = format!(
            "{}/{}.{}",
            self.base_path,
            path,
            self.mime_to_extension(&temp_file.content_type.unwrap())
        );

        fs::create_dir_all(&self.base_path)
            .map_err(|e| AppError::Internal(e.to_string()))
            .await?;

        let mut src = File::from_std(temp_file.file.into_file());
        let mut dst = File::create(format!("./{}", full_path))
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        copy(&mut src, &mut dst)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(full_path)
    }

    async fn delete(&self, path: &str) -> Result<(), AppError> {
        fs::remove_file(path)
            .map_err(|e| AppError::Internal(e.to_string()))
            .await
    }
}

impl S3Storage {
    pub fn new(bucket: String, region: String) -> Self {
        Self { bucket, region }
    }
}
