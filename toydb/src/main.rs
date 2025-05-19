use crate::database::Database;

mod persistence;
mod database;

fn main() {

    let db = Database::new("test.db".to_string());
}
