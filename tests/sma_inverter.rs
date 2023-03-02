use tokio_sunspec::error::Error;
use tokio_sunspec::models::model1;

#[tokio::test]
async fn sma_inverter_test() -> Result<(), Error> {
    let socket_addr = "192.168.178.99:502".parse().unwrap();
    let slave_id: u8 = 126;
    let start_addr: u16 = 40000;

    let mut client = tokio_sunspec::connect_tcp(socket_addr, slave_id, start_addr).await?;

    let mut model_ids = Vec::from_iter(client.models.keys().cloned());
    model_ids.sort();

    assert_eq!(
        model_ids,
        vec![1, 11, 12, 103, 120, 121, 122, 123, 124, 126, 127, 128, 129, 130, 131, 132, 160]
    );

    let res = client.read_point(model1::Mn).await?;
    assert_eq!(res, "SMA");

    return Ok(());
}
