
use std::hash::Hash;

use dashmap::DashMap;
use canary::{prelude::*, routes::Route, Serialize};
use serde::de::DeserializeOwned;
use srpc::prelude::*;

const ID: &'static str = "sail";

#[srpc::rpc]
pub struct SailDB<K, V> {
    map: DashMap<K, V>,
}

impl<K: Eq + Hash, V> Default for SailDB<K, V> {
    fn default() -> Self {
        Self { map: DashMap::new() }
    }
}

#[srpc::rpc(none)]
impl<K: Hash + Eq, V> SailDB<K, V> {
    #[server_manual]
    async fn get(&self, mut c: Channel) -> Res<Channel> {
        let key = c.receive().await?;
        let val = self.map.get(&key)
            .and_then(|s| Some(s.value()));
        c.send(val).await?;
        Ok(c)
    }
    #[client_manual]
    async fn get(c: &mut Channel, k: impl std::borrow::Borrow<K>) -> Res<Option<V>> {
        c.send(k.borrow()).await?;
        let res = c.receive().await?;
        Ok(res)
    }
    #[server_manual]
    async fn insert(&self, mut c: Channel) -> Res<Channel> {
        let (key, val) = c.receive().await?;
        let res = self.map.insert(key, val);
        c.send(res).await?;
        Ok(c)
    }
    #[client_manual]
    async fn insert(c: &mut Channel, k: impl std::borrow::Borrow<K>, v: impl std::borrow::Borrow<V>) -> Res<Option<V>> {
        c.send((k.borrow(), v.borrow())).await?;
        let res = c.receive().await?;
        Ok(res)
    }

    #[server_manual]
    async fn remove(&self, mut c: Channel) -> Res<Channel> {
        let key: K = c.receive().await?;
        let res = self.map.remove(&key);
        c.send(res).await?;
        Ok(c)
    }
    #[client_manual]
    async fn remove(c: &mut Channel, k: impl std::borrow::Borrow<K>) -> Res<Option<(K, V)>> {
        c.send(k.borrow()).await?;
        let res = c.receive().await?;
        Ok(res)
    }
}

impl<K, V> SailDB<K, V>
where
    K: Hash + Eq + Serialize + DeserializeOwned,
    V: Serialize + DeserializeOwned
{
    pub fn bind() -> Res<()>
    where
        K: Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        Self::bind_at(ID)
    }
    pub fn bind_at(at: &str) -> Res<()>
    where
        K: Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        Self::bind_in(&GLOBAL_ROUTE, at)
    }
    pub fn bind_in(route: &Route, at: &str) -> Res<()>
    where
        K: Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        route.add_service_default_at::<Self>(at)
    }
}

pub struct Sail<K, V> {
    db: SailDBPeer<K, V>,
}

impl<K: Hash + Eq, V> Sail<K, V> {
    pub fn from_channel(chan: Channel) -> Self {
        Sail { db: chan.client::<SailDB<K, V>>() }
    }
    pub async fn new_at(addr: &ServiceAddr) -> Res<Self> {
        let chan = addr.connect().await?;
        Ok(Self::from_channel(chan))
    }
    pub async fn new(addr: Addr) -> Res<Self> {
        let addr = addr.service(ID);
        Self::new_at(&addr).await
    }
}

impl <K, V> std::ops::Deref for Sail<K, V> {
    type Target = SailDBPeer<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

impl<K, V> std::ops::DerefMut for Sail<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.db
    }
}
