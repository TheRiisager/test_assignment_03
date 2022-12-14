// @generated automatically by Diesel CLI.

diesel::table! {
    bookings (id) {
        id -> Int4,
        customer_id -> Nullable<Int4>,
        booking_date -> Date,
        employee_id -> Nullable<Int4>,
        end_time -> Varchar,
        start_time -> Varchar,
    }
}

diesel::table! {
    customers (id) {
        id -> Int4,
        birthdate -> Nullable<Date>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        mobile -> Nullable<Varchar>,
    }
}

diesel::table! {
    employees (id) {
        id -> Int4,
        birthdate -> Nullable<Date>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
    }
}

diesel::joinable!(bookings -> customers (customer_id));
diesel::joinable!(bookings -> employees (employee_id));

diesel::allow_tables_to_appear_in_same_query!(
    bookings,
    customers,
    employees,
);
