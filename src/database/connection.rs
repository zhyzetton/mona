use rusqlite::Connection;

pub fn open() -> rusqlite::Result<Connection> {
    Connection::open("library.db")
}