// a Mutex gives you exclusive access to a value

use std::{sync::RwLock, thread};

// a RwLock gives any number of readers or one writer at a time
fn main() {
    let x = RwLock::new(0);

    thread::scope(|scope| {
        for _ in 0..10 {
            scope.spawn(|| {
                let num = x.read().unwrap();
                println!("num: {}", *num);
            });
        }

        scope.spawn(|| {
            let mut num = x.write().unwrap();
            *num += 1;
        });
    });

    // the RwLock is prone to deadlocks if multiple .read() calls occur in the same thread
    // [EXAMPLE]
    thread::scope(|scope| {
        // thread1
        scope.spawn(|| {
            let num = x.read().unwrap(); // 1
            let num2 = x.read().unwrap(); // 3
            println!("num: {}, num2: {}", *num, *num2);
        });
        // thread2
        scope.spawn(|| {
            let mut num = x.write().unwrap(); // 2
            *num += 1;
        });
    });
    // if the order of execution is 1,2,3 then the program may deadlock
    // since there is a queue system: if a write is requested, some read requests may be blocked to avoid the write from starving -> but since the first read lock isn't released until it goes out of scope, the second read lock will be blocked indefinitely
}
