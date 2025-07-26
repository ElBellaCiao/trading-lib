use crate::model::InstrumentId;
use anyhow::{Result, anyhow, bail};
use bytemuck::Pod;
use heapless::FnvIndexMap;
use manager_service_discovery_client::AddressBook;
use std::collections::HashMap;
use std::io::Write;
use std::net::IpAddr;
use std::net::TcpStream;

pub struct TcpClient<const MAX_IPS: usize, const MAX_IPS_PER_INSTRUMENT: usize> {
    instrument_to_ips:
        FnvIndexMap<InstrumentId, heapless::Vec<TcpStream, MAX_IPS_PER_INSTRUMENT>, MAX_IPS>,
}

impl<const MAX_IPS: usize, const MAX_IPS_PER_INSTRUMENT: usize>
    TcpClient<MAX_IPS, MAX_IPS_PER_INSTRUMENT>
{
    pub fn new(address_book: AddressBook, socket: u16) -> Result<Self> {
        Ok(TcpClient {
            instrument_to_ips: Self::create_fixed_map(address_book.instrument_to_ips, socket)?,
        })
    }

    #[inline(always)]
    pub fn send<T: Pod>(&mut self, instrument_id: &InstrumentId, data: &T) -> Result<()> {
        let ips = self
            .instrument_to_ips
            .get_mut(instrument_id)
            .ok_or_else(|| anyhow!("Instrument {} not found", instrument_id))?;

        let bytes = bytemuck::bytes_of(data);

        for stream in ips.iter_mut() {
            stream.write_all(bytes)?;
            stream.flush()?;
        }

        Ok(())
    }

    fn create_fixed_map(
        address_book: HashMap<InstrumentId, Vec<IpAddr>>,
        port: u16,
    ) -> Result<FnvIndexMap<InstrumentId, heapless::Vec<TcpStream, MAX_IPS_PER_INSTRUMENT>, MAX_IPS>>
    {
        let mut fixed_map = FnvIndexMap::new();
        for (instrument_id, ips) in address_book {
            if ips.len() > MAX_IPS_PER_INSTRUMENT {
                bail!(
                    "Instrument {} has too many IPs: {} exceeds maximum of {}",
                    instrument_id,
                    ips.len(),
                    MAX_IPS_PER_INSTRUMENT
                );
            }

            let mut ip_list = heapless::Vec::new();
            for ip in ips {
                let stream = TcpStream::connect((ip, port))
                    .map_err(|e| anyhow::anyhow!("Failed to connect to {}: {}", ip, e))?;
                stream.set_nodelay(true)?;
                stream.set_nonblocking(false)?;

                ip_list.push(stream).map_err(|_| {
                    anyhow::anyhow!("Failed to push IP for instrument {}", instrument_id)
                })?;
            }
            fixed_map
                .insert(instrument_id, ip_list)
                .map_err(|_| anyhow::anyhow!("Too many instruments: maximum is {}", MAX_IPS))?;
        }

        Ok(fixed_map)
    }
}
