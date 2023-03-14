use std::{net::SocketAddr, time::Duration};
use tokio_sunspec::error::Error;
use tokio_sunspec::models::model1;

mod mock;

#[tokio::test]
async fn read_string() -> Result<(), Error> {
    let socket_addr = "127.0.0.1:5502".parse().unwrap();
    let slave_id: u8 = 255;
    let start_addr: u16 = 40000;

    tokio::select! {
        _ = mock::mock_server_context(socket_addr, start_addr) => unreachable!(),
        resp = client_context(socket_addr, slave_id, start_addr) => return resp,
    }
}

async fn client_context(socket_addr: SocketAddr, s_id: u8, start_addr: u16) -> Result<(), Error> {
    // Give the server some time for starting up
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Connect client
    let mut client = tokio_sunspec::connect_tcp(socket_addr, s_id, start_addr).await?;

    let mut model_ids = Vec::from_iter(client.models.keys().cloned());
    model_ids.sort();
    assert_eq!(model_ids, vec![1]);

    let res = client.read_point(model1::Mn).await?;
    assert_eq!(res, "Test");

    return Ok(());
}
