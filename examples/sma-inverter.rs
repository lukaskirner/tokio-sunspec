use tokio_sunspec::error::Error;
use tokio_sunspec::models::{model1, model120};
use tokio_sunspec::utils;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let socket_addr = "192.168.178.99:502".parse().unwrap();
    let slave_id: u8 = 126;
    let start_addr: u16 = 40000;

    let mut client = tokio_sunspec::connect_tcp(socket_addr, slave_id, start_addr).await?;

    // Assert supported models
    let mut model_ids = Vec::from_iter(client.models.keys().cloned());
    model_ids.sort();
    assert_eq!(
        model_ids,
        vec![1, 11, 12, 103, 120, 121, 122, 123, 124, 126, 127, 128, 129, 130, 131, 132, 160]
    );

    // Assert manufacturer
    let res = client.read_point(model1::Mn).await?;
    assert_eq!(res, "SMA");

    // Assert max power
    let w_rtg = client.read_point(model120::WRtg).await?;
    let w_rtg_sf = client.read_point(model120::WRtg_SF).await?;
    let scaled = utils::apply_scale_factor(w_rtg, w_rtg_sf);
    assert_eq!(scaled, 10_000);

    return Ok(());
}
