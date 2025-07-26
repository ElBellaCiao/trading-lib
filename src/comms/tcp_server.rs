use anyhow::Result;
use bytemuck::Pod;
use std::io::{BufReader, Read};
use std::net::{SocketAddr, TcpListener, TcpStream};

pub struct TcpServer<const BUFFER_SIZE: usize> {
    pub listener: TcpListener,
    stream: BufReader<TcpStream>,
    buffer: [u8; BUFFER_SIZE],
}

impl<const BUFFER_SIZE: usize> TcpServer<BUFFER_SIZE> {
    pub fn new(socket_addr: SocketAddr) -> Result<Self> {
        let listener = TcpListener::bind(socket_addr)?;
        let (stream, _addr) = listener.accept()?;

        stream.set_nodelay(true)?;
        stream.set_nonblocking(false)?;

        Ok(Self {
            listener,
            stream: BufReader::new(stream),
            buffer: [0; BUFFER_SIZE],
        })
    }

    pub fn receive<T: Pod>(&mut self) -> Result<T> {
        const { assert!(size_of::<T>() == BUFFER_SIZE) };

        self.stream.read_exact(&mut self.buffer)?;
        Ok(*bytemuck::from_bytes(&self.buffer))
    }
}
