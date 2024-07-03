use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    // takes ~11s on my M1 Macbook to run
    let sum: f64 = (1..1000000000u64).into_iter().map(|x| x as f64).sum();
    println!("Sum: {}", sum);

    // can use rayon to parallelize the computation
    // takes ~6s on my M1 Macbook to run
    let sum = (1..1000000000u64)
        .into_par_iter()
        .map(|x| x as f64)
        .sum::<f64>();
    println!("Sum: {}", sum);
}
