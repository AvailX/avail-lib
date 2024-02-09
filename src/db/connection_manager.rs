use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use std::fmt;

use crate::errors::AvailResult;

type DbConnectionManager = AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>;
type DbConnectionPool = deadpool::managed::Pool<DbConnectionManager>;

pub type DbConnection = deadpool::managed::Object<DbConnectionManager>;

#[derive(Clone)]
pub struct DbManager {
    connection_pool: DbConnectionPool,
}

impl DbManager {
    pub fn new(connection_string: &str, pool_max_size: usize) -> AvailResult<Self> {
        let pool = Pool::builder(DbConnectionManager::new(connection_string)).build()?;
        pool.resize(pool_max_size);

        Ok(Self {
            connection_pool: pool,
        })
    }

    pub async fn get_connection(&self) -> AvailResult<DbConnection> {
        Ok(self.connection_pool.get().await?)
    }
}

impl fmt::Debug for DbManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DbManager")
    }
}

// NOTE: This is used only for tests when mocking and a function takes a db manager as parameter.
impl PartialEq for DbManager {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for DbManager {}

pub mod mocks {
    use super::DbManager;

    pub fn db_manager() -> DbManager {
        DbManager::new("dummy_connection_string", 10).unwrap()
    }
}
