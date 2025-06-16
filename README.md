# kvdb

* project 1
  * key/value store setup using clap crate
* p2
  * used a simpilified version of the storage algorithm used by bitcask
  * WAL (write-ahead log) on disk that is evaluated on startup to re-create the state of the database in memory
  * storing only the keys in memory, along with offets into the on-disk log
  * log compaction so that it does not grow indefinitely

Credit:
lessons and taken from https://github.com/pingcap/talent-plan