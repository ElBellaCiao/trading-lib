use anyhow::{Result, anyhow};
use bytemuck::Pod;
use retry::{delay::Exponential, retry};
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpStream;

pub struct MonoTcpClient {
    streams: Vec<TcpStream>,
}

impl MonoTcpClient {
    const INITIAL_DELAY_MS: u64 = 100;
    const MAX_RETRY_ATTEMPTS: usize = 12;

    pub fn new(address_book: Vec<SocketAddr>) -> Result<Self> {
        let streams: Result<Vec<TcpStream>> =
            address_book.iter().map(Self::connect_and_add).collect();

        Ok(MonoTcpClient { streams: streams? })
    }

    fn connect_and_add(addr: &SocketAddr) -> Result<TcpStream> {
        let stream = retry(
            Exponential::from_millis(Self::INITIAL_DELAY_MS).take(Self::MAX_RETRY_ATTEMPTS),
            || TcpStream::connect(addr),
        )
        .map_err(|e| {
            anyhow!(
                "Failed to connect to {addr:?} after retrying for {} ms: {e:?}",
                Self::INITIAL_DELAY_MS * Self::MAX_RETRY_ATTEMPTS as u64
            )
        })?;

        stream.set_nodelay(true)?;
        stream.set_nonblocking(false)?;

        Ok(stream)
    }

    pub fn send<T: Pod>(&mut self, data: &T) -> Result<()> {
        let bytes = bytemuck::bytes_of(data);

        for stream in &mut self.streams {
            stream.write_all(bytes)?;
            stream.flush()?;
        }

        Ok(())
    }
}
