use futures::stream::{FuturesUnordered, StreamExt};
use tokio::time::{sleep, Duration};


async fn task(id: u8, delay: u64) -> u8 {
    sleep(Duration::from_millis(delay)).await;
    println!("task {id} finished");
    id
}

#[tokio::main]
async fn main() {
        let mut set = 
        FuturesUnordered::new();

    set.push(task(1, 300));
    set.push(task(2, 100));
    set.push(task(3, 1000));
    set.push(task(4, 200));
    set.push(task(5, 3000));

    let mut results = vec![];

    while let Some(res) = set.next().await {
        println!("got result: {res}");
        results.push(res);
    }

    println!("final: {:?}", results);
}
