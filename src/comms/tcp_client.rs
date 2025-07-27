use crate::model::InstrumentId;
use anyhow::{Result, anyhow, bail};
use bytemuck::Pod;
use heapless::FnvIndexMap;
use manager_service_discovery_client::AddressBook;
use retry::{delay::Fixed, retry};
use std::collections::HashMap;
use std::io::Write;
use std::net::IpAddr;
use std::net::TcpStream;

pub struct TcpClient<const MAX_IPS: usize, const MAX_IPS_PER_INSTRUMENT: usize> {
    instrument_to_ips:
        FnvIndexMap<InstrumentId, TcpConnectionPool<MAX_IPS_PER_INSTRUMENT>, MAX_IPS>,
}

impl<const MAX_IPS: usize, const MAX_IPS_PER_INSTRUMENT: usize>
    TcpClient<MAX_IPS, MAX_IPS_PER_INSTRUMENT>
{
    pub fn new(address_book: AddressBook, socket: u16) -> Result<Self> {
        Self::validate_address_book(&address_book.instrument_to_ips)?;

        Ok(TcpClient {
            instrument_to_ips: Self::create_fixed_map(address_book.instrument_to_ips, socket)?,
        })
    }

    #[inline(always)]
    pub fn send<T: Pod>(&mut self, instrument_id: &InstrumentId, data: &T) -> Result<()> {
        let connection_pool = self
            .instrument_to_ips
            .get_mut(instrument_id)
            .ok_or_else(|| anyhow!("Instrument {} not found", instrument_id))?;

        connection_pool.send_to_all(data)
    }

    fn validate_address_book(address_book: &HashMap<InstrumentId, Vec<IpAddr>>) -> Result<()> {
        if address_book.len() > MAX_IPS {
            bail!(
                "Too many instruments: {} exceeds maximum of {}",
                address_book.len(),
                MAX_IPS
            );
        }

        for (instrument_id, ips) in address_book {
            if ips.len() > MAX_IPS_PER_INSTRUMENT {
                bail!(
                    "Instrument {} has too many IPs: {} exceeds maximum of {}",
                    instrument_id,
                    ips.len(),
                    MAX_IPS_PER_INSTRUMENT
                );
            }
        }

        Ok(())
    }

    fn create_fixed_map(
        address_book: HashMap<InstrumentId, Vec<IpAddr>>,
        port: u16,
    ) -> Result<FnvIndexMap<InstrumentId, TcpConnectionPool<MAX_IPS_PER_INSTRUMENT>, MAX_IPS>> {
        let mut fixed_map = FnvIndexMap::new();
        for (instrument_id, ips) in address_book {
            let mut connection_pool = TcpConnectionPool::new();
            for ip in ips {
                connection_pool.connect_and_add(ip, port)?;
            }

            fixed_map
                .insert(instrument_id, connection_pool)
                .map_err(|_| anyhow!("Too many instruments: maximum is {}", MAX_IPS))?;
        }
        Ok(fixed_map)
    }
}

pub struct TcpConnectionPool<const MAX_CONNECTIONS: usize> {
    streams: heapless::Vec<TcpStream, MAX_CONNECTIONS>,
}

impl<const MAX_CONNECTIONS: usize> TcpConnectionPool<MAX_CONNECTIONS> {
    const RETRY_DELAY_MS: u64 = 5000;
    const RETRY_ATTEMPTS: usize = 60;

    fn new() -> Self {
        Self {
            streams: heapless::Vec::new(),
        }
    }

    fn connect_and_add(&mut self, ip: IpAddr, port: u16) -> Result<()> {
        let stream = retry(
            Fixed::from_millis(Self::RETRY_DELAY_MS).take(Self::RETRY_ATTEMPTS),
            || TcpStream::connect((ip, port)),
        )
        .map_err(|e| {
            anyhow!(
                "Failed to connect to {ip}:{port} after retrying for {} ms: {e:?}",
                Self::RETRY_DELAY_MS * Self::RETRY_ATTEMPTS as u64
            )
        })?;

        stream.set_nodelay(true)?;
        stream.set_nonblocking(false)?;

        self.streams
            .push(stream)
            .map_err(|_| anyhow!("Too many connections: maximum is {}", MAX_CONNECTIONS))?;

        Ok(())
    }

    #[inline(always)]
    pub fn send_to_all<T: Pod>(&mut self, data: &T) -> Result<()> {
        let bytes = bytemuck::bytes_of(data);

        for stream in &mut self.streams {
            stream.write_all(bytes)?;
            stream.flush()?;
        }

        Ok(())
    }
}
