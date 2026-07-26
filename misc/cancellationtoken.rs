/**
Add in cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
tokio-util = "0.7"
anyhow = "1"
*/


use std::time::Duration;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Default)]
struct Record {
    a: i32,
    b: i32,
    c: i32,
}

async fn update_record(token: CancellationToken) -> Record {
    let mut record = Record::default();

    record.a = 1;
    tokio::time::sleep(Duration::from_millis(10)).await;
    if token.is_cancelled() {
        println!("cancelled after field a — rolling back");
        return Record::default();
    }

    record.b = 2;
    tokio::time::sleep(Duration::from_millis(10)).await;
    if token.is_cancelled() {
        println!("cancelled after field b — rolling back");
        return Record::default();
    }

    record.c = 3;
    record
}

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();
    let child_token = token.clone();

    let mut handle = tokio::spawn(async move { update_record(child_token).await });

    tokio::select! {
        result = &mut handle => {
            println!("update finished on its own: {:?}", result.unwrap());
        }
        _ = tokio::time::sleep(Duration::from_millis(10005)) => {
            println!("deadline hit — signaling cooperative cancellation");
            token.cancel();
            let result = handle.await.unwrap();
            println!("final result: {:?}", result);
        }
    }
}
