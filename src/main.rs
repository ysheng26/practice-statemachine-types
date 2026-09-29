mod state;

#[tokio::main]
async fn main() {
    let (remote_control, join_handle) = state::spawn_client();
    let res = remote_control.to_green().await;
    println!("{:?}", res);

    let res = remote_control.to_yellow().await;
    println!("{:?}", res);

    let res = remote_control.to_red().await;
    println!("{:?}", res);

    let res = remote_control.to_green().await;
    println!("{:?}", res);

    drop(remote_control);
    join_handle.await.unwrap();
}
