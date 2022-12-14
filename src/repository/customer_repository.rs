use crate::models::customer::Customer;

use super::repository_trait::Repository;

pub struct CustomerRepository{}

impl Repository<Customer> for CustomerRepository {
    fn create(&self, data: Customer) -> Result<Customer, Box<dyn std::error::Error>> {
        todo!()
    }

    fn get_by_id(&self, data: Customer) -> Result<Customer, Box<dyn std::error::Error>> {
        todo!()
    }
}