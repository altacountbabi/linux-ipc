use linux_ipc::IpcChannel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct Post {
    pub title: String,
    pub body: String,
}

impl Post {
    fn new(title: &str, body: &str) -> Self {
        Self {
            title: title.to_string(),
            body: body.to_string(),
        }
    }
}

fn main() {
    println!("Connecting to example channel");
    let mut channel = IpcChannel::bind("/tmp/example.sock").expect("Failed to create channel");
    let post = Post::new(
        "Hello world!",
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Diam phasellus vestibulum lorem sed risus ultricies tristique nulla. Nibh sit amet commodo nulla facilisi nullam vehicula ipsum a. Eget dolor morbi non arcu risus quis varius. Et ligula ullamcorper malesuada proin libero nunc consequat interdum varius."
    );

    println!("Sending message in channel `example`");
    let response = channel.send(post).expect("Failed to send message");

    if response.is_some() {
        print!("{}", response.unwrap());
    }
}
