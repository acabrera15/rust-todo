use rusqlite::{Connection, Result};

fn main() {
    connection();
    println!("testing");
}

fn connection() -> Result<()> {
    let conn = Connection::open("todo.db")?;

    conn.execute(
        "CREATE TABLE todo (
              id INTEGER PRIMARY KEY,
              task VARCHAR(256) NOT NULL,
              completed BOOLEAN NOT NULL DEFAULT FALSE
        )",
        []
       )?;

    Ok(())
}
