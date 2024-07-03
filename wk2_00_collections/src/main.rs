fn main() {
    println!("Hello, world!");
}

fn _array() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _second = a[1];
}

fn _vector() {
    let mut v: Vec<i32> = Vec::new(); // or vec![];
    v.push(1);
    v.push(2);
    v.push(3);
    v.remove(2);
    let _first = v[0];
    let _second = v[1];

    // declare a vector using a macro
    let v: Vec<i32> = vec![1, 2, 3];
    let _first = v[0];

    // iterate through elements in a vector
    for elem in v {
        println!("{}", elem);
    }

    // reduce reallocations
    let mut v: Vec<i32> = Vec::with_capacity(200);
    for i in 0..200 {
        v.push(i);
    }

    // get iterator through elements in a vector
    let mut iter = v.iter();
    let _first: Option<&i32> = iter.next();

    // turn vector into an iterator
    let mut iter = v.into_iter();
    let _first: Option<i32> = iter.next();
}

fn _string() {
    let mut s: String = String::new();
    s.push_str("hello");
    s.push(' ');
    s.push_str("world");
    let _first = s.chars().nth(0);
    let _second = s.chars().nth(1);

    // declare a string using a macro
    let s: String = "hello world".to_string();
    let _first = s.chars().nth(0);

    // iterate through characters in a string
    for c in s.chars() {
        println!("{}", c);
    }

    // get iterator through characters in a string
    let mut iter = s.chars();
    let _first: Option<char> = iter.next();
}

fn _hash_map() {
    use std::collections::HashMap;

    let mut m: HashMap<String, i32> = HashMap::new();
    m.insert("hello".to_string(), 1);
    m.insert("world".to_string(), 2);
    let _first = m.get("hello");
    let _second = m.get("world");

    // declare a hash map using a macro
    let mut m: HashMap<String, i32> = [("hello".to_string(), 1), ("world".to_string(), 2)]
        .iter()
        .cloned()
        .collect();
    let _first = m.get("hello");
    let _second = m.get("world");

    // insert if key does not exist
    m.entry("hello".to_string()).and_modify(|x| *x += 1).or_insert(1);

    // iterate through key-value pairs in a hash map
    for (k, v) in &m {
        println!("{}: {}", k, v);
    }

    // get iterator through key-value pairs in a hash map
    let mut iter = m.iter();
    let _first: Option<(&String, &i32)> = iter.next();

    // turn hash map into an iterator
    let mut iter = m.into_iter();
    let _first: Option<(String, i32)> = iter.next();
}

// examples
fn _iterator_methods() {
    let _v: Vec<i32> = vec![1, 2, 3];

    // map
    let _v: Vec<i32> = _v.iter().map(|x| x * 2).collect();

    // filter
    let _v = _v.iter().filter(|x| **x > 1);
}

// example functions
fn _mean(x: Vec<i32>) -> Option<f64> {
    if x.is_empty() {
        None
    } else {
        // the .sum() function is a zero-cost abstraction -- it is optimized to be as fast as a for loop
        Some((x.iter().sum::<i32>() as f64) / (x.len() as f64))
    }
}

fn _mode(x: Vec<i32>) -> Option<i32> {
    use std::collections::HashMap;

    let mut counts: HashMap<i32, i32> = HashMap::new();
    for elem in x {
        *counts.entry(elem).or_insert(0) += 1;
    }
    
    match counts.into_iter().max_by_key(|(_, count)| *count) {
        Some((elem, _)) => Some(elem),
        None => None,
    }
}

fn _longest_equal_run_imperative(x: Vec<i32>, y: Vec<i32>) -> usize {
    let mut max_run = 0;
    let mut run = 0;
    for i in 0..x.len() {
        if x[i] == y[i] {
            run += 1;
        } else {
            run = 0;
        }
        if run > max_run {
            max_run = run;
        }
    }
    max_run
}

fn _longest_equal_run_functional(x: Vec<i32>, y: Vec<i32>) -> usize {
    x.iter()
        .zip(y.iter())
        .map(|(x, y)| x == y)
        .fold((0, 0), |(max_run, run), eq| {
            if eq {
                (max_run.max(run + 1), run + 1)
            } else {
                (max_run, 0)
            }
        })
        .0
}
