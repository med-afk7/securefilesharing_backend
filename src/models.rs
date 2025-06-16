use serde::{Deserialize, Serialize};

#[derive(Debug,Clone , Deserialize,Serialize,sqlx::FromRow , sqlx::Type)]

pub struct User{
    pub id :uuid::Uuid,
    pub name :String,
    pub email:String,
    pub password :String,
    pub public_key: Option <String>,
    pub created_at:Option <DateTime<Utc>>,
    pub updated_at:Option <DateTime<Utc>>,

}

#[derive(Debug,Clone , Deserialize,Serialize,sqlx::FromRow , sqlx::Type)]
pub struct  File{
    pub id :uuid::Uuid,
    pub user_id :Option<uuid:Uuid>,
    pub file_name:String,
    pub file_size :i64,
    pub encrypted_aes_key:vec<u8>,
     pub encrypted_file:vec<u8>,
     pub iv:vec<u8>,
    pub created_at:Option <DateTime<Utc>>,
   
}

#[derive(Debug,Clone , Deserialize,Serialize,sqlx::FromRow , sqlx::Type)]
pub struct  ShareLink{
    pub id :uuid::Uuid,
    pub file_id :Option<uuid:Uuid>,
    pub recipient_user_id :Option<uuid:Uuid>,
    pub password:String,
    pub expiration_date:Option <DateTime<Utc>>,
    pub created_at:Option <DateTime<Utc>>,
   
}

#[derive(sqlx::FromRow)]

pub struct SentFileDetails{
    pub file_id:uuid::Uuid,
    pub file_name:String,
    pub recipient_email:String,
    pub expiration_date:Option <DateTime<Utc>>,
    pub created_at:Option <DateTime<Utc>>,
}


#[derive(sqlx::FromRow)]

pub struct RecieveFileDetails{
    pub file_id:uuid::Uuid,
    pub file_name:String,
    pub sender_email:String,
    pub expiration_date:Option <DateTime<Utc>>,
    pub created_at:Option <DateTime<Utc>>,
}