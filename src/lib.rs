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
    /// Slave Id of device
    pub slave_id: u8,

    /// Address where the sunspec models start
    pub start_address: Address,

    /// Contains all discovered models. Key = Model id, Value = Start address
    pub models: HashMap<u16, Address>,

    /// Modbus client
    modbus_client: Context,
}

pub async fn connect_tcp(
    socket_addr: SocketAddr,
    slave_id: u8,
    start_address: Address,
) -> Result<Client, Error> {
    println!("Connecting SunSpec client ...");
    let context = tcp::connect_slave(socket_addr, Slave(slave_id)).await;
    if context.is_err() {
        return Err(Error::Io(context.err().unwrap()));
    }

    println!("SunSpec client connected to `{}`.", socket_addr);
    let mut client = Client {
        slave_id,
        start_address,
        models: HashMap::new(),
        modbus_client: context.unwrap(),
    };

    println!("SunSpec checking for compatability ...");
    let supported_models = client.model_discovery().await?;
    client.models = supported_models;

    return Ok(client);
}

pub async fn connect_rtu(
    device_path: &str,
    baud_rate: u32,
    slave_id: u8,
    start_address: Address,
) -> Result<Client, Error> {
    println!("Connecting SunSpec client ...");
    let builder = tokio_serial::new(device_path, baud_rate);
    let serial = SerialStream::open(&builder).unwrap();

    let context = rtu::connect_slave(serial, Slave(slave_id)).await;
    if context.is_err() {
        return Err(Error::Io(context.err().unwrap()));
    }

    println!("SunSpec client connected to `{}`.", device_path);
    let mut client = Client {
        slave_id,
        start_address,
        models: HashMap::new(),
        modbus_client: context.unwrap(),
    };

    println!("SunSpec checking for compatability ...");
    let supported_models = client.model_discovery().await?;
    client.models = supported_models;

    return Ok(client);
}

impl Client {
    /// Discover the supported models of the connected device.
    async fn model_discovery(&mut self) -> Result<HashMap<u16, Address>, Error> {
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
        let mut supported_models = HashMap::new();
        loop {
            let res = self.read_holding_registers(base_addr, 2).await?;
            let model_id = res[0];
            let model_length = res[1];

            if model_id == 0xFFFF || model_length == 0xFFFF {
                return Ok(supported_models); // Last model reached. We are done parsing.
            }
            supported_models.insert(model_id, base_addr + 2);

            base_addr += 2; // increase by two register which we were reading eralier
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
}

impl Client {
    /// Read the data for the given point
    pub async fn read_point<T: Model, K: PointType<K>>(
        &mut self,
        point: Point<T, K>,
    ) -> Result<K, Error> {
        if let Some(model_addr) = self.models.get(&T::ID) {
            let address = *model_addr + point.offset;
            return self
                .modbus_client
                .read_holding_registers(address, point.length)
                .await
                .and_then(|res| Ok(K::convert_to(res)))
                .map_err(Error::Io);
        }

        return Err(Error::UnsupportedModel(T::ID));
    }
}
