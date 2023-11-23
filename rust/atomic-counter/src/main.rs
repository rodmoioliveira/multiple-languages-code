use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::task;

#[tokio::main]
async fn main() {
    let val = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::with_capacity(1000);

    for _ in 0..1000 {
        let val = Arc::clone(&val);

        let handle = task::spawn(async move {
            let _ = val.fetch_add(1, Ordering::Relaxed);
            println!("Result: {:?}", val.load(Ordering::Relaxed));
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
    println!("Result: {:?}", val.load(Ordering::Relaxed));
}
