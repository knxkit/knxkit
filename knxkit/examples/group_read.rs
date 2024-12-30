use std::{net::Ipv4Addr, str::FromStr, time::Duration};

use knxkit::{
    connection::{self, ops::GroupOps, KnxBusConnection, RemoteSpec},
    core::address::GroupAddress,
};

use knxkit_dpt::{
    specific::{SpecificDataPoint, DPT_9_1},
    typeinfo,
};

#[tokio::main]
async fn main() {
    let remote = RemoteSpec::KnxIpTunnel("192.168.8.2:3671".parse().unwrap());
    let local = Ipv4Addr::from_str("192.168.7.51").unwrap();

    let mut tunnel = connection::connect(local, &remote).await.unwrap();

    let group = GroupAddress::from_str("2/0/0").unwrap();
    let timeout = Duration::from_secs(1);

    let data_point = tunnel.group_read(group, timeout).await.unwrap();

    let temp = DPT_9_1::from_data_point(&data_point).unwrap();
    let info = typeinfo::lookup(DPT_9_1::DPT).unwrap();

    println!("{}: {}{}", info.text.unwrap(), temp.0, info.unit.unwrap());
    // -> temperature (°C): 19.5°C

    tunnel.terminate().await
}
