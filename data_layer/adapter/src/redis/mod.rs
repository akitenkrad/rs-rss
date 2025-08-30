use redis::{cmd, AsyncCommands, Client};
use shared::{
    config::RedisConfig,
    errors::{AppError, AppResult},
};

pub struct RedisClient {
    client: Client,
}

impl RedisClient {
    pub fn new(config: &RedisConfig) -> AppResult<Self> {
        let client = Client::open(format!("redis://{}:{}", config.host, config.port))?;
        Ok(Self { client })
    }

    pub async fn set_ex<T: RedisKey>(&self, key: &T, value: &T::Value, ttl: u64) -> AppResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        cmd("SETEX")
            .arg(&key.inner())
            .arg(ttl)
            .arg(&value.inner())
            .exec_async(&mut conn)
            .await?;
        Ok(())
    }

    pub async fn get<T: RedisKey>(&self, key: &T) -> AppResult<Option<T::Value>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let result: Option<String> = conn.get(key.inner()).await?;
        result.map(T::Value::try_from).transpose()
    }

    pub async fn delete<T: RedisKey>(&self, key: &T) -> AppResult<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        cmd("DEL").arg(&key.inner()).exec_async(&mut conn).await?;
        Ok(())
    }

    pub async fn try_connect(&self) -> AppResult<()> {
        let _ = self.client.get_multiplexed_async_connection().await?;
        Ok(())
    }
}

pub trait RedisKey {
    type Value: RedisValue + TryFrom<String, Error = AppError>;
    fn inner(&self) -> String;
}

pub trait RedisValue {
    fn inner(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::errors::AppError;

    #[derive(Debug, PartialEq, Eq)]
    pub struct TestContent {
        pub name: String,
    }

    pub struct TestContentKey(String);

    impl RedisKey for TestContentKey {
        type Value = TestContent;
        fn inner(&self) -> String {
            self.0.to_string()
        }
    }

    impl TryFrom<String> for TestContent {
        type Error = AppError;

        fn try_from(s: String) -> Result<Self, Self::Error> {
            Ok(Self { name: s })
        }
    }

    impl RedisValue for TestContent {
        fn inner(&self) -> String {
            self.name.to_string()
        }
    }

    #[test_log::test(tokio::test)]
    async fn test_connect() -> anyhow::Result<()> {
        let config = RedisConfig {
            host: std::env::var("REDIS_HOST").unwrap(),
            port: std::env::var("REDIS_INNER_PORT").unwrap().parse().unwrap(),
        };
        tracing::info!("Connecting to Redis at {}:{}", config.host, config.port);
        let client = RedisClient::new(&config)?;
        client.try_connect().await?;

        // try to get a non-existent token
        let res_nonexist = client.get(&TestContentKey("redis:key".to_string())).await?;
        assert!(res_nonexist.is_none());

        // set the token
        client
            .set_ex(
                &TestContentKey("redis:key".to_string()),
                &TestContent {
                    name: "bbb".to_string(),
                },
                1000,
            )
            .await?;

        // get the token
        let res = client.get(&TestContentKey("redis:key".to_string())).await?;
        assert_eq!(
            res,
            Some(TestContent {
                name: "bbb".to_string()
            })
        );

        // delete the token
        client.delete(&TestContentKey("redis:key".to_string())).await?;

        // try to get a non-existent token
        let res_nonexist = client.get(&TestContentKey("redis:key".to_string())).await?;
        assert!(res_nonexist.is_none());

        Ok(())
    }
}
