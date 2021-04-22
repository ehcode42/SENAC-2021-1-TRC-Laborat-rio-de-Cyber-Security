#[tokio::main]
async fn main() {
    warp::serve(warp::fs::dir("publico"))
        .run(([0,0,0,0],8080))
        .await;
}
