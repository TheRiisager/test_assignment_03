use chrono::NaiveDate;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Customer {
    pub id: i32,
    pub birthdate: NaiveDate,
    pub first_name: String,
    pub last_name: String,
    pub mobile: String
}

pub struct NewCustomer {
    pub birthdate: NaiveDate,
    pub first_name: String,
    pub last_name: String,
    pub mobile: String
}