use r2d2::Pool;
use redis::{Client, Commands};

#[derive(Clone)]
pub struct Data {
    pool: Pool<Client>,
}

impl Data {
    pub fn new(url: String) -> Self {
        let client = Client::open(url).expect("Error while opening redis");
        let pool = Pool::builder().build(client).unwrap();
        Self { pool }
    }

    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        let mut conn = self.pool.get().unwrap();
        conn.get(key).unwrap()
    }

    pub async fn set(&self, key: &str, value: &[u8], ex: Option<i64>) {
        let mut conn = self.pool.get().unwrap();
        conn.set::<_, _, ()>(key, value).unwrap();
        if let Some(ex) = ex {
            conn.expire::<_, ()>(key, ex).unwrap();
        }
    }

    pub async fn exists(&self, key: &str) -> bool {
        let mut conn = self.pool.get().unwrap();
        conn.exists(key).unwrap()
    }
}
