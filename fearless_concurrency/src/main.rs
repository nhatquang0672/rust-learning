use std::{sync::Mutex, thread, rc::Rc};

fn main() {
    let counter = Rc::new(Mutex::new(5));
    let mut handles = vec![];
    for _ in 1..10 {
        let rc_counter = Rc::clone(&counter);
        let spawn = thread::spawn(move || {
            let mut num = rc_counter.lock().unwrap();
            *num = 6;
        });
        handles.push(spawn);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}