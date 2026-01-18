//! This example demonstrates basic usage of the API for traffic control
//! queueing disciplines. Given an existing network device (`wg0` by default),
//! it creates, displays, and deletes a tc prio qdisc with 8 bands, equivalent
//! to the following:
//!
//! ```sh
//! tc qdisc add dev wg0 root handle 0xbeef: prio bands 8 priomap 7 6 5 4 3 2 1 0
//! tc qdisc show dev wg0
//! tc qdisc del dev wg0 root
//! ```
//!
//! Run with: `cargo run --example tc-prio --features=tc,rt-link [-- ifname]`

use netlink_bindings::{
    rt_link,
    tc::{PushTcPrioQopt, PushTcmsg, Request},
};
use netlink_socket2::NetlinkSocket;

const TC_H_ROOT: u32 = 0xffffffff;

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
#[cfg_attr(feature = "tokio", tokio::main(flavor = "current_thread"))]
#[cfg_attr(feature = "smol", macro_rules_attribute::apply(smol_macros::main))]
async fn main() {
    let ifname = std::env::args().skip(1).next().unwrap_or("wg0".to_string());

    let mut sock = NetlinkSocket::new();
    let ifi = link_get_ifindex(&mut sock, &ifname).await;

    // Handle id consists of major:minor parts, each is 16 bits.
    // Qdisc handle has only the major part, minor is used by classes.
    let qdisc_handle = 0xbeef << 16;

    tc_prio_add(&mut sock, ifi, qdisc_handle).await;
    tc_prio_show(&mut sock, ifi, qdisc_handle).await;
    tc_prio_del(&mut sock, ifi, qdisc_handle).await;
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn tc_prio_add(sock: &mut NetlinkSocket, ifi: i32, handle: u32) {
    let mut header = PushTcmsg::new();

    header.set_family(0);
    header.set_ifindex(ifi);
    header.set_handle(handle);
    header.set_parent(TC_H_ROOT);

    let mut req = Request::new()
        .set_create()
        .set_excl()
        .op_newqdisc_do_request(&header);

    let priomap: [u8; 16] = [7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let mut tc_prio_opt = PushTcPrioQopt::new();
    tc_prio_opt.set_bands(8);
    tc_prio_opt.set_priomap(priomap);

    req.encode().nested_options_prio(&tc_prio_opt);

    let mut iter = sock.request(&req).await.unwrap();
    iter.recv_ack().await.unwrap();

    println!("tc prio add on ifi {} OK", ifi);
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn tc_prio_show(sock: &mut NetlinkSocket, ifi: i32, qdisc_handle: u32) {
    let mut header = PushTcmsg::new();

    header.set_family(0);
    header.set_ifindex(ifi);
    header.set_handle(qdisc_handle);
    header.set_parent(TC_H_ROOT);
    header.set_info(0);

    let req = Request::new().op_getqdisc_dump_request(&header);

    let mut iter = sock.request(&req).await.unwrap();
    while let Some(res) = iter.recv().await {
        let (header, attrs) = res.unwrap();

        if header.ifindex() == ifi && header.handle() == qdisc_handle {
            println!("{:#?}", (header, attrs));
            println!("tc prio show on ifi {} OK", ifi);
            return;
        }
    }

    panic!("No qdisc found");
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn tc_prio_del(sock: &mut NetlinkSocket, ifi: i32, qdisc_handle: u32) {
    let mut header = PushTcmsg::new();

    header.set_family(0);
    header.set_ifindex(ifi);
    header.set_handle(qdisc_handle);
    header.set_parent(TC_H_ROOT);
    header.set_info(0);

    let req = Request::new().op_delqdisc_do_request(&header);

    let mut iter = sock.request(&req).await.unwrap();
    iter.recv_ack().await.unwrap();

    println!("tc prio del on ifi {} OK", ifi);
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
pub async fn link_get_ifindex(sock: &mut NetlinkSocket, ifname: &str) -> i32 {
    let mut request = rt_link::Request::new().op_getlink_do_request(&Default::default());
    request.encode().push_ifname_bytes(ifname.as_bytes());

    let mut iter = sock.request(&request).await.unwrap();
    let (header, _attrs) = iter.recv_one().await.unwrap();

    header.ifi_index()
}
