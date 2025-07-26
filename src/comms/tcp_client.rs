use crate::model::{InstrumentId, TickData};
use anyhow::{Result, anyhow, bail};
use bincode::config::{Configuration, Fixint, LittleEndian, NoLimit};
use heapless::FnvIndexMap;
use manager_service_discovery_client::AddressBook;
use std::collections::HashMap;
use std::io::Write;
use std::net::IpAddr;
use std::net::TcpStream;

pub struct TcpClient<const MAX_IPS: usize, const MAX_IPS_PER_INSTRUMENT: usize> {
    instrument_to_ips:
        FnvIndexMap<InstrumentId, heapless::Vec<TcpStream, MAX_IPS_PER_INSTRUMENT>, MAX_IPS>,
    tick_data_buffer: Vec<u8>,
    serialization_config: Configuration<LittleEndian, Fixint, NoLimit>,
}

impl<const MAX_IPS: usize, const MAX_IPS_PER_INSTRUMENT: usize>
    TcpClient<MAX_IPS, MAX_IPS_PER_INSTRUMENT>
{
    const TICK_DATA_SERIALIZED_SIZE: usize = 128;
    pub fn new(address_book: AddressBook, socket: u16) -> Result<Self> {
        Ok(TcpClient {
            instrument_to_ips: Self::create_fixed_map(address_book.instrument_to_ips, socket)?,
            tick_data_buffer: Vec::with_capacity(Self::TICK_DATA_SERIALIZED_SIZE),
            serialization_config: bincode::config::standard().with_fixed_int_encoding(),
        })
    }

    #[inline(always)]
    pub fn publish(&mut self, tick_data: TickData) -> Result<()> {
        let ips = self
            .instrument_to_ips
            .get_mut(&tick_data.instrument_id)
            .ok_or_else(|| anyhow!("Instrument {} not found", tick_data.instrument_id))?;

        self.tick_data_buffer.clear();
        bincode::encode_into_std_write(
            tick_data,
            &mut self.tick_data_buffer,
            self.serialization_config,
        )?;

        for stream in ips.iter_mut() {
            stream.write_all(&self.tick_data_buffer)?;
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
