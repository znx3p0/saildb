# SailDB

SailDB is an in-memory database based on Canary and SRPC.

SailDB is extremely performant since communications are based on top of Canary, and
the key-value store is backed by Dashmap.

SailDB is generic over the keys and values, and can be constructed at runtime, and
is designed to be as simple to use as possible.

Creating a SailDB database is extremely simple:
```rust
// bind the global route to this socket
Tcp::bind("127.0.0.1:8080").await?;
// bind a SailDB to the global route with the default id
Sail::<String, String>::bind();
```

Accessing the database should be as simple as this
```rust
let addr = "127.0.0.1:8080".parse::<Addr>()?;
let mut db: Sail<String, String> = Sail::new(addr).await?;

let (key, val) = ("hello".to_string(), "world!".to_string());

db.insert(&key, &val).await?;

let val = db.get(&key).await?;
assert!(val, Some("world!".to_string()))
```

[A simple example of SailDB](https://github.com/znx3p0/saildb_test)
