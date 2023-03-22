pub mod error;
pub mod model;
pub mod models;
pub mod point;
pub mod types;
pub mod utils;

use error::Error;
use model::Model;
use point::{Point, PointType};
use std::{collections::HashMap, net::SocketAddr};
use tokio_modbus::{client::Context, prelude::*};
use tokio_serial::SerialStream;
use types::Address;

pub struct Client {
    /// Slave Id of currently selected device
    pub slave_id: u8,

    /// Address where the sunspec models start
    pub start_address: Address,

    /// Contains all discovered models. Key = Model id, Value = Start address
    pub models: HashMap<u8, HashMap<u16, Address>>,

    /// Modbus client
    modbus_client: Context,
}

#[cfg(feature = "tcp")]
pub async fn connect_tcp(
    socket_addr: SocketAddr,
    slave_id: u8,
    start_address: Address,
) -> Result<Client, Error> {
    let modbus_client = tcp::connect_slave(socket_addr, Slave(slave_id))
        .await
        .map_err(Error::Io)?;

    return connect(modbus_client, slave_id, start_address).await;
}

#[cfg(feature = "rtu")]
pub async fn connect_rtu(
    device_path: &str,
    baud_rate: u32,
    slave_id: u8,
    start_address: Address,
) -> Result<Client, Error> {
    let builder = tokio_serial::new(device_path, baud_rate);
    let serial = SerialStream::open(&builder).unwrap();
    let modbus_client = rtu::connect_slave(serial, Slave(slave_id))
        .await
        .map_err(Error::Io)?;

    return connect(modbus_client, slave_id, start_address).await;
}

pub async fn connect(
    client: Context,
    slave_id: u8,
    start_address: Address,
) -> Result<Client, Error> {
    let mut client = Client {
        slave_id,
        start_address,
        models: HashMap::new(),
        modbus_client: client,
    };

    client.model_discovery().await?;
    return Ok(client);
}

impl Client {
    /// Discover the supported models of the connected device.
    async fn model_discovery(&mut self) -> Result<(), Error> {
        let mut base_addr = self.start_address;

        // Check for Sunspec identifier
        let res = self
            .read_holding_registers(base_addr, 2)
            .await
            .expect("SunS identifier");

        if res != vec![0x5375, 0x6e53] {
            return Err(Error::Client());
        }
        base_addr += 2;

        // Scan supported models
        loop {
            let res = self.read_holding_registers(base_addr, 2).await?;
            let model_id = res[0];
            let model_length = res[1];

            if model_id == 0xFFFF || model_length == 0xFFFF {
                return Ok(()); // Last model reached. We are done parsing.
            }

            let models = self
                .models
                .entry(self.slave_id)
                .or_insert_with(|| HashMap::new());
            models.insert(model_id, base_addr + 2);

            base_addr += 2; // increase by two register which we were reading earlier
            base_addr += model_length; // increase by length of model to get to next model
        }
    }

    /// Easy access to modbus `read_holding_registers`.
    async fn read_holding_registers(&mut self, addr: Address, cnt: u16) -> Result<Vec<u16>, Error> {
        return self
            .modbus_client
            .read_holding_registers(addr, cnt)
            .await
            .map_err(Error::Io);
    }

    /// Easy access to modbus `write_multiple_registers`.
    async fn write_holding_registers(
        &mut self,
        addr: Address,
        data: Vec<u16>,
    ) -> Result<(), Error> {
        return self
            .modbus_client
            .write_multiple_registers(addr, &data)
            .await
            .map_err(Error::Io);
    }
}

impl Client {
    /// Read the data for the given point
    pub async fn read_point<T: Model, K: PointType<K>>(
        &mut self,
        point: Point<T, K>,
    ) -> Result<K, Error> {
        let model = self.models.get(&self.slave_id).unwrap();
        if let Some(model_addr) = model.get(&T::ID) {
            let address = *model_addr + point.offset;
            return self
                .read_holding_registers(address, point.length)
                .await
                .and_then(|res| Ok(K::decode(res)));
        }

        return Err(Error::UnsupportedModel(T::ID));
    }

    /// Write data to the given point
    pub async fn write_point<T: Model, K: PointType<K>>(
        &mut self,
        point: Point<T, K>,
        data: K,
    ) -> Result<(), Error> {
        if !point.write_access {
            return Err(Error::WriteNotSupported());
        }

        let model = self.models.get(&self.slave_id).unwrap();
        if let Some(model_addr) = model.get(&T::ID) {
            let address = *model_addr + point.offset;
            let buff = K::encode(data);
            return self.write_holding_registers(address, buff).await;
        }

        return Err(Error::UnsupportedModel(T::ID));
    }

    /// Set a new slave_id, and do model discovery if it has not yet been done.
    ///
    /// This allows re-using a single connection (TCP or RTU) to use multiple
    /// SunSpec devices on a single bus.
    pub async fn set_slave(&mut self, slave_id: u8) -> Result<(), Error> {
        self.slave_id = slave_id;
        self.modbus_client.set_slave(Slave(slave_id));

        if !self.models.contains_key(&slave_id) {
            self.model_discovery().await?;
        }

        Ok(())
    }
}
