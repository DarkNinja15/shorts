use diesel::{deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Queryable,Serialize,Deserialize,Insertable,Debug)]
#[table_name="users"]
pub struct User{
    pub email:String,
    pub name:String,
    pub password:String,
}