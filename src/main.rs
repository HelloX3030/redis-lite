mod database;
mod store;

use database::Database;
use store::{Store, store_all};

fn main() {
    let mut database: Database = Database::new();
    database.set("Spaten".to_string(), "Tool".to_string());
    database.set("ABC".to_string(), "Alphabet".to_string());
    database.set("XXXXXXXX".to_string(), "Interesting".to_string());
    database.set("Spaten".to_string(), "almighty god".to_string());
    if let Some(spaten) = database.get("Spaten") {
        println!("Der Wahre Spaten: {}", spaten);
    }
    database.delete("ABC");
    match database.get("ABC") {
        Some(abc) => {
            println!("ABC: {}", abc);
        }
        None => {
            println!("No ABC found!!!!!!");
        }
    }
    database.print();
    let mut database2: Database = Database::new();
    database2.set("Spaten".to_string(), "Of Course its Spaten Again!".to_string());
    let refs: [&dyn Store; 2] = [&database, &database2];
    store_all(&refs);
}
