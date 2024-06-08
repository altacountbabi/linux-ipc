use linux_ipc::IpcChannel;
use std::env;

fn main() {
    let arg = &env::args().collect::<Vec<String>>()[1..].join(" ");
    let mut channel = IpcChannel::connect("/tmp/example.sock").expect("Failed to create channel");
    let response = channel.send(arg).expect("Failed to send message");

    if let Some(response) = response {
        print!("{}\n", response);
    }
}
