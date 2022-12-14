use std::error::Error;

use mockall::automock;

#[automock]
pub trait Repository<T> {
    fn create(&self, data: T) -> Result<T, Box<dyn Error>>;

    fn get_by_id(&self, data: T) -> Result<T, Box<dyn Error>>;
}