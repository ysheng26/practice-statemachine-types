mod state;

fn main() {
    let remote_control = state::spawn_client();
    remote_control.to_green();
    remote_control.to_yellow();
    remote_control.to_red();
    remote_control.to_green();
}
