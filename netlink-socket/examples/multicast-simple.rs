//! This example demonstrates receiving Netlink multicast notifications emitted
//! by generic netlink subsystems, aka genetlink.
//!
//! Multicast notification are quite sparsely documented, so netlink-bindings
//! only provides a "raw" socket, meaning you have to provide `group_id` to
//! listen to and later to choose how to decode received messages yourself.
//!
//! Run with: `cargo run --example multicast-simple --features=netdev,rt-link`

use netlink_bindings::{builtin::BuiltinNfgenmsg, netdev, nlctrl, rt_link, traits::NetlinkRequest};
use netlink_socket2::{MulticastSocketRaw, NetlinkSocket};

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
#[cfg_attr(feature = "tokio", tokio::main(flavor = "current_thread"))]
#[cfg_attr(feature = "smol", macro_rules_attribute::apply(smol_macros::main))]
async fn main() {
    if !check_linux_at_least_6_3() {
        println!("Netdev notifications were only added in Linux 6.3. The current kernel is older");
        return;
    }

    let mut sock = NetlinkSocket::new();
    let mut multicast_sock = MulticastSocketRaw::new(nlctrl::PROTONUM).unwrap();

    // Before receiving message, you have to subscribe to relevant groups.
    // Looking at `<subsystem>.yaml` specification, those are usuall at the
    // bottom called "mcast-groups".
    //
    // Under the hood, .listen() calls setsockopt with NETLINK_ADD_MEMBERSHIP.
    let group_id =
        resolve_genl_group_id(&mut sock, netdev::PROTONAME, netdev::NotifGroup::MGMT).await;
    multicast_sock.listen(group_id).unwrap();

    // This should emit notifications for us to process
    let link = "example-link";
    link_add(&mut sock, link).await;
    link_del(&mut sock, link).await;

    loop {
        let (_recv, buf) = multicast_sock.recv().await.unwrap();

        let BuiltinNfgenmsg { cmd, version, .. } = BuiltinNfgenmsg::new_from_zeroed(buf);

        println!("Message cmd={cmd:?} version={version:?}");

        // netlink_bindings::utils::dump_hex(buf);

        let attrs = netdev::OpDevGetDo::decode_reply(buf);
        dbg!(attrs);

        // Cmd is a sequential number of an operation in operation.list[]
        let op = match cmd {
            netdev::OpDevAddNotif::CMD => Some("deleting"),
            netdev::OpDevDelNotif::CMD => Some("deleting"),
            netdev::OpDevChangeNotif::CMD => Some("changing"),
            _ => None,
        };

        let ifindex = attrs.get_ifindex().unwrap();
        if let Some(op) = op {
            println!("Cought {op} device with ifindex={ifindex}");
        }

        if std::env::var("TESTING").is_ok() && op == Some("deleting") {
            return;
        }
    }
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn resolve_genl_group_id(sock: &mut NetlinkSocket, faily: &str, group_name: &str) -> u32 {
    let mut request = nlctrl::Request::new().op_getfamily_do();
    request.encode().push_family_name_bytes(faily.as_bytes());

    let mut iter = sock.request(&request).await.unwrap();
    let attrs = iter.recv_one().await.unwrap();

    for group in attrs.get_mcast_groups().unwrap_or_default() {
        if group.get_name().unwrap().to_bytes() == group_name.as_bytes() {
            return group.get_id().unwrap();
        }
    }

    panic!("Couldn't resolve group id by group_name={group_name:?}")
}

/// Equivalent to `ip link add dev {ifname} type dummy`
#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn link_add(sock: &mut NetlinkSocket, ifname: &str) {
    let mut request = rt_link::Request::new()
        .set_create()
        .set_excl()
        .op_newlink_do(&rt_link::Ifinfomsg::new());

    request
        .encode()
        .push_ifname_bytes(ifname.as_bytes())
        .nested_linkinfo()
        .push_kind(c"dummy");

    let mut iter = sock.request(&request).await.unwrap();
    let _ = iter.recv_ack().await;
}

/// Equivalent to `ip link del dev {ifname}`
#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn link_del(sock: &mut NetlinkSocket, ifname: &str) {
    let mut request = rt_link::Request::new().op_dellink_do(&Default::default());

    request.encode().push_ifname_bytes(ifname.as_bytes());

    let mut iter = sock.request(&request).await.unwrap();
    let _ = iter.recv_ack().await;
}

fn check_linux_at_least_6_3() -> bool {
    let res = std::process::Command::new("uname")
        .arg("-r")
        .output()
        .unwrap();
    let ver = String::from_utf8(res.stdout).unwrap();
    let ver: Vec<u32> = ver
        .trim()
        .split(".")
        .take(2)
        .map(|c| c.parse().unwrap())
        .collect();
    return ver[0] > 6 || (ver[0] == 6 && ver[1] >= 3);
}
