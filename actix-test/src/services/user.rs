use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserTest {
    // Comes in as stream, will need to be created with push to s3
    pub icon: String,
    pub username: String,
    // Not to be used outside of impl
    password: String,
}

pub struct UserService {
    pub user: UserTest,
}

impl UserService {
    pub fn 
}
