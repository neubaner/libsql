//! # libSQL API for Rust
//!
//! libSQL is an embeddable SQL database engine based on SQLite.
//! This Rust API is a batteries-included wrapper around the SQLite C API to support transparent replication while retaining compatibility with the SQLite ecosystem, such as the SQL dialect and extensions. If you are building an application in Rust, this is the crate you should use.
//! There are also libSQL language bindings of this Rust crate to other languages such as [JavaScript](), Python, Go, and C.
//!
//! ## Getting Started
//!
//! To get started, you first need to create a [`Database`] object and then open a [`Connection`] to it, which you use to query:
//!
//! ```rust,no_run
//! # async fn run() {
//! use libsql::Database;
//!
//! let db = Database::open_in_memory().unwrap();
//! let conn = db.connect().unwrap();
//! conn.execute("CREATE TABLE IF NOT EXISTS users (email TEXT)", ()).await.unwrap();
//! conn.execute("INSERT INTO users (email) VALUES ('alice@example.org')", ()).await.unwrap();
//! # }
//! ```
//!
//! ## Embedded Replicas
//!
//! Embedded replica is libSQL database that's running in your application process, which keeps a local copy of a remote database.
//! They are useful if you want to move data in the memory space of your application for fast access.
//!
//! You can open an embedded read-only replica by using the [`Database::with_replicator`] constructor:
//!
//! ```rust,no_run
//! # async fn run() {
//! use libsql::{Database, Opts};
//! use libsql_replication::{Frame, Frames, Replicator};
//!
//! let mut db = Database::open_with_local_sync("/tmp/test.db").await.unwrap();
//!
//! let frames = Frames::Vec(vec![]);
//! db.sync_frames(frames).unwrap();
//! let conn = db.connect().unwrap();
//! conn.execute("SELECT * FROM users", ()).await.unwrap();
//! # }
//! ```
//!
//! ## Examples
//!
//! You can find more examples in the [`examples`](https://github.com/penberg/libsql-experimental/tree/libsql-api/crates/core/examples) directory.

// Legacy mode, for compatibility with the old libsql API, it is doc hidden so
// that new users do not use this api as its deprecated in favor of the v2 api.
#[cfg(feature = "core")]
mod v1;
#[cfg(feature = "core")]
mod v2;

pub mod errors;
mod box_clone_service;
pub use errors::Error;

#[cfg(all(feature = "core", feature = "replication"))]
pub use v1::database::Opts;

#[cfg(feature = "core")]
pub use v1::{
    params,
    params::{params_from_iter, Value, ValueRef},
    version, version_number, RowsFuture,
};

#[cfg(feature = "core")]
pub use v2::{
    hrana, rows,
    rows::{Row, Rows},
    statement,
    statement::{Column, Statement},
    transaction,
    transaction::{Transaction, TransactionBehavior},
    Connection, Database, OpenFlags,
};

#[cfg(feature = "core")]
pub use libsql_sys::ffi;

pub type Result<T> = std::result::Result<T, errors::Error>;
pub(crate) type BoxError = Box<dyn std::error::Error + Send + Sync>;
