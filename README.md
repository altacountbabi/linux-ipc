# Linux IPC

Linux IPC is a high-level Inter-Process Communication (IPC) library designed specifically for Linux systems. It provides a convenient and efficient way to facilitate communication between processes running on the same system using Unix domain sockets.

## Features

- **Simplified API:** This library offers a straightforward API for creating, connecting to, sending data and receiving data from Unix domain sockets, making IPC implementation hassle-free.
  
- **Client-Server Architecture:** The library supports both client and server roles, enabling you to establish communication channels between processes in a flexible and scalable manner.
  
- **Efficient Data Serialization:** This library uses serde and bincode for efficient serialization and deserialization of data, allowing you to send more complicated data structures over IPC.

## Getting Started

Add the dependency `linux-ipc` to your rust project and take a look at one of the example folders, it will contain an example for a server receiving a struct and a client sending the struct.


## Contributing

Contributions to this library are welcome! If you encounter any issues or have ideas for improvements, feel free to open an issue or submit a pull request on GitHub.

## License

This project is licensed under the GPLv3 License - see the [LICENSE](LICENSE) file for details.