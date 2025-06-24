# kvdb

<i>for personal growth & development</i>

### creating an in-memory key/value store accessible through command-line arguments
* `feat: basic kvs using clap crate for cli parsing`
  - key/value store setup using clap crate

### converting the in-memory kv store to a persistent kv store
* `feat: on-disk kv store using WAL`
  - WAL (write-ahead log) on disk that is evaluated on startup to re-create the state of the database in memory
  - storing only the keys in memory, along with offets into the on-disk log
* `feat: storing log pointers in the index`
  - removed storage of values on in-memory hashmap
  - in-memory hashmap stores position of offset within log database (log pointer)
* `feat: log compaction to prevent unbounded log growth`
  - previous append-only design caused the log file to grow indefinitely, storing redundant data from overwritten keys and removals.
  - this changes involves a counter (`stale_log_ct`) to track the number of stale records and an automatic trigger that runs compaction after 10 stale writes.

### converting the persistent kv store to a single-threaded kv server and kv client with synchronous networking over a custom protocol
* `feat: separation of cli binary crates for server and client`

Credit:
built through TDD with tests/lessons sourced from https://github.com/pingcap/talent-plan