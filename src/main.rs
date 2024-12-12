use futures::stream::StreamExt;
use netlink_sys::{AsyncSocket, SocketAddr};
use rtnetlink::{
    constants::{
        RTMGRP_IPV4_IFADDR, RTMGRP_IPV4_ROUTE, RTMGRP_IPV6_IFADDR,
        RTMGRP_IPV6_ROUTE, RTMGRP_LINK,
    },
    new_connection,
};

#[tokio::main]
async fn main() -> Result<(), String> {
    let (mut connection, _, mut messages) =
        new_connection().map_err(|e| format!("{e}"))?;

    let mgroup_flags = RTMGRP_LINK
        | RTMGRP_IPV4_IFADDR
        | RTMGRP_IPV4_ROUTE
        | RTMGRP_IPV6_IFADDR
        | RTMGRP_IPV6_ROUTE;

    let addr = SocketAddr::new(0, mgroup_flags);

    connection
        .socket_mut()
        .socket_mut()
        .bind(&addr)
        .expect("failed to bind");
    tokio::spawn(connection);

    while let Some((message, _)) = messages.next().await {
        let payload = message.payload;
        println!("Route change message - {payload:?}");
    }

    Ok(())
}
