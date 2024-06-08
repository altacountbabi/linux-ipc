use linux_ipc::IpcChannel;

fn main() {
    let mut channel = IpcChannel::new("/tmp/example.sock").expect("Failed to create channel");

    loop {
        println!("Listening for messages...");
        let (result, reply) = channel.receive::<String, String>().expect("Failed to receive post");

        reply(result).expect("Failed to reply to client");
    }
}
