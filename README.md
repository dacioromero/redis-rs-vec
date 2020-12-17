# redis-rs-vec

Demonstrates an issue with [redis::Commands::get](https://docs.rs/redis/0.18.0/redis/trait.Commands.html#method.get) where the following statement is not not correct:

> Get the value of a key. If key is a vec this becomes an MGET.

If a 1-length `Vec` is passed to `connection.get`, it uses `GET` instead of `MGET`  which panics when serializing the result into another `Vec`.
