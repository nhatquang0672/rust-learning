use std::{thread::{Thread, self}, time::Duration};

fn thread_spawn() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hello from {} spawn thread", i);
            thread::sleep(Duration::from_millis(1));
        } 
    });
    for i in 1..5 {
        println!("Hello from {} main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}
