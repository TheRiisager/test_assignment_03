use chrono::NaiveDate;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Employee {
    pub id: i32,
    pub birthdate: NaiveDate,
    pub first_name: String,
    pub last_name: String
}