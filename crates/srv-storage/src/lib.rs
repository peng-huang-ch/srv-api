#[cfg(not(feature = "async"))]
mod pg;

#[cfg(not(feature = "async"))]
pub use pg::{init_db, DbConnection, DbConnectionManger, DbError, DbPool, DbRunError};

#[cfg(feature = "async")]
mod pg_async;

#[cfg(feature = "async")]
pub use pg_async::{init_db, DbConnection, DbConnectionManger, DbError, DbPool, DbRunError};

pub mod prelude {
    pub use crate::{init_db, DbConnection, DbConnectionManger, DbError, DbPool, DbRunError};

    #[cfg(feature = "async")]
    pub use diesel_async::RunQueryDsl;

    #[cfg(not(feature = "async"))]
    pub use diesel::RunQueryDsl;
}

pub use diesel;
pub mod models;
pub mod schema;