# redis-lite
🧭 Roadmap: Build a Redis-lite Key-Value Store in Rust
✅ Phase 1: Core Concepts & Setup
📌 Goals

    Learn how to persist key-value data to disk.

    Understand basic Rust traits and data ownership.

    Use serde for serialization/deserialization.

🛠 Tasks

    Project setup

        Create a new Rust project with cargo new kv-store

        Add dependencies:

    [dependencies]
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
    anyhow = "1.0"

    Later you can swap serde_json with borsh or bincode.

Define a Key-Value Store API

    pub trait KVStore {
        fn set(&mut self, key: String, value: String);
        fn get(&self, key: &str) -> Option<&String>;
        fn delete(&mut self, key: &str);
    }

    Implement In-Memory Store

        Use HashMap<String, String> under the hood.

        Add impl KVStore for InMemoryStore.

    Add File Persistence

        Serialize the map using serde_json.

        Save/load from a file (store.json) on startup and shutdown.

🚀 Phase 2: Serialization & Traits
📌 Goals

    Learn Rust traits in practice.

    Explore serde deeper (derive vs manual impl).

    Prepare for Solana: Serialization is key (Borsh, binary formats).

🛠 Tasks

    Swap JSON with a binary serializer (e.g., bincode or borsh)

bincode = "1.3" # or borsh = "0.10"

Test storing structured data

    #[derive(Serialize, Deserialize)]
    struct Value {
        user_id: u32,
        balance: u64,
    }

    Use traits to generalize serialization

        Implement a DiskBackedStore<T> that saves any serializable type.

🌐 Phase 3 (Optional): Networking (Redis Protocol Lite)
📌 Goals

    Learn basic async networking with tokio.

    Model client/server interaction.

    Handle serialization over the wire.

🛠 Tasks

    Add dependencies

    tokio = { version = "1.0", features = ["full"] }

    Create a basic TCP server

        Accept simple commands: SET key val, GET key, DEL key.

    Respond to commands

        Parse incoming strings.

        Deserialize values and respond.

    Optional: use a custom protocol like RESP (Redis Serialization Protocol).

🎯 Phase 4: Testing & Tooling
📌 Goals

    Ensure robustness and persistence.

    Learn about property-based testing (quickcheck) or fuzzing.

    Prepare for real-world contracts with durable state.

🛠 Tasks

    Unit tests for each store operation

    Test serialization + deserialization integrity

    Simulate crash recovery

        Save state before each op.

        Reload on restart.

🧩 Bonus: CLI Interface

    Use clap to make a command-line client for interacting with your KV store.

    Commands like:

kv-store set mykey myval
kv-store get mykey
