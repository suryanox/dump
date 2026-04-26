use futures::future::join_all;
use tokio::time::{sleep, Duration};

async fn task(id: u8, delay: u64) -> u8 {
    sleep(Duration::from_millis(delay)).await;
    println!("task {id} finished");
    id
}

#[tokio::main]
async fn main() {
    let futures = vec![
        task(1, 300),
        task(2, 100),
        task(3, 200),
        task(4, 1000),
        task(5, 3000),
    ];

    let results = join_all(futures).await;

    println!("results: {:?}", results);
}
