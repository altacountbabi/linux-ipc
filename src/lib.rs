use serde::{de::DeserializeOwned, Serialize};
use std::{
    fmt::Debug,
    fs,
    io::{self, Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::PathBuf,
};

#[derive(Debug)]
pub struct IpcChannel {
    socket_path: PathBuf,
    listener: Option<UnixListener>,
    stream: Option<UnixStream>,
    is_client: bool,
}

type ReplyFn<T> = Box<dyn FnOnce(T) -> io::Result<()>>;

impl IpcChannel {
    pub fn new(path: &str) -> io::Result<Self> {
        fs::remove_file(path).ok();
        let listener = Some(UnixListener::bind(path)?);

        Ok(Self {
            socket_path: PathBuf::from(path),
            listener,
            stream: None,
            is_client: false,
        })
    }

    pub fn connect(path: &str) -> io::Result<Self> {
        let stream = Some(UnixStream::connect(path)?);

        Ok(Self {
            socket_path: PathBuf::from(path),
            listener: None,
            stream,
            is_client: true,
        })
    }

    fn get_stream(&mut self) -> io::Result<&mut UnixStream> {
        if self.stream.is_none() {
            let stream = UnixStream::connect(&self.socket_path)?;
            self.stream = Some(stream);
        }
        self.stream
            .as_mut()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotConnected, "Failed to connect to stream"))
    }

    pub fn send<T: Serialize>(&mut self, value: T) -> io::Result<Option<String>> {
        let stream = self.get_stream()?;
        let binary = bincode::serialize(&value).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        stream.write_all(&binary)?;
        stream.flush()?;
        stream.shutdown(std::net::Shutdown::Write)?;

        let mut response = String::new();
        stream.read_to_string(&mut response)?;

        if response.is_empty() {
            Ok(None)
        } else {
            Ok(Some(response))
        }
    }

    pub fn receive<T: DeserializeOwned + Debug, R: Serialize>(&mut self) -> io::Result<(T, ReplyFn<R>)> {
        let listener = self
            .listener
            .as_mut()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotConnected, "Listener not found"))?;
        let (mut stream, _) = listener.accept()?;

        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer)?;

        let request: T = bincode::deserialize(&buffer).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

        let reply_fn = move |value: R| {
            let binary = bincode::serialize(&value).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            stream.write_all(&binary)?;
            stream.flush()?;
            Ok(())
        };

        Ok((request, Box::new(reply_fn)))
    }
}

impl Drop for IpcChannel {
    fn drop(&mut self) {
        if !self.is_client {
            fs::remove_file(&self.socket_path).ok();
        }
    }
}
