#![allow(unused)]

mod data_race;

fn main() {
    concurrency_review();

    concurrency_example();

    joining_threads();

    concurrency_issues();

    data_race::attempt4::main();
}

/// concurency:
///
/// imagine this scenario:
/// our program needs to perform 3 fetch requests, each of which don't depend on each other.
/// then it uses the results from all 3 requests to perform an overall result.
/// and assume we have a single-core CPU.
///
/// the naive approach would be to perform the requests sequentially, one after the other.
/// this would be slow, as we would have to wait for each request to finish before starting the next one.
///
/// a better approach would be to perform the requests concurrently.
/// this way, we can start all 3 requests at the same time, and then wait for all of them to finish.
/// this would be faster, as we would be able to overlap the time spent waiting for each request.
///
/// idea of a BLOCKING FUNCTION: a function that takes a long time to complete, and blocks the thread it's running on.
/// blocking call -> once we call the function, we can't do anything else until it returns.
fn concurrency_review() {}

// [EXAMPLE - writing a function simultaneously reads in input whilst printing out a message]
fn concurrency_example() {
    use std::io;
    use std::thread;
    use std::time::Duration;

    // [BAD EXAMPLE]
    // read in input from the user
    // let mut line = String::new();
    // io::stdin().read_line(&mut line).unwrap();

    // print out a message
    // loop {
    // println!("Hello, world");
    // thread::sleep(Duration::from_secs(1));
    // }

    // [WORKING EXAMPLE]
    thread::spawn(|| loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line);

        println!("Read line: {}", line);
    });

    loop {
        println!("Hello, world");
        thread::sleep(Duration::from_secs(1));
    }
}

fn joining_threads() {
    use std::thread;

    let handle = thread::spawn(|| {
        let mut sum = 0;
        for i in 1..10 {
            sum += i;
        }

        sum
    });

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
    }

    let x = handle.join().unwrap();
}

fn concurrency_issues() {
    // RACE CONDITIONS

    // see data_race.c and DataRace.java

    // idea: two threads are trying to access and write to the same data at the same time.
    // there is a race condition, as the final value of the data depends on the order in which the threads access it -- which is non-deterministic (i.e., undefined behaviour).

    // fix: use a MUTEX (mutual exclusion) to lock the data, so that only one thread can access it at a time.
}

// crossbream is a crate that has a lot of concurrency primitives
// https://docs.rs/crossbeam/0.8.0/crossbeam/
