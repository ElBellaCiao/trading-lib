use anyhow::Result;
use bytemuck::Pod;
use std::io::{BufReader, ErrorKind, Read};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};

pub struct TcpServer {
    pub listener: TcpListener,
    stream: BufReader<TcpStream>,
}

impl TcpServer {
    pub fn new(port: u16) -> Result<Self> {
        let socket_addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
        let listener = TcpListener::bind(socket_addr)?;
        let (stream, _addr) = listener.accept()?;

        stream.set_nodelay(true)?;
        stream.set_nonblocking(false)?;

        Ok(Self {
            listener,
            stream: BufReader::new(stream),
        })
    }

    #[inline(always)]
    pub fn recv<T: Pod>(&mut self) -> Result<Option<T>> {
        let mut buffer = T::zeroed();
        let bytes = bytemuck::bytes_of_mut(&mut buffer);
        match self.stream.read_exact(bytes) {
            Ok(()) => Ok(Some(buffer)),
            Err(e) => match e.kind() {
                // Connection closed successfully
                ErrorKind::UnexpectedEof => Ok(None),
                _ => Err(e.into()),
            },
        }
    }
}
