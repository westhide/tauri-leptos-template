// use std::ops::Deref;

// pub type PgPool = Pool<ConnectionManager<PgConnection>>;

// #[derive(Debug)]
// pub struct Database {
//     pool: PgPool,
// }

// impl Database {
//     pub fn new(url: &str) -> Result<Self, PoolError> {
//         let manager = ConnectionManager::new(url);
//         let pool = Pool::builder().build(manager)?;
//         Ok(Self { pool })
//     }
// }

// impl Deref for Database {
//     type Target = PgPool;

//     fn deref(&self) -> &Self::Target {
//         &self.pool
//     }
// }
