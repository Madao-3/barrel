//! Some unit tests that create create tables
#![allow(unused_imports)]

use backend::{Sqlite, SqlGenerator};
use {Migration, Table};

#[test]
fn create_multiple_tables() {
    use Type::*;
    let mut m = Migration::new();
    m.create_table("artist", |t| {
        t.add_column("name", Text);
        t.add_column("description", Text);
        t.add_column("pic", Text);
        t.add_column("mbid", Text);
    });
    m.create_table("album", |t| {
        t.add_column("name", Text);
        t.add_column("pic", Text);
        t.add_column("mbid", Text);
    });
    assert_eq!(m.make::<Sqlite>(), String::from("CREATE TABLE \"artist\" (\"id\" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT  , \"name\" TEXT  , \"description\" TEXT  , \"pic\" TEXT  , \"mbid\" TEXT  );CREATE TABLE \"album\" (\"id\" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT  , \"name\" TEXT  , \"pic\" TEXT  , \"mbid\" TEXT  );"));
}