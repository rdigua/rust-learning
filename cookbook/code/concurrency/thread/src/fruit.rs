//Declare global state using lazy_static. lazy_static creates a globally available static ref which requires a Mutex to allow mutation (also see RwLock). The Mutex wrap ensures the state cannot be simultaneously accessed by multiple threads, preventing race conditions. A MutexGuard must be acquired to read or mutate the value stored in a Mutex.
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

error_chain!{ }

lazy_static! {
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn insert(fruit: &str) -> Result<()> {
    let mut db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;
    db.push(fruit.to_string());
    Ok(())
}

fn main() -> Result<()> {
    insert("apple")?;
    insert("orange")?;
    insert("peach")?;
    {
        let db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;

        db.iter().enumerate().for_each(|(i, item)| println!("{}: {}", i, item));
    }
    insert("grape")?;
    Ok(())
}

