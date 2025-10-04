use crate::model::InstrumentId;
use crate::network::MonoTcpClient;
use anyhow::{Result, anyhow};
use bytemuck::Pod;
use rustc_hash::FxHashMap;
use std::collections::HashMap;
use std::net::SocketAddr;

pub struct RoutingTcpClient {
    address_map: FxHashMap<InstrumentId, MonoTcpClient>,
}

impl RoutingTcpClient {
    pub fn new(addresses: Vec<(InstrumentId, SocketAddr)>) -> Result<Self> {
        let mut grouped: HashMap<InstrumentId, Vec<SocketAddr>> = Default::default();
        for (instrument_id, addr) in addresses {
            grouped.entry(instrument_id).or_default().push(addr);
        }

        let mut address_map = FxHashMap::default();
        for (instrument_id, addrs) in grouped {
            let client = MonoTcpClient::new(addrs)?;
            address_map.insert(instrument_id, client);
        }

        Ok(Self { address_map })
    }

    pub fn send<T: Pod>(&mut self, instrument_id: InstrumentId, data: &T) -> Result<()> {
        self.address_map
            .get_mut(&instrument_id)
            .ok_or_else(|| anyhow!("Instrument {} not found", instrument_id))?
            .send(data)
    }
}
