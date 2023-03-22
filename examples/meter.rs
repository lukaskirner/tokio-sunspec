use std::{
    thread,
    time::Duration,
};
use tokio_sunspec::error::Error;
use tokio_sunspec::models::{model1, model213};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let socket_addr = "127.0.0.1:5502".parse().unwrap();
    let slave_id: u8 = 1;
    let start_addr: u16 = 40000;

    let mut client = tokio_sunspec::connect_tcp(socket_addr, slave_id, start_addr).await?;

    // Assert supported models
    let mut model_ids = Vec::from_iter(client.models.keys().cloned());
    model_ids.sort();
    assert_eq!(
        model_ids,
        vec![1, 213]
    );

    // Assert manufacturer
    let res = client.read_point(model1::Mn).await?;
    assert_eq!(res, "Manufactor");

    client.write_point(model1::DA, slave_id as u16).await?;
    let da = client.read_point(model1::DA).await?;
    let md = client.read_point(model1::Md).await?;
    let sn = client.read_point(model1::SN).await?;
    println!("Model 1 data:\n Device Addr: {}\n Model: {}\n Serial Number: {}", da, md, sn);

    // Assert max power
    let current = client.read_point(model213::A).await?;
    println!("Current: {}", current);
    let frequency = client.read_point(model213::Hz).await?;
    println!("Frequency: {}", frequency);
    let pf = client.read_point(model213::PF).await?;
    println!("Power Factor: {}", pf);

    loop {
        thread::sleep(Duration::from_secs(1));
        let current = client.read_point(model213::A).await?;
        println!("Current: {}", current);
    }
}