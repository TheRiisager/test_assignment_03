use std::error::Error;

use crate::models::customer::{Customer, NewCustomer};

pub struct CustomerService {
}

impl CustomerService {
    pub fn create_customer(&self, customer: NewCustomer) -> Result<Customer, Box<dyn Error>>{
        todo!()
    }

    pub fn get_by_id(&self, id: i32) -> Result<Customer, Box<dyn Error>>{
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::models::customer::NewCustomer;

    use super::*;

    #[test]
    fn test_create_customer() {
        let service = CustomerService {
        };
        let customer = NewCustomer {
            birthdate: NaiveDate::from_ymd_opt(1998, 10, 30).expect("date should be valid"),
            first_name: String::from("Frederik"),
            last_name: String::from("Johnsen"),
            mobile: String::from("12345678"),
        };
        let customer_created = match service.create_customer(customer) {
            Ok(_) => true,
            Err(_) => false,
        };
        assert_eq!(customer_created, true);
    }

    #[test]
    fn test_get_by_id() {
        let service = CustomerService {
        };
        let customer = NewCustomer {
            birthdate: NaiveDate::from_ymd_opt(1998, 10, 30).expect("date should be valid"),
            first_name: String::from("Frederik"),
            last_name: String::from("Johnsen"),
            mobile: String::from("12345678"),
        };

        let created_customer = service.create_customer(customer).unwrap();
        let fetched_customer = service.get_by_id(created_customer.id).unwrap();
        assert_eq!(created_customer.id, fetched_customer.id);
        assert_eq!(created_customer.first_name, fetched_customer.first_name);
        assert_eq!(created_customer.mobile, fetched_customer.mobile);
    }
}