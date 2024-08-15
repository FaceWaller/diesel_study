use std::sync::Arc;
use lazy_static::*;

mod error;
pub use error::*;
mod pool;
use pool::*;

pub use diesel_derives::*;
pub use diesel_migrations::*;
pub extern crate diesel;

lazy_static! {
    pub static ref IMCONNPOOL: Arc<DBPool> = Arc::new(DBPool::new("./test.sqlite").unwrap());
}

pub fn get_conn() -> MyResult<DBConn> {
    IMCONNPOOL.connect()
}

