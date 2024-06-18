use diesel::{deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};
use crate::schema::shorts;

#[derive(Queryable,Serialize,Deserialize,Insertable,Debug)]
#[table_name="shorts"]
pub struct Shorts{
    id:String,
    ref_url:String,
    title:String,
    description:String,
    author:String,
}