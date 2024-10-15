use std::ops::{Deref, DerefMut};
use sqlx::{Pool, Postgres};
use crate::types::Writer;

impl Deref for Writer {
    type Target = Pool<Postgres>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Writer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
