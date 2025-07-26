use anyhow::Result;
use bytemuck::Pod;
use std::io::{BufReader, Read};
use std::net::{SocketAddr, TcpListener, TcpStream};

pub struct TcpServer<T: Pod> {
    pub listener: TcpListener,
    stream: BufReader<TcpStream>,
    buffer: T,
}

impl<T: Pod> TcpServer<T> {
    pub fn new(socket_addr: SocketAddr) -> Result<Self> {
        let listener = TcpListener::bind(socket_addr)?;
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
    pub fn receive(&mut self) -> Result<T> {
        let bytes = bytemuck::bytes_of_mut(&mut self.buffer);
        self.stream.read_exact(bytes)?;
        Ok(self.buffer)
    }
}
