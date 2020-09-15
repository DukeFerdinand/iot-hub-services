use std::env;
use std::fmt::Display;

use rusoto_core::credential::{AwsCredentials, EnvironmentProvider, ProvideAwsCredentials};
use rusoto_core::{HttpClient, Region, RusotoError};

use rusoto_s3::{PutObjectError, PutObjectRequest, S3Client, S3};

pub struct S3Service {
    region: Region,
    s3: S3Client,
    bucket_name: String,
    endpoint: String,
}

impl S3Service {
    pub async fn new() -> Result<S3Service, Box<dyn std::error::Error>> {
        // first get all env stuff
        let creds = EnvironmentProvider::default().credentials().await;
        let s3_region = env::var("S3_REGION");
        let s3_endpoint = env::var("S3_ENDPOINT");
        let s3_bucket = env::var("S3_BUCKET");

        if let Ok(c) = creds {
            if s3_region.is_err() || s3_endpoint.is_err() || s3_bucket.is_err() {
                panic!("Could not get all required env keys. Please make sure they are present when running this service.\n keys:\n {:?}\n {:?}\n {:?}",
                    s3_region, s3_endpoint, s3_bucket
                )
            }

            let region = Region::Custom {
                name: s3_region.clone().unwrap(),
                endpoint: s3_endpoint.clone().unwrap(),
            };

            Ok(S3Service {
                region: region.to_owned(),
                s3: S3Client::new(region),
                bucket_name: s3_bucket.unwrap(),
                endpoint: s3_endpoint.unwrap(),
            })
        } else {
            Err(Box::new(creds.unwrap_err()))
        }
    }

    fn format_object_url(&self, file_name: &str) -> String {
        // https://url/bucket/file
        format!("{}/{}/{}", self.endpoint, self.bucket_name, file_name)
    }

    ///
    /// # put_object
    /// Take a file (as bytes) and put into s3 bucket. Returns s3 url if successful
    pub async fn put_object(
        &self,
        file_name: &str,
        file: Vec<u8>,
        acl: Option<String>,
    ) -> Result<String, RusotoError<PutObjectError>> {
        let request = PutObjectRequest {
            bucket: self.bucket_name.to_owned(),
            key: file_name.to_owned(),
            body: Some(file.into()),
            acl,

            // Tons more options that I don't need
            ..Default::default()
        };

        let res = self.s3.put_object(request).await;

        match res {
            Ok(_) => Ok(self.format_object_url(file_name)),
            Err(why) => Err(why),
        }
    }
}
