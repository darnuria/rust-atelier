// -----------------------------------------------------------------------------
// Atelier Rust
// 2019-11-17 - 14h -> 16h
// Animateur: Axel Viala (darnuria)
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
// completer soit-même c'est normal que Rust indique une erreur! :)
// -----------------------------------------------------------------------------

use std::thread;

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

    fn increment(&self) {
        self.counter += 1;
    }
}

fn main() {
    let mut counter = SharedCounter::new();
    let mut threads = Vec::new();

    println!("Main: create 3 threads!");
    for _ in 0..3 {
        let thread = thread::spawn(|| {
            let tid = thread::current().id();
            println!("Thread <{:?}>: I'am ready.", tid);
            for _ in 0..10000 {
                counter.increment();
            }
            println!("Thread <{:?}>: I'am done.", tid);
        });
        threads.push(thread);
    }

    for t in threads {
        t.join().expect("Unable to join!");
    }

    println!("Counter end of program: {}.", counter.value())
}