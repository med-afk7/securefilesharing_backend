use core::str;
use chrono::{DateTime , Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::models::{RecieveFileDetails , SentFileDetails, User};

#[derive(Validate, Debug,Default,Clone,Serialize,Deserialize)]
pub struct RegisterUserDto{

#[validate(length(min=1,message="name is required"))]
pub name:String,

#[validate(length(min=1,message="email is required"),email(message="Email is not valid"))]
pub email:String,

#[validate( length(min=6,message = "password must be at least 6 characters"))]
pub password:String,

#[validate(length(min=1, message ="confirm password is required") , must_match(other="password", message="passwords do not match"))]
#[serde(rename="passwordConfirm")]
pub password_confirm: String,
}

#[derive(Validate, Debug , Default,Clone,Serialize,Deserialize)]
pub struct  LoginUserDto{
    #[validate(length(min=1,message="Email is required") , email(message = "Email is invalid "))]
    pub email:String,
    #[validate( length(min=6,message = "password must be at least 6 characters"))]
pub password:String,
}

#[derive(Serialize,Deserialize,Validate)]
pub struct RequestQueryDto{
    #[validate(range(min=1))]
    pub page:Option<usize>,
    #[validate(range(min=1,max=50))]
    pub limit:Option<usize>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct FilterUserDto{
    pub id:String,
    pub name:String,
    pub email:String,
    pub public_key:Option<String>,
    pub created_at:DateTime<Utc>,
    pub updated_at:DateTime<Utc>,

}

impl FilterUserDto{
    pub fn filter_user(user:&User) -> Self{
        FilterUserDto{
            id:user.id.to_string(),
            name:user.name.to_owned(),
            email:user.email.to_owned(),
            public_key:user.public_key.to_owned(),
            created_at:user.created_at.unwrap(),
            updated_at:user.updated_at.unwrap(),
        }
    }
}

#[derive(Debug,Serialize,Deserialize)]

pub struct UserDate{
    pub user:FilterUserDto,

}

#[derive(Debug,Serialize,Deserialize)]
pub struct UserResponseDto{
    pub status:String,
    pub data:UserData,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct UserSendFileDto{
    pub file_id:String,
    pub file_name:String,
    pub recipient_email:String,
    pub expiration_date:DateTime<Utc>,
    pub created_at:DateTime<Utc>,
}

impl UserSendFileDto{
    pub fn filter_send_user_file(file_data: &SentFileDetails) -> Self{
        UserSendFileDto { 
            file_id: file_data.file_id.to_string(),
            file_name:file_data,file_name.to_owned (),
            recipient_email:file_data.recipient_email.to_owned (),
            expiration_date:file_data.expiration_date.unwrap (),
            created_at:file_data.created_at.unwrap (),
            
             }
    }
}

pub fn filter_send_user_file(user: &[SentFileDetails]) -> vec<UserSendFileDto>{
    user.iter().map(UserSendFileDto::filter_send_user_file).collect()
}