// https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/dining-philosophers.html

use std::thread;
use std::time::Duration;

struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} is eating", self.name);
        thread::sleep(Duration::from_millis(1000));
        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let names = [
        "Judith Butler",
        "Gilles Deleuze",
        "Karl Marx",
        "Emma Goldman",
        "Michel Foucault",
    ];

    let philosophers = names.iter().map(|&n| Philosopher::new(&n));

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            thread::spawn(move || {
                p.eat();
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
