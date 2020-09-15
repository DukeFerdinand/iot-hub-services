use std::collections::HashMap;

use actix_multipart::{Field, Multipart};
use actix_web::http::{header::ContentDisposition, StatusCode};
use actix_web::{get, middleware, post, web::Bytes, App, HttpResponse, HttpServer, Responder};

use futures::{StreamExt, TryStreamExt};
use mime::*;

use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;

use serde::{Deserialize, Serialize};

mod services;

use services::s3_service::S3Service;

struct Storage {
    name: String,
    region: Region,
    credentials: Credentials,
    bucket: String,
    location_supported: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct UserTest {
    // Comes in as stream, will need to be created with push to s3
    pub icon: String,
    pub username: String,
    // Not to be used outside of impl
    password: String,
}

impl UserTest {
    fn to_hash_map(&self) -> HashMap<String, &str> {
        let mut hash = HashMap::<String, &str>::new();

        // TODO: Find out how to do this automatically
        hash.insert("username".into(), &self.username);
        // Don't leak password
        // hash.insert("password".into(), &self.password);
        hash.insert("icon".into(), &self.icon);

        hash
    }

    fn from_hash_map(hash: HashMap<String, String>) -> UserTest {
        let mut u = UserTest {
            ..Default::default()
        };

        for key in hash.iter() {
            match key.0.as_str() {
                "username" => u.username = key.1.to_string(),
                "password" => u.password = key.1.to_string(),
                "icon" => u.icon = key.1.to_string(),
                s => println!("[UNIMPLEMENTED at User] Got unknown key -> {}", s),
            }
        }

        u
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    println!("Print");

    let creds =
        Credentials::from_env_specific(Some("S3_ACCESS_KEY"), Some("S3_SECRET_KEY"), None, None);

    if creds.is_err() {
        return HttpResponse::from("Got Error at s3 creation");
    }

    // iot-bucket.us-east-1.linodeobjects.com
    let linode = Storage {
        name: "iot-bucket".into(),
        region: Region::Custom {
            region: "us-east-1".into(),
            endpoint: "https://us-east-1.linodeobjects.com".into(),
        },
        credentials: creds.unwrap(),
        bucket: "iot-bucket".to_string(),
        location_supported: false,
    };

    let bucket = Bucket::new(&linode.bucket, linode.region, linode.credentials);

    if bucket.is_err() {
        println!("Got error creating bucket");
        return HttpResponse::from("Got Error at bucket creation");
    }

    let bucket = bucket.unwrap();
    let file = bucket.get_object("Tux.png").await;
    println!("{:?}", file.is_ok());

    // await bucket.put_object(path: S, content: &[u8])

    HttpResponse::Ok().body("Hello world!")
}

#[post("/signup")]
async fn signup(mut payload: Multipart) -> impl Responder {
    let mut user_hash: HashMap<String, String> = HashMap::new();
    // Multipart payload returns a stream, so we need to iterate over it
    while let Ok(Some(mut field)) = payload.try_next().await {
        // Just to get type completion, not really needed
        let content_type: Option<ContentDisposition> = field.content_disposition();

        if content_type.is_none() {
            return HttpResponse::from("Got error");
        }

        let name = content_type.clone().unwrap();

        match name.get_name() {
            // Current field is username, get the bytes
            Some("icon") => {
                let f = content_type.unwrap();
                let file_name = f.get_filename().unwrap();
                while let Some(chunk) = field.next().await {
                    let bytes = chunk.unwrap();
                    let s3 = S3Service::new().await;

                    if s3.is_err() {
                        return HttpResponse::InternalServerError()
                            .body(format!("Error creating s3 service"));
                    }

                    let res = s3
                        .unwrap()
                        .put_object(file_name, bytes.to_vec(), Some("public-read".into()))
                        .await;

                    if res.is_ok() {
                        println!("putting icon into user_hash");
                        user_hash.insert("icon".into(), res.unwrap());
                    } else {
                        println!("Broke at s3 service: {}", res.unwrap_err());
                        return HttpResponse::InternalServerError()
                            .body("Unable to put object into s3");
                    }
                }
            }
            Some(k) => {
                while let Some(chunk) = field.next().await {
                    let bytes = chunk.unwrap();
                    match String::from_utf8(bytes.to_vec()) {
                        Ok(v) => {
                            user_hash.insert(k.to_string(), v);
                        }
                        Err(e) => println!("{:?}", e),
                    }
                }
            }
            None => println!("ERROR, got None match for name.get_name()"),
        }
    }

    let user = UserTest::from_hash_map(user_hash);

    println!("{:?}", user);

    let string = serde_json::to_string(&user);

    if string.is_ok() {
        HttpResponse::Ok().body(string.unwrap())
    } else {
        HttpResponse::from("error serializing user data")
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// Start server

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Actix server");
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(hello)
            .service(echo)
            .service(signup)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
