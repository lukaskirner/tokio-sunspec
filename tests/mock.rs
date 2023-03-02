use std::{future, net::SocketAddr};
use tokio_modbus::prelude::*;
use tokio_modbus::server::{self, Service};

pub async fn server_context(socket_addr: SocketAddr, start_addr: u16) {
    println!("Starting up server...");
    let server = server::tcp::Server::new(socket_addr);

    server
        .serve(move || Ok(MockServer { start_addr }))
        .await
        .unwrap();
}

struct MockServer {
    start_addr: u16,
}

impl Service for MockServer {
    type Request = Request;
    type Response = Response;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match req {
            Request::ReadInputRegisters(addr, cnt) => {
                let registers = self.get_data_by_address(addr, cnt);
                future::ready(Ok(Response::ReadInputRegisters(registers)))
            }
            Request::ReadHoldingRegisters(addr, cnt) => {
                let registers = self.get_data_by_address(addr, cnt);
                future::ready(Ok(Response::ReadHoldingRegisters(registers)))
            }
            _ => unimplemented!(),
        }
    }
}

impl MockServer {
    pub fn get_data_by_address(&self, addr: u16, cnt: u16) -> Vec<u16> {
        let start_addr = addr - self.start_addr;
        let end_addr = start_addr + cnt - 1;

        match (start_addr, end_addr) {
            (0, 1) => vec![0x5375, 0x6e53],
            (2, 2) => vec![1],  // ID
            (3, 3) => vec![66], // Length
            (4, 19) => vec![
                0x0044, 0x0065, 0x006d, 0x006f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ], // Mn
            _ => vec![0x0; cnt.into()],
        }
    }
}
