const N_INCREMENTS: usize = 500_000;
const N_THREADS: usize = 50;

pub mod attempt1 {
    // rust does not allow compilation where a race condition is possible
    use std::array;

    use super::*;

    static MY_NUMBER: u64 = 0;

    fn thread() {
        for _ in 0..N_INCREMENTS {
            // MY_NUMBER += 1; // error: Rust compiler warns of race condition
        }
    }

    pub fn main() {
        let mut threads: [_; N_THREADS] = array::from_fn(|_| None);

        for i in 0..N_THREADS {
            threads[i] = Some(std::thread::spawn(thread));
        }
        for i in 0..N_THREADS {
            threads[i].take().unwrap().join();
        }

        println!(
            "Final total: {} (expected {})",
            MY_NUMBER,
            N_INCREMENTS * N_THREADS
        );
    }
}

pub mod attempt2 {
    // move from global variable to mut
    use std::{array, thread::JoinHandle};

    use super::*;

    fn thread(my_number: &mut u64) {
        for _ in 0..N_INCREMENTS {
            *my_number += 1;
        }
    }

    pub fn main() {
        let mut my_number = 0;

        let mut threads: [_; N_THREADS] =
            array::from_fn::<Option<JoinHandle<()>>, N_THREADS, _>(|_| None);

        for i in 0..N_THREADS {
            // threads[i] = Some(std::thread::spawn(|| thread(&mut my_number))); // error
        }
        for i in 0..N_THREADS {
            threads[i].take().unwrap().join();
        }

        println!(
            "Final total: {} (expected {})",
            my_number,
            N_INCREMENTS * N_THREADS
        );
    }
}

// why this error?

// this is the definition for the spawn function in the std::thread module:
pub fn spawn<F, T>(f: F) -> std::thread::JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static, // captures must be `Send` and `'static`
{
    std::thread::spawn(f)
}

// the `Send` trait is a marker trait that indicates that a type is safe to send to another thread.
// it is also an `auto` trait - the compiler will automatically implement it for any type that is safe to send to another thread.
// signature: `pub unsafe auto trait Send`

// the `Sync` trait is a marker trait that indicates that a type is safe to share references to between threads.
// signature: `pub unsafe auto trait Sync`
// a type T is Sync if and only if &T is Send

pub mod attempt3 {
    // try to use a mutex -- but now there's a lifetime issue
    use std::{array, sync::Mutex, thread::JoinHandle};

    use super::*;

    fn thread(my_number: &Mutex<u64>) {
        for _ in 0..N_INCREMENTS {
            // we can't forget to lock the mutex before accessing the data
            // since it is impossible

            // the mutex also automatically unlocks when it goes out of scope
            *my_number.lock().unwrap() += 1;
        }
    }

    pub fn main() {
        // use a mutex
        let mut my_number: Mutex<u64> = Mutex::new(0);

        let mut threads: [_; N_THREADS] =
            array::from_fn::<Option<JoinHandle<()>>, N_THREADS, _>(|_| None);

        for i in 0..N_THREADS {
            // error: still a lifetime issue -- since the thread could outlive the mutex (i.e., main could finish before the threads)
            // threads[i] = Some(std::thread::spawn(|| thread(&my_number))); // error
        }
        for i in 0..N_THREADS {
            threads[i].take().unwrap().join();
        }

        println!(
            "Final total: {} (expected {})",
            my_number.lock().unwrap(),
            N_INCREMENTS * N_THREADS
        );
    }
}

pub mod attempt4 {
    // use Arcs (i.e., reference counting)
    use std::{
        array,
        sync::{Arc, Mutex},
        thread::JoinHandle,
    };

    use super::*;

    fn thread(my_number: Arc<Mutex<u64>>) {
        for _ in 0..N_INCREMENTS {
            // we can't forget to lock the mutex before accessing the data
            // since it is impossible

            // the mutex also automatically unlocks when it goes out of scope
            *my_number.lock().unwrap() += 1;
        }
    }

    pub fn main() {
        // use an Arc (atomic reference counter) to share the mutex between threads
        // when we .clone() the Arc, we are incrementing the reference count
        let mut my_number: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

        let mut threads: [_; N_THREADS] =
            array::from_fn::<Option<JoinHandle<()>>, N_THREADS, _>(|_| None);

        for i in 0..N_THREADS {
            // error: still a lifetime issue -- since the thread could outlive the mutex (i.e., main could finish before the threads)
            let my_number = my_number.clone();
            threads[i] = Some(std::thread::spawn(|| thread(my_number))); // error
        }
        for i in 0..N_THREADS {
            threads[i].take().unwrap().join();
        }

        println!(
            "Final total: {} (expected {})",
            my_number.lock().unwrap(),
            N_INCREMENTS * N_THREADS
        );
    }
}

pub mod attempt5 {
    // use scoped threads
    use std::{
        array,
        sync::{Arc, Mutex},
        thread::JoinHandle,
    };

    use super::*;

    fn thread(my_number: &Mutex<u64>) {
        for _ in 0..N_INCREMENTS {
            // we can't forget to lock the mutex before accessing the data
            // since it is impossible

            // the mutex also automatically unlocks when it goes out of scope
            *my_number.lock().unwrap() += 1;
        }
    }

    pub fn main() {
        let mut my_number: Mutex<u64> = Mutex::new(0);

        // use scoped threads, which automatically join when they go out of scope
        // helps the compiler realise that the threads will not outlive the mutex
        std::thread::scope(|scope| {
            for i in 0..N_THREADS {
                scope.spawn(|| thread(&my_number));
            }
        });

        println!(
            "Final total: {} (expected {})",
            my_number.lock().unwrap(),
            N_INCREMENTS * N_THREADS
        );
    }
}

pub mod attempt6 {
    // use atomics
    use std::{
        array,
        sync::{
            atomic::{AtomicU64, Ordering},
            Arc, Mutex,
        },
        thread::JoinHandle,
    };

    use super::*;

    static MY_NUMBER: AtomicU64 = AtomicU64::new(0);

    fn thread() {
        for _ in 0..N_INCREMENTS {
            MY_NUMBER.fetch_add(1, Ordering::SeqCst);
        }
    }

    pub fn main() {
        let mut threads: [_; N_THREADS] = array::from_fn(|_| None);

        for i in 0..N_THREADS {
            threads[i] = Some(std::thread::spawn(thread));
        }
        for i in 0..N_THREADS {
            threads[i].take().unwrap().join();
        }

        println!(
            "Final total: {} (expected {})",
            MY_NUMBER.load(Ordering::SeqCst),
            N_INCREMENTS * N_THREADS
        );
    }
}

pub mod attempt7 {
    // unsafe blocks to force it to compile
    use std::{
        array,
        sync::{
            atomic::{AtomicU64, Ordering},
            Arc, Mutex,
        },
        thread::JoinHandle,
    };

    use super::*;

    static mut MY_NUMBER: u64 = 0;

    fn thread() {
        for _ in 0..N_INCREMENTS {
            unsafe {
                MY_NUMBER += 1;
            }
        }
    }

    pub fn main() {
        let mut threads: [_; N_THREADS] = array::from_fn(|_| None);

        for i in 0..N_THREADS {
            threads[i] = Some(std::thread::spawn(thread));
        }
        for i in 0..N_THREADS {
            threads[i].take().unwrap().join();
        }

        println!(
            "Final total: {} (expected {})",
            unsafe { MY_NUMBER },
            N_INCREMENTS * N_THREADS
        );
    }
}
