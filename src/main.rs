use clap::{App, Arg};
use tokio::net::UdpSocket;
use tokio_util::udp::UdpFramed;
use futures::{FutureExt, SinkExt, stream, StreamExt};

use crate::protocol::Codec;
use crate::protocol::request::Request;
use std::net::{SocketAddr, ToSocketAddrs};

mod protocol;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Michael Ripley <zkxs00@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("SESSION_ID")
            .required(true)
            .help("sets the session ID to check")
            .index(1))
        .get_matches();

    let matchmaker_addr: SocketAddr = "matchx.centralus.cloudapp.azure.com:12500".to_socket_addrs().unwrap().next().expect("No DNS results");
    let session_id = matches.value_of("SESSION_ID").expect("Session ID was missing");

    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    let framed = UdpFramed::new(socket, Codec::default());
    let (mut tx, rx) = framed.split();

    let request = Request::nat_punch("".into(), 0, "".into());

    todo!();

    tx.send((request, matchmaker_addr)).await.expect("failed to send nat punch");
    drop(tx);

    // TODO: consume from rx

    println!("Hello, world! {}", session_id);
}

struct Foo {}
