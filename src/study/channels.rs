use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 30;

#[test]
fn test() {
    let (tx, tr): (Sender<i32>, Receiver<i32>) = mpsc::channel();


    let mut children = Vec::new();
    for id in 0..NTHREADS {
        let thread_tx = tx.clone();
        let child = thread::spawn(move || {
            thread_tx.send(id);
            println!("thread {} finished", id);
        });
        children.push(child);
    }
    for child in children {
        child.join().unwrap();
    }
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for i in 0..NTHREADS {
        println!("push {}", i);
        ids.push(tr.recv());
    }

    println!("{:?}", ids);
}