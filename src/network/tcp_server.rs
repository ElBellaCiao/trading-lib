use anyhow::Result;
use bytemuck::Pod;
use std::io::{BufReader, ErrorKind, Read};
use std::net::{SocketAddr, TcpListener, TcpStream};

pub struct TcpServer<T: Pod> {
    pub listener: TcpListener,
    stream: BufReader<TcpStream>,
    buffer: T,
}

impl<T: Pod> TcpServer<T> {
    pub fn new(socket_addr: impl Into<SocketAddr>) -> Result<Self> {
        let listener = TcpListener::bind(socket_addr.into())?;
        let (stream, _addr) = listener.accept()?;

        stream.set_nodelay(true)?;
        stream.set_nonblocking(false)?;

        Ok(Self {
            listener,
            stream: BufReader::new(stream),
            buffer: T::zeroed(),
        })
    }

    #[inline(always)]
    pub fn recv(&mut self) -> Result<Option<T>> {
        let bytes = bytemuck::bytes_of_mut(&mut self.buffer);
        match self.stream.read_exact(bytes) {
            Ok(()) => Ok(Some(self.buffer)),
            Err(e) => match e.kind() {
                // Connection closed successfully
                ErrorKind::UnexpectedEof => Ok(None),
                _ => Err(e.into()),
            },
        }
    }
}
