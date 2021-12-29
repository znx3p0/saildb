#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::hash::Hash;
use std::sync::Arc;
use dashmap::DashMap;
use serde::de::DeserializeOwned;
use serde::Serialize;
use canary::providers::ServiceAddr;
use canary::routes::{Route, GLOBAL_ROUTE};
use canary::{Addr, Channel, Result};
use srpc::IntoClient;
const ID: &'static str = "sail";
pub struct InnerSailDB<K: Hash + Eq, V> {
    map: DashMap<K, V>,
}
impl<K: Hash + Eq, V> ::srpc::canary::routes::RegisterEndpoint for InnerSailDB<K, V> {
    const ENDPOINT: &'static str = "inner_sail_db";
}
pub struct InnerSailDBPeer<K, V>(
    pub ::srpc::canary::Channel,
    ::core::marker::PhantomData<(K, V)>,
);
impl<K: Hash + Eq, V> From<::srpc::canary::Channel> for InnerSailDBPeer<K, V> {
    fn from(c: ::srpc::canary::Channel) -> Self {
        InnerSailDBPeer(c, ::core::marker::PhantomData::default())
    }
}
impl<K: Hash + Eq, V> ::srpc::Peer for InnerSailDB<K, V> {
    type Struct = InnerSailDBPeer<K, V>;
}
impl<K: Hash + Eq, V> InnerSailDB<K, V> {
    pub fn new() -> Self {
        InnerSailDB {
            map: DashMap::new(),
        }
    }
}
const _: () = {
    impl<
            K: Hash + Eq + ::srpc::__private::Serialize + ::srpc::__private::DeserializeOwned,
            V: ::srpc::__private::Serialize + ::srpc::__private::DeserializeOwned,
        > InnerSailDB<K, V>
    {
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
        pub async fn insert(&self, mut chan: Channel) -> Result<Channel> {
            let (key, value) = chan.receive().await?;
            let value = self.map.insert(key, value);
            chan.send(value).await?;
            Ok(chan)
        }
        pub async fn remove(&self, mut chan: Channel) -> Result<Channel> {
            let key: K = chan.receive().await?;
            let value = self.map.remove(&key).and_then(|s| Some(s.1));
            chan.send(value).await?;
            Ok(chan)
        }
    }
    #[allow(non_camel_case_types)]
    #[repr(u8)]
    enum __srpc_action {
        get,
        insert,
        remove,
    }
    impl serde::Serialize for __srpc_action {
        #[allow(clippy::use_self)]
        fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let value: u8 = match *self {
                __srpc_action::get => __srpc_action::get as u8,
                __srpc_action::insert => __srpc_action::insert as u8,
                __srpc_action::remove => __srpc_action::remove as u8,
            };
            serde::Serialize::serialize(&value, serializer)
        }
    }
    impl<'de> serde::Deserialize<'de> for __srpc_action {
        #[allow(clippy::use_self)]
        fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct discriminant;
            #[allow(non_upper_case_globals)]
            impl discriminant {
                const get: u8 = __srpc_action::get as u8;
                const insert: u8 = __srpc_action::insert as u8;
                const remove: u8 = __srpc_action::remove as u8;
            }
            match <u8 as serde::Deserialize>::deserialize(deserializer)? {
                discriminant::get => core::result::Result::Ok(__srpc_action::get),
                discriminant::insert => core::result::Result::Ok(__srpc_action::insert),
                discriminant::remove => core::result::Result::Ok(__srpc_action::remove),
                other => core::result::Result::Err(serde::de::Error::custom(
                    ::core::fmt::Arguments::new_v1(
                        &["invalid value: ", ", expected one of: ", ", ", ", "],
                        &match (
                            &other,
                            &discriminant::get,
                            &discriminant::insert,
                            &discriminant::remove,
                        ) {
                            _args => [
                                ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(_args.2, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(_args.3, ::core::fmt::Display::fmt),
                            ],
                        },
                    ),
                )),
            }
        }
    }
    impl<
            K: Hash
                + Eq
                + Send
                + Sync
                + 'static
                + ::srpc::__private::Serialize
                + ::srpc::__private::DeserializeOwned,
            V: Send
                + Sync
                + 'static
                + ::srpc::__private::Serialize
                + ::srpc::__private::DeserializeOwned,
        > ::srpc::canary::service::Service for InnerSailDB<K, V>
    {
        const ENDPOINT: &'static str = "inner_sail_db";
        type Pipeline = ();
        type Meta = ::std::sync::Arc<InnerSailDB<K, V>>;
        fn service(
            __srpc_inner_meta: ::std::sync::Arc<InnerSailDB<K, V>>,
        ) -> Box<dyn Fn(::srpc::canary::igcp::BareChannel) + Send + Sync + 'static> {
            ::canary::service::run_metadata(
                __srpc_inner_meta,
                |__srpc_inner_meta: ::std::sync::Arc<InnerSailDB<K, V>>,
                 mut __srpc_inner_channel: ::srpc::canary::Channel| async move {
                    loop {
                        match __srpc_inner_channel.receive::<__srpc_action>().await? {
                            __srpc_action::get => {
                                match __srpc_inner_meta.get(__srpc_inner_channel).await {
                                    Ok(chan) => __srpc_inner_channel = chan,
                                    Err(e) => return Err(e),
                                }
                            }
                            __srpc_action::insert => {
                                match __srpc_inner_meta.insert(__srpc_inner_channel).await {
                                    Ok(chan) => __srpc_inner_channel = chan,
                                    Err(e) => return Err(e),
                                }
                            }
                            __srpc_action::remove => {
                                match __srpc_inner_meta.remove(__srpc_inner_channel).await {
                                    Ok(chan) => __srpc_inner_channel = chan,
                                    Err(e) => return Err(e),
                                }
                            }
                        }
                    }
                },
            )
        }
    }
    impl<
            K: Hash
                + Eq
                + Send
                + Sync
                + 'static
                + ::srpc::__private::Serialize
                + ::srpc::__private::DeserializeOwned,
            V: Send
                + Sync
                + 'static
                + ::srpc::__private::Serialize
                + ::srpc::__private::DeserializeOwned,
        > ::srpc::canary::service::StaticService for InnerSailDB<K, V>
    {
        type Meta = ::std::sync::Arc<InnerSailDB<K, V>>;
        type Chan = ::srpc::canary::Channel;
        fn introduce(
            __srpc_inner_meta: ::std::sync::Arc<InnerSailDB<K, V>>,
            mut __srpc_inner_channel: ::srpc::canary::Channel,
        ) -> ::srpc::canary::runtime::JoinHandle<::srpc::canary::Result<()>> {
            ::srpc::canary::runtime::spawn(async move {
                loop {
                    match __srpc_inner_channel.receive::<__srpc_action>().await? {
                        __srpc_action::get => {
                            match __srpc_inner_meta.get(__srpc_inner_channel).await {
                                Ok(chan) => __srpc_inner_channel = chan,
                                Err(e) => return Err(e),
                            }
                        }
                        __srpc_action::insert => {
                            match __srpc_inner_meta.insert(__srpc_inner_channel).await {
                                Ok(chan) => __srpc_inner_channel = chan,
                                Err(e) => return Err(e),
                            }
                        }
                        __srpc_action::remove => {
                            match __srpc_inner_meta.remove(__srpc_inner_channel).await {
                                Ok(chan) => __srpc_inner_channel = chan,
                                Err(e) => return Err(e),
                            }
                        }
                    }
                }
            })
        }
    }
    impl<
            K: Hash
                + Eq
                + Send
                + Sync
                + 'static
                + ::srpc::__private::Serialize
                + ::srpc::__private::DeserializeOwned,
            V: Send
                + Sync
                + 'static
                + ::srpc::__private::Serialize
                + ::srpc::__private::DeserializeOwned,
        > InnerSailDBPeer<K, V>
    {
        pub async fn get(mut self) -> ::srpc::canary::Result<::srpc::canary::Channel> {
            self.0.send(__srpc_action::get).await?;
            Ok(self.0)
        }
        pub async fn insert(mut self) -> ::srpc::canary::Result<::srpc::canary::Channel> {
            self.0.send(__srpc_action::insert).await?;
            Ok(self.0)
        }
        pub async fn remove(mut self) -> ::srpc::canary::Result<::srpc::canary::Channel> {
            self.0.send(__srpc_action::remove).await?;
            Ok(self.0)
        }
    }
};
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
struct DistributedList<T> {
    list: Vec<T>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::default::Default> ::core::default::Default for DistributedList<T> {
    #[inline]
    fn default() -> DistributedList<T> {
        DistributedList {
            list: ::core::default::Default::default(),
        }
    }
}
impl<T> ::srpc::canary::routes::RegisterEndpoint for DistributedList<T> {
    const ENDPOINT: &'static str = "distributed_list";
}
struct DistributedListPeer<T>(pub ::srpc::canary::Channel, ::core::marker::PhantomData<(T)>);
impl<T> From<::srpc::canary::Channel> for DistributedListPeer<T> {
    fn from(c: ::srpc::canary::Channel) -> Self {
        DistributedListPeer(c, ::core::marker::PhantomData::default())
    }
}
impl<T> ::srpc::Peer for DistributedList<T> {
    type Struct = DistributedListPeer<T>;
}
const _: () = {
    impl<T: Clone + ::srpc::__private::Serialize + ::srpc::__private::DeserializeOwned>
        DistributedList<T>
    {
        async fn push(&mut self, value: T) {
            self.list.push(value);
        }
        async fn get(&self, index: usize) -> Option<T> {
            self.list.get(index).and_then(|val| Some(val.clone()))
        }
        async fn remove(&mut self, index: usize) -> T {
            self.list.remove(index)
        }
    }
    #[allow(non_camel_case_types)]
    #[repr(u8)]
    enum __srpc_action {
        get,
        push,
        remove,
    }
    impl serde::Serialize for __srpc_action {
        #[allow(clippy::use_self)]
        fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let value: u8 = match *self {
                __srpc_action::get => __srpc_action::get as u8,
                __srpc_action::push => __srpc_action::push as u8,
                __srpc_action::remove => __srpc_action::remove as u8,
            };
            serde::Serialize::serialize(&value, serializer)
        }
    }
    impl<'de> serde::Deserialize<'de> for __srpc_action {
        #[allow(clippy::use_self)]
        fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct discriminant;
            #[allow(non_upper_case_globals)]
            impl discriminant {
                const get: u8 = __srpc_action::get as u8;
                const push: u8 = __srpc_action::push as u8;
                const remove: u8 = __srpc_action::remove as u8;
            }
            match <u8 as serde::Deserialize>::deserialize(deserializer)? {
                discriminant::get => core::result::Result::Ok(__srpc_action::get),
                discriminant::push => core::result::Result::Ok(__srpc_action::push),
                discriminant::remove => core::result::Result::Ok(__srpc_action::remove),
                other => core::result::Result::Err(serde::de::Error::custom(
                    ::core::fmt::Arguments::new_v1(
                        &["invalid value: ", ", expected one of: ", ", ", ", "],
                        &match (
                            &other,
                            &discriminant::get,
                            &discriminant::push,
                            &discriminant::remove,
                        ) {
                            _args => [
                                ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(_args.2, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(_args.3, ::core::fmt::Display::fmt),
                            ],
                        },
                    ),
                )),
            }
        }
    }
    impl<
            T: Clone
                + Send
                + Sync
                + 'static
                + ::srpc::__private::Serialize
                + ::srpc::__private::DeserializeOwned,
        > ::srpc::canary::service::Service for DistributedList<T>
    {
        const ENDPOINT: &'static str = "distributed_list";
        type Pipeline = ();
        type Meta = ::std::sync::Arc<::srpc::RwLock<DistributedList<T>>>;
        fn service(
            __srpc_inner_meta: ::std::sync::Arc<::srpc::RwLock<DistributedList<T>>>,
        ) -> Box<dyn Fn(::srpc::canary::igcp::BareChannel) + Send + Sync + 'static> {
            ::canary::service::run_metadata(
                __srpc_inner_meta,
                |__srpc_inner_meta: ::std::sync::Arc<::srpc::RwLock<DistributedList<T>>>,
                 mut __srpc_inner_channel: ::srpc::canary::Channel| async move {
                    loop {
                        match __srpc_inner_channel.receive::<__srpc_action>().await? {
                            __srpc_action::push => {
                                #[allow(unused_parens)]
                                let (value): (T) = __srpc_inner_channel.receive().await?;
                                __srpc_inner_meta.write().await.push(value).await;
                            }
                            __srpc_action::get => {
                                #[allow(unused_parens)]
                                let (index): (usize) = __srpc_inner_channel.receive().await?;
                                let res = __srpc_inner_meta.read().await.get(index).await;
                                __srpc_inner_channel.send(res).await?;
                            }
                            __srpc_action::remove => {
                                #[allow(unused_parens)]
                                let (index): (usize) = __srpc_inner_channel.receive().await?;
                                let res = __srpc_inner_meta.write().await.remove(index).await;
                                __srpc_inner_channel.send(res).await?;
                            }
                        }
                    }
                },
            )
        }
    }
    impl<
            T: Clone
                + Send
                + Sync
                + 'static
                + ::srpc::__private::Serialize
                + ::srpc::__private::DeserializeOwned,
        > ::srpc::canary::service::StaticService for DistributedList<T>
    {
        type Meta = ::std::sync::Arc<::srpc::RwLock<DistributedList<T>>>;
        type Chan = ::srpc::canary::Channel;
        fn introduce(
            __srpc_inner_meta: ::std::sync::Arc<::srpc::RwLock<DistributedList<T>>>,
            mut __srpc_inner_channel: ::srpc::canary::Channel,
        ) -> ::srpc::canary::runtime::JoinHandle<::srpc::canary::Result<()>> {
            ::srpc::canary::runtime::spawn(async move {
                loop {
                    match __srpc_inner_channel.receive::<__srpc_action>().await? {
                        __srpc_action::push => {
                            #[allow(unused_parens)]
                            let (value): (T) = __srpc_inner_channel.receive().await?;
                            __srpc_inner_meta.write().await.push(value).await;
                        }
                        __srpc_action::get => {
                            #[allow(unused_parens)]
                            let (index): (usize) = __srpc_inner_channel.receive().await?;
                            let res = __srpc_inner_meta.read().await.get(index).await;
                            __srpc_inner_channel.send(res).await?;
                        }
                        __srpc_action::remove => {
                            #[allow(unused_parens)]
                            let (index): (usize) = __srpc_inner_channel.receive().await?;
                            let res = __srpc_inner_meta.write().await.remove(index).await;
                            __srpc_inner_channel.send(res).await?;
                        }
                    }
                }
            })
        }
    }
    impl<
            T: Clone
                + Send
                + Sync
                + 'static
                + ::srpc::__private::Serialize
                + ::srpc::__private::DeserializeOwned,
        > DistributedListPeer<T>
    {
        pub async fn push(&mut self, value: impl AsRef<T>) -> ::srpc::canary::Result<()> {
            self.0.send(__srpc_action::push).await?;
            #[allow(unused_parens)]
            self.0.send((value.as_ref())).await?;
            Ok(())
        }
        pub async fn get(&mut self, index: impl AsRef<usize>) -> ::srpc::canary::Result<Option<T>> {
            self.0.send(__srpc_action::get).await?;
            #[allow(unused_parens)]
            self.0.send((index.as_ref())).await?;
            self.0.receive().await
        }
        pub async fn remove(&mut self, index: impl AsRef<usize>) -> ::srpc::canary::Result<T> {
            self.0.send(__srpc_action::remove).await?;
            #[allow(unused_parens)]
            self.0.send((index.as_ref())).await?;
            self.0.receive().await
        }
    }
};
