> *If you get one percent better each day for one year, you'll end up thirty-seven times better by the time you're done* &nbsp; - James Clear
#
my journey into software engineering began in 2021 with a backend development bootcamp. Since then, I've work across multiple roles including platform/infra teams using AWS, integration and deployment automation for edge devices in a systems role, and backend development for a TypeScript codebase. 

most recently, I've been driven to grow as an engineer by exploring some concepts of distributed systems with a particular focus on 🦀 Rust.

for a quick overview, I've added some key highlights of my recent (and unfinished) experiments below...

##### [Concurrent Network Server](https://github.com/JuneSYi/concurrent-network-server)
this was aimed to do an initial deep dive into sync and async network programming. I focused on understanding various concurrency models and their practical implications.
*   high-level goals for this project -> implementing server patterns and evolving (re-writing) them to improve concurrency: single-threaded, thread-per-client, thread-pool, async/await.
*   few example concepts I explored -> TCP networking, Rust's ownership, borrowing, error-handling, modular design, familiarization with [`tokio` crate](https://crates.io/crates/tokio) for async i/o and the `spawn_blocking` api within tokio for offloading CPU-intensive tasks.

##### [KVDB](https://github.com/JuneSYi/kvdb)
I initially wanted to work on something related to the raft consensus algorithm but pivoted to this project on piecing together a simple key-value store to better understand persistence and custom protocols.
*   high-level goal for this -> persistent key-value store with client-server architecture, building a custom protocol
*   some example concepts covered -> client-server communication, write-ahead log (WAL) with log compaction, in-memory indexing for on-disk data, custom synchronous network protocol. familiarization with [`sled` crate](https://crates.io/crates/sled) (a transactional embedded database).