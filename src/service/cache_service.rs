use crate::config::config::Setting;
use crate::error::Result;
use crate::service::{MemService, RedisService};
use futures_util::future::BoxFuture;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::Duration;

pub trait ICacheService: Sync + Send {
    fn set_string(&self, k: &str, v: &str) -> BoxFuture<Result<String>>;

    fn get_string(&self, k: &str) -> BoxFuture<Result<String>>;

    fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> BoxFuture<Result<String>>;

    fn ttl(&self, k: &str) -> BoxFuture<Result<i64>>;
}

pub struct CacheService {
    pub inner: Box<dyn ICacheService>,
}

impl CacheService {
    pub fn new(cfg: &Setting) -> crate::error::Result<Self> {
        match cfg.cache_type.as_str() {
            "mem" => {
                println!("[abs_admin] cache_type: mem");
                Ok(Self {
                    inner: Box::new(MemService::default()),
                })
            }
            "redis" => {
                println!("[abs_admin] cache_type: redis");
                Ok(Self {
                    inner: Box::new(RedisService::new(&cfg.redis_url)),
                })
            }
            e => {
                panic!(
                    "[abs_admin] unknown of cache_type: \"{}\",current support 'mem' or 'redis'",
                    e
                );
            }
        }
    }

    pub async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        self.inner.set_string(k, v).await
    }

    pub async fn get_string(&self, k: &str) -> Result<String> {
        self.inner.get_string(k).await
    }

    pub async fn set_json<T>(&self, k: &str, v: &T) -> Result<String>
    where
        T: Serialize + Sync,
    {
        let data = serde_json::to_string(v);
        if data.is_err() {
            return Err(crate::error::Error::from(format!(
                "MemCacheService set_json fail:{}",
                data.err().unwrap()
            )));
        }
        let data = self.set_string(k, data.unwrap().as_str()).await?;
        Ok(data)
    }

    pub async fn get_json<T>(&self, k: &str) -> Result<T>
    where
        T: DeserializeOwned + Sync,
    {
        let mut r = self.get_string(k).await?;
        if r.is_empty() {
            r = "null".to_string();
        }
        let data: serde_json::Result<T> = serde_json::from_str(r.as_str());
        if data.is_err() {
            return Err(crate::error::Error::from(format!(
                "MemCacheService GET fail:{}",
                data.err().unwrap()
            )));
        }
        Ok(data.unwrap())
    }

    pub async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> Result<String> {
        self.inner.set_string_ex(k, v, ex).await
    }

    pub async fn ttl(&self, k: &str) -> Result<i64> {
        self.inner.ttl(k).await
    }
}
