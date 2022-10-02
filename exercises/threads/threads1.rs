// threads1.rs
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a hint.
// This program should wait until all the spawned threads have finished before exiting.

use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    // spawn all the threads
    let handles: Vec<JoinHandle<()>> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(250));
                println!("thread {} is complete", i);
            })
        })
        .collect();

    // wait for the threads to finish
    let completed_threads = handles.into_iter().map(|h| h.join()).count();

    if completed_threads != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }
}
