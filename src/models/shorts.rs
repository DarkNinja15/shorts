use diesel::{deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};
use crate::schema::shorts;

#[derive(Queryable,Serialize,Deserialize,Insertable,Debug)]
#[table_name="shorts"]
pub struct Shorts{
    pub id:String,
    pub ref_url:String,
    pub title:String,
    pub description:String,
    pub author:String,
}