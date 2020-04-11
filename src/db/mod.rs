use diesel::PgConnection;

pub mod schema;

#[database("quantiz_db")]
pub struct DbConnection(PgConnection);