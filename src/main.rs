use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;
use std::time::Duration;

fn main() {
    // Create a vector of numbers to calculate squares for.
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Create a thread pool with 4 threads.
    let pool = ThreadPool::new(4);

    // Shared vector to store the results, wrapped in Arc and Mutex for thread-safe access.
    let results = Arc::new(Mutex::new(Vec::new()));

    for num in numbers {
        // Clone Arc so that each thread has ownership of the reference.
        let results = Arc::clone(&results);

        // Execute each task in the thread pool.
        pool.execute(move || {
            let square = num * num;
            println!("Calculating square of {}: {}", num, square);

            // Simulate work with sleep.
            std::thread::sleep(Duration::from_millis(500));

            // Store the result.
            let mut results = results.lock().unwrap();
            results.push((num, square));
        });
    }

    // Wait for all tasks to finish by dropping the pool.
    pool.join();

    // Print the results.
    let results = results.lock().unwrap();
    println!("\nResults:");
    for (num, square) in results.iter() {
        println!("{}^2 = {}", num, square);
    }
}
