use chrono::NaiveDate;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Booking {
    pub id: i32,
    pub customer_id: i32,
    pub employee_id: i32,
    pub booking_date: NaiveDate,
    pub start_time: String,
    pub end_time: String
}