> *If you get one percent better each day for one year, you'll end up thirty-seven times better by the time you're done* &nbsp; - James Clear

My journey into software engineering began in 2021 with a backend development bootcamp. Since then, I've work across multiple roles including platform/infra teams using AWS, integration and deployment automation for edge devices in a systems role, and backend development for a TypeScript codebase. 

Most recently, I've been driven to grow as an engineer by exploring some concepts of distributed systems with a particular focus on 🦀 Rust. Some of the latest personal projects (though unfinished) can be found at [concurrent-network-server](https://github.com/JuneSYi/concurrent-network-server) and [kvdb](https://github.com/JuneSYi/kvdb).

For a quick overview, I've added some key highlights below...

### Recent Personal Projects & Learning Milestones

#### Concurrent Network Server (Rust)
This project was my initial deep dive into synchronous and asynchronous network programming. I focused on understanding various concurrency models and their practical implications in Rust.
*   **Objective**: Re-implementing concurrent server patterns (single-threaded, thread-per-client, thread-pool, async/await) in idiomatic Rust.
*   **Key Learnings**: Deepened understanding of TCP networking, Rust's ownership, borrowing, error-handling, and modular design. Explored `tokio` for asynchronous I/O and `spawn_blocking` for offloading CPU-intensive tasks.
*   **Skills Demonstrated**: Concurrency patterns, network programming, Rust ecosystem, performance optimization, refactoring for separation of concerns.

---

#### KVDB (Rust)
Building on the networking fundamentals, this project challenged me to design and implement a robust data storage solution from the ground up, including persistence and a custom protocol.
*   **Objective**: Building a persistent key-value store with client-server architecture, custom protocol, and pluggable storage engines.
*   **Key Learnings**: Implemented a Write-Ahead Log (WAL) with log compaction, managed in-memory indexing for on-disk data, and designed a custom synchronous network protocol. Explored Rust traits for pluggable storage backends (custom `KvStore` vs. `sled`).
*   **Skills Demonstrated**: Data persistence, file I/O, custom protocol design, client-server communication, trait-based design, benchmarking, error handling across network boundaries.