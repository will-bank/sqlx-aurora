use sqlx::{Pool, Postgres};

pub struct Reader(Pool<Postgres>);
pub struct Writer(Pool<Postgres>);