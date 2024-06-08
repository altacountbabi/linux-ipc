use linux_ipc::IpcChannel;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    pub name: String,
    pub content: String,
}

fn main() {
    let arg = &env::args().collect::<Vec<String>>()[1..].join(" ");
    let mut channel = IpcChannel::connect("/tmp/example.sock").expect("Failed to create channel");

    for _ in 0..2 {
        let test = Test {
            name: "test".to_string(),
            content: arg.to_string(),
        };

        println!("Sending: {:#?}", test);

        let response = channel.send::<_, Test>(test).expect("Failed to send message");

        if let Some(response) = response {
            println!("Received: {:#?}", response);
        }
    }
}
