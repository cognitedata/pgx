//! This exists just to make sure we can compile various things under `#![no_std]`

#![no_std]
extern crate alloc;

use pgx::*;
use serde::{Deserialize, Serialize};

use alloc::string::String;

pg_module_magic!();

/// standard Rust equality/comparison derives
#[derive(Eq, PartialEq, Ord, Hash, PartialOrd)]

/// Support using this struct as a Postgres type, which the easy way requires Serde
#[derive(PostgresType, Serialize, Deserialize)]

/// automatically generate =, <> SQL operator functions
#[derive(PostgresEq)]

/// automatically generate <, >, <=, >=, and a "_cmp" SQL functions
/// When "PostgresEq" is also derived, pgx also creates an "opclass" (and family)
/// so that the type can be used in indexes `USING btree`
#[derive(PostgresOrd)]

/// automatically generate a "_hash" function, and the necessary "opclass" (and family)
/// so the type can also be used in indexes `USING hash`
#[derive(PostgresHash)]
pub struct Thing(String);

#[derive(PostgresType, Serialize, Deserialize, Eq, PartialEq)]
pub struct MyType {
    value: i32,
}

#[pg_operator]
#[opname(=)]
fn my_eq(left: MyType, right: MyType) -> bool {
    left == right
}

#[pg_extern]
fn hello_nostd() -> &'static str {
    "Hello, nostd"
}

#[pg_extern]
fn echo(input: String) -> String {
    input
}
