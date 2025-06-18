# kvdb

<i>for personal development</i>

* feat: basic kvs using clap crate for cli parsing
  - key/value store setup using clap crate
* feat: on-disk kv store using WAL
  - WAL (write-ahead log) on disk that is evaluated on startup to re-create the state of the database in memory
  - storing only the keys in memory, along with offets into the on-disk log

Credit:
built through TDD with tests/lessons sourced from https://github.com/pingcap/talent-plan