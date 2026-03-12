//! This example demonstrates a standard procedure for setting up a wireguard
//! device, equivalent to the following:
//!
//! ```sh
//! ip link add dev wg-example type wireguard
//! ip addr add 10.0.0.1 dev wg-example
//! wg set wg-example peer AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA= endpoint 127.0.0.1:12345 allowed-ips 10.0.0.1/32
//! ip link del dev wg-example
//! ```
//!
//! Run with: `cargo run --example wireguard-setup --features=wireguard,rt-link,rt-addr`

use std::net::{IpAddr, SocketAddr};

use netlink_bindings::{
    rt_addr::{self, Ifaddrmsg},
    rt_link::{self, Ifinfomsg},
    wireguard,
};
use netlink_socket2::NetlinkSocket;

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
#[cfg_attr(feature = "tokio", tokio::main(flavor = "current_thread"))]
#[cfg_attr(feature = "smol", macro_rules_attribute::apply(smol_macros::main))]
async fn main() {
    let mut sock = NetlinkSocket::new();

    let ifname = "wg-example";
    let addr: IpAddr = "10.0.0.1".parse().unwrap();
    let prefix = 32;
    let peer_key = [0u8; wireguard::KEY_LEN as usize];
    let peer: SocketAddr = "127.0.0.1:12345".parse().unwrap();

    println!("Adding {ifname:?}");
    link_add(&mut sock, ifname).await;

    let ifindex = link_get_ifindex(&mut sock, ifname).await;
    println!("Device {ifname:?} received ifindex {ifindex}");

    println!("Assigning {ifname:?} address {addr}/{prefix}");
    addr_add(&mut sock, ifindex, addr, prefix).await;

    println!("Configuring wireguard parameters");
    wg_set(&mut sock, ifname, addr, peer, &peer_key[..]).await;

    println!("Dumping wireguard devices (all)");
    wg_dump(&mut sock).await;

    // Comment these lines to inspect manually from the console
    println!("Deleting {ifname:?}");
    link_del(&mut sock, ifname).await;
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn wg_dump(sock: &mut NetlinkSocket) {
    // Additional socket for handling wireguard requests
    let mut sock_wg = NetlinkSocket::new();

    // Consecutive requests can reuse the same buffer using [`Request::new_with_buf`],
    // you don't need to .clean() the buffer yourself
    let mut buf = Vec::new();

    let request_links = rt_link::Request::new().op_getlink_dump(&Ifinfomsg::new());
    let mut iter = sock.request(&request_links).await.unwrap();
    while let Some(res) = iter.recv().await {
        let (_header, attrs) = res.unwrap();

        let link = attrs.get_ifname().unwrap();

        if Ok(c"wireguard") != attrs.get_linkinfo().unwrap_or_default().get_kind() {
            println!("Skipping {link:?}");
            continue;
        };
        println!("Dumping {link:?}");

        let mut request = wireguard::Request::new_with_buf(&mut buf).op_get_device_dump();
        request.encode().push_ifname(link);

        let mut iter = sock_wg.request(&request).await.unwrap();
        while let Some(res) = iter.recv().await {
            let attrs = res.unwrap();
            println!("{:#?}", attrs);
        }

        println!();
    }
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn wg_set(
    sock: &mut NetlinkSocket,
    ifname: &str,
    addr: IpAddr,
    peer: SocketAddr,
    peer_key: &[u8],
) {
    let mut request = wireguard::Request::new().op_set_device_do();

    request
        .encode()
        .push_ifname_bytes(ifname.as_bytes())
        .push_flags(wireguard::WgdeviceFlags::ReplacePeers as u32) // Remove existing peers
        .array_peers()
        .entry_nested()
        .push_public_key(peer_key)
        .push_endpoint(peer)
        .array_allowedips()
        .entry_nested()
        .push_family(libc_addr_family(&addr) as u16)
        .push_ipaddr(addr)
        .push_cidr_mask(32)
        .end_nested()
        .end_array()
        .end_nested()
        .end_array();

    let mut iter = sock.request(&request).await.unwrap();
    iter.recv_ack().await.unwrap();
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn addr_add(sock: &mut NetlinkSocket, ifindex: u32, addr: IpAddr, addr_prefix: u8) {
    let header = Ifaddrmsg {
        ifa_family: libc_addr_family(&addr) as u8,
        ifa_index: ifindex,
        ifa_prefixlen: addr_prefix,
        ..Default::default()
    };

    let mut request = rt_addr::Request::new()
        .set_change() // Don't fail if address already assigned
        .op_newaddr_do(&header);

    request.encode().push_local(addr);

    let mut iter = sock.request(&request).await.unwrap();
    iter.recv_ack().await.unwrap();
}

/// Equivalent to `ip link add dev {ifname} type wireguard`
#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn link_add(sock: &mut NetlinkSocket, ifname: &str) {
    let mut request = rt_link::Request::new()
        .set_create()
        // .set_excl() // If exclusive flag set, existing device will cause an error
        .op_newlink_do(&rt_link::Ifinfomsg::new());

    request
        .encode()
        .push_ifname_bytes(ifname.as_bytes())
        .nested_linkinfo()
        .push_kind(c"wireguard");

    let mut iter = sock.request(&request).await.unwrap();
    iter.recv_ack().await.unwrap();
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn link_get_ifindex(sock: &mut NetlinkSocket, ifname: &str) -> u32 {
    let mut request = rt_link::Request::new().op_getlink_do(&rt_link::Ifinfomsg::new());

    request.encode().push_ifname_bytes(ifname.as_bytes());

    let mut iter = sock.request(&request).await.unwrap();
    let (header, _attrs) = iter.recv_one().await.unwrap();

    header.ifi_index as u32
}

/// Equivalent to `ip link del dev {ifname}`
#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn link_del(sock: &mut NetlinkSocket, ifname: &str) {
    let mut request = rt_link::Request::new().op_dellink_do(&Default::default());

    request.encode().push_ifname_bytes(ifname.as_bytes());

    let mut iter = sock.request(&request).await.unwrap();
    iter.recv_ack().await.unwrap();
}

fn libc_addr_family(addr: &IpAddr) -> i32 {
    if addr.is_ipv4() {
        libc::AF_INET
    } else {
        libc::AF_INET6
    }
}
