use std::hash::Hash;
use std::sync::Arc;

use dashmap::DashMap;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sia::providers::ServiceAddr;
use sia::routes::{Route, GLOBAL_ROUTE};
use sia::{Addr, Channel, Result};
use srpc::IntoClient;

const ID: &'static str = "sail";

#[srpc::rpc]
pub struct InnerSailDB<K: Hash + Eq, V> {
    map: DashMap<K, V>,
}

impl<K: Hash + Eq, V> InnerSailDB<K, V> {
    pub fn new() -> Self {
        InnerSailDB {
            map: DashMap::new(),
        }
    }
}

#[srpc::rpc(none)]
impl<K: Hash + Eq, V> InnerSailDB<K, V> {
    #[manual]
    pub async fn get(&self, mut chan: Channel) -> Result<Channel> {
        let key: K = chan.receive().await?;
        let val = match self.map.get(&key) {
            Some(val) => {
                let val = val.value();
                Some(val)
            }
            None => None,
        };
        chan.send(val).await?;
        Ok(chan)
    }
    #[manual]
    pub async fn insert(&self, mut chan: Channel) -> Result<Channel> {
        let (key, value) = chan.receive().await?;
        let value = self.map.insert(key, value);
        chan.send(value).await?;
        Ok(chan)
    }
    #[manual]
    pub async fn remove(&self, mut chan: Channel) -> Result<Channel> {
        let key: K = chan.receive().await?;
        let value = self.map.remove(&key).and_then(|s| Some(s.1));
        chan.send(value).await?;
        Ok(chan)
    }
}

pub struct Sail<K: Hash + Eq, V> {
    db: Option<InnerSailDBPeer<K, V>>,
}

impl<K, V> Sail<K, V>
where
    K: Hash + Eq + Serialize + DeserializeOwned + Sync + Send + 'static,
    V: Serialize + DeserializeOwned + Sync + Send + 'static,
{
    pub fn bind() -> Result<()>
    where
        K: Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        Self::bind_at(ID)
    }
    /// binds in global route
    pub fn bind_at(at: &str) -> Result<()>
    where
        K: Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        Self::bind_in(&GLOBAL_ROUTE, at)
    }
    pub fn bind_in(route: &Route, at: &str) -> Result<()>
    where
        K: Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        route.add_service_at::<InnerSailDB<K, V>>(at, Arc::new(InnerSailDB::new()))
    }
    pub async fn new(addr: Addr) -> Result<Self> {
        let addr = addr.service(ID);
        Self::new_at(addr).await
    }
    pub async fn new_at(addr: ServiceAddr) -> Result<Self> {
        let db = addr.connect().await?.client::<InnerSailDB<K, V>>();
        Ok(Sail { db: Some(db) })
    }
    pub async fn get(&mut self, k: &K) -> Result<Option<V>> {
        let db = self.db.take().unwrap();
        let mut chan: Channel = db.get().await?;
        chan.send(k).await?;
        let val: Option<V> = chan.receive().await?;
        self.db = Some(chan.client::<InnerSailDB<K, V>>());
        Ok(val)
    }
    pub async fn insert(&mut self, k: &K, v: &V) -> Result<Option<V>> {
        let db = self.db.take().unwrap();
        let mut chan = db.insert().await?;
        chan.send((k, v)).await?;
        let val = chan.receive().await?;
        self.db = Some(chan.client::<InnerSailDB<K, V>>());
        Ok(val)
    }
    pub async fn remove(&mut self, k: &K) -> Result<Option<V>> {
        let db = self.db.take().unwrap();
        let mut chan = db.remove().await?;
        chan.send(k).await?;
        let val = chan.receive().await?;
        self.db = Some(chan.client::<InnerSailDB<K, V>>());
        Ok(val)
    }
}

