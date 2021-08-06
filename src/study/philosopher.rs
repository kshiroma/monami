use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use env_logger;
use log::{debug, error, info, warn};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

struct Table {
    forks: Vec<Mutex<()>>,
}


impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name,
            left: left,
            right: right,
        }
    }

    pub fn eat(&self, table: &Table) {
        log::trace!("({}) hold no fork.",line!());
        let _left = table.forks[self.left].lock().unwrap();
        log::trace!("({}) hold left fork.",line!());
        let _right = table.forks[self.right].lock().unwrap();
        log::trace!("({}) hold right fork.",line!());

        log::info!("{},{} is eating.", file!(),self.name);
        thread::sleep(Duration::from_millis(100));
        log::info!("{},{} is done eating",file!(), self.name);
    }
}

#[test]
fn main() {
    use std::env;
    use std::sync::Arc;
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ]
    });

    let philosophers = vec![
        Philosopher::new("1.Judith", 0, 1),
        Philosopher::new("2.Giles", 1, 2),
        Philosopher::new("3.Karl", 2, 3),
        Philosopher::new("4.Emma", 3, 4),
        Philosopher::new("5.Michel", 0, 4),
    ];


    let handles: Vec<_> = philosophers .into_iter().map(|p| {
        let table = table.clone();
        thread::spawn(move || { p.eat(&table); })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}