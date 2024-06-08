use linux_ipc::IpcChannel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    pub name: String,
    pub content: String,
}

fn main() {
    let mut channel = IpcChannel::new("/tmp/example.sock").expect("Failed to create channel");

    loop {
        let (response, reply) = channel.receive::<Test, Test>().expect("Failed to receive post");
        println!("Received: {:#?}", response);

        let to_send = Test {
            name: response.content,
            content: response.name,
        };

        println!("\nSending: {:#?}", to_send);

        reply(to_send).expect("Failed to reply to client");
    }
}
