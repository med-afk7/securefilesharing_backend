




use sqlx::{Pool, Postgres};

use crate::models::{File, RecieveFileDetails, SentFileDetails, ShareLink , User};

#[derive (Debug,Clone)]
pub struct DBClient{
    pool: Pool<Postgres>,
}

impl DBClient {
    pub fn new (pool:Pool<Postgres>) -> Self{
        DBClient { pool }
    }
}

#[async_trait]
pub trait UserExt{
    
}

