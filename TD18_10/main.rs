use std::sync::atomic::{AtomicUsize, Ordering};
mod threadpool;

use threadpool::{ThreadPool, Worker};

fn main() {
    let pool = ThreadPool::new(4);
    let count = Arc::new(AtomicUsize::new(0));

    let count1 = count.clone();
    pool.execute(move || {
        println!("Thread 1");
        count1.fetch_add(1, Ordering::SeqCst);
        std::thread::sleep(std::time::Duration::from_secs(1));
    });
    let count2 = count.clone();
    pool.execute(move || {
        println!("Thread 2");
        count2.fetch_add(1, Ordering::SeqCst);
        std::thread::sleep(std::time::Duration::from_secs(1));
    });
    let count3 = count.clone();
    pool.execute(move || {
        println!("Thread 3");
        count3.fetch_add(1, Ordering::SeqCst);
        std::thread::sleep(std::time::Duration::from_secs(1));
    });
    let count4 = count.clone();
    pool.execute(move || {
        println!("Thread 4");
        count4.fetch_add(1, Ordering::SeqCst);
        std::thread::sleep(std::time::Duration::from_secs(1));
    });
    std::thread::sleep(std::time::Duration::from_secs(2));

    let count = count.load(Ordering::SeqCst);

    assert_eq!(count, 4);
}
