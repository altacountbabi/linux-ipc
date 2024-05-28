use linux_ipc::IpcChannel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct Post {
    title: String,
    body: String,
}

fn main() {
    println!("Creating example channel");
    let mut channel = IpcChannel::new("/tmp/example.sock").expect("Failed to create channel");

    loop {
        println!("Receiving post in channel `init`");
        let result = channel.receive::<Post>();

        if result.is_err() {
            eprintln!("{}", result.unwrap_err());
        } else {
            let result = result.unwrap();
            println!("{:?}", result);
        }
    }
}
