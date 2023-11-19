use rand::{distributions::Alphanumeric, Rng};
use sha2::{Sha256, Digest};
use std::{thread, time};

fn main() {
    let mut current_string = generate_random_string(10);
    let shared_secret = "your_shared_secret";

    loop {
        println!("Current string: {}", current_string);
        
        current_string = evolve_string(&current_string, shared_secret);
        thread::sleep(time::Duration::new(10, 0));
    }
}

fn generate_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn evolve_string(current: &str, secret: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(current);
    hasher.update(secret);
    format!("{:x}", hasher.finalize())
}
