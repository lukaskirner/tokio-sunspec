use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::vec;
use std::{future, net::SocketAddr};
use tokio_modbus::prelude::*;
use tokio_modbus::server::{self, Service};

pub async fn mock_server_context(socket_addr: SocketAddr, start_addr: u16) {
    println!("Starting up server...");
    let server = server::tcp::Server::new(socket_addr);

    server
        .serve(move || Ok(MockServer::new(start_addr)))
        .await
        .unwrap();
}

pub struct MockServer {
    start_addr: u16,
    holding_registers: Arc<Mutex<HashMap<u16, u16>>>,
}

impl MockServer {
    pub fn new(start_addr: u16) -> MockServer {
        let map = HashMap::from_iter([
            (0, 21365),
            (1, 28243),
            // Model id
            (2, 1),
            // Model length
            (3, 66),
            // Mn
            (4, 0x5465),
            (5, 0x7374),
        ]);

        return MockServer {
            start_addr,
            holding_registers: Arc::new(Mutex::new(map)),
        };
    }
}

impl Service for MockServer {
    type Request = Request;
    type Response = Response;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match req {
            Request::ReadHoldingRegisters(addr, cnt) => {
                let registers = read_holding_registers(
                    self.start_addr,
                    &self.holding_registers.lock().unwrap(),
                    addr,
                    cnt,
                );

                future::ready(Ok(Response::ReadHoldingRegisters(registers)))
            }
            Request::WriteMultipleRegisters(addr, values) => {
                write_holding_registers(
                    self.start_addr,
                    &mut self.holding_registers.lock().unwrap(),
                    addr,
                    values.clone(),
                );

                future::ready(Ok(Response::WriteMultipleRegisters(
                    addr,
                    values.len() as u16,
                )))
            }
            _ => unimplemented!(),
        }
    }
}

fn read_holding_registers(
    start_addr: u16,
    registers: &HashMap<u16, u16>,
    addr: u16,
    cnt: u16,
) -> Vec<u16> {
    let start_addr = addr - start_addr;
    let end_addr = start_addr + cnt - 1;

    let mut result = vec![0x0000; cnt as usize];

    let mut found_one = false;
    for n in start_addr..=end_addr {
        if let Some(r) = registers.get(&n) {
            let index = n - start_addr;
            result[index as usize] = *r;
            found_one = true;
        }
    }

    if found_one {
        return result;
    }

    return vec![0xFFFF; cnt as usize];
}

fn write_holding_registers(
    start_addr: u16,
    registers: &mut HashMap<u16, u16>,
    addr: u16,
    values: Vec<u16>,
) {
    for (i, v) in values.iter().enumerate() {
        registers.insert(addr - start_addr + i as u16, *v);
    }
}
