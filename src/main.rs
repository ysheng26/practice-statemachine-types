use std::{thread::sleep, time::Duration};

mod state;

fn main() {
    let remote_control = state::spawn_client();
    let res = remote_control.to_green();
    println!("{:?}", res);

    let res = remote_control.to_yellow();
    println!("{:?}", res);

    let res = remote_control.to_red();
    println!("{:?}", res);

    let res = remote_control.to_green();
    println!("{:?}", res);
}
