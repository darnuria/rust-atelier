// -----------------------------------------------------------------------------
// Atelier Rust
// 2019-11-17 - 14h -> 16h
// Animateur: Axel (darnuria) && Aurelia
//
// 10-shared-counter.rs
//
// Objectifs pédagogiques:
//
// - Multi-threading les bases
// - Mécanismes de partages
// - move/borrow et threads
//
// /!\ Quand vous verrez les symboles: `???`, il s'agit de code à
// compléter soi-même c'est normal que Rust indique une erreur! :)
// -----------------------------------------------------------------------------

use std::thread;
use std::sync::{Arc, Mutex};

struct SharedCounter {
    counter: i32,
}

impl SharedCounter {
    fn new() -> SharedCounter {
        SharedCounter { counter: 0 }
    }

    #[inline]
    fn value(&self) -> i32 {
        self.counter
    }

    fn increment(&mut self) {
        self.counter += 1;
    }
}

fn main() {
    let counter = SharedCounter::new();
    
    let mut threads = Vec::new();

    println!("Main: create 3 threads!");
    for i in 0..3 {
        // let counter = Arc::clone(&counter);
        println!("Arthas (main): Thread {} I raise you from the grave.", i + 2);
        let thread = thread::spawn(move || {
            let tid = thread::current().id();
            println!("Thread <{:?}>: I'am ready.", tid);
            for _ in 0..10000 {
                // let mut counter = counter
                //     .lock()
                //     .expect("Whops cannot acquire lock.");
                counter.increment();
            }
            println!("Thread <{:?}>: I'am done.", tid);
        });
        threads.push(thread);
    }

    for t in threads {
        t.join().expect("Unable to join!");
    }

    // let counter = counter.lock().expect("It is fine.");
    println!("Counter end of program: {}.", counter.value())
}