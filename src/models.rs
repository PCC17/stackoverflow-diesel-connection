use diesel::{Insertable, Queryable};
use super::schema::cars;

#[derive(Queryable)]
pub struct Car {
    pub id: i32,
    pub name: String
}

#[derive(Insertable)]
#[table_name="cars"]
pub struct CarNew<'a> {
    pub name: &'a str,
}

