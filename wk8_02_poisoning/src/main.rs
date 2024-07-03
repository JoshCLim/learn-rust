use std::sync::Mutex;

fn main() {
    let x = Mutex::new(0);

    // note how the .lock() returns a LockResult<MutexGuard<'_, T>>, so we are forced to unwrap...
    *x.lock().unwrap() += 1;

    // why? if another thread has locked the mutex, the current thread will block until the mutex is available. but if that thread panics while holding the lock, the mutex is now poisoned and the current thread will get an Err

    // the .clear_poison() method clears the poison

    // other libraries like parking_lot simply just release the lock when a thread panics, so the other threads can continue
}
