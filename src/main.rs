mod database;

use database::Store;

fn main() {
    let mut store: Store = Store::new();
    store.set("Spaten".to_string(), "Tool".to_string());
    store.set("ABC".to_string(), "Alphabet".to_string());
    store.set("XXXXXXXX".to_string(), "Interesting".to_string());
    store.set("Spaten".to_string(), "almighty god".to_string());
    if let Some(spaten) = store.get("Spaten") {
        println!("Der Wahre Spaten: {}", spaten);
    }
    store.print();
}
