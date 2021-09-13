use std::net::{SocketAddr, ToSocketAddrs};

use clap::{App, Arg};
use futures::{SinkExt, StreamExt};
use tokio::net::UdpSocket;
use tokio::time::Duration;
use tokio_util::udp::UdpFramed;

use crate::protocol::Codec;
use crate::protocol::request::Request;
use crate::protocol::response::Response;

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

    let matchmaker_addr: SocketAddr = "matchx.centralus.cloudapp.azure.com:12500"
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("No DNS results for matchmaker");

    let session_id = matches.value_of("SESSION_ID").expect("Session ID was missing");

    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    let port = socket.local_addr().unwrap().port();
    let mut socket = UdpFramed::new(socket, Codec::default());
    let request = Request::nat_punch("169.254.203.224".into(), port.into(), format!("C;{}", session_id));

    println!("sending request on port {} for {}", port, session_id);
    socket.send((request, matchmaker_addr)).await.expect("failed to send nat punch");

    println!("sent request! awaiting response...");

    let (response, _) = socket.next().await
        .expect("some matchmaker result")
        .expect("no matchmaker error");

    println!("got response:");

    match response {
        Response::NatPunch(r) => {
            println!("{:?}", r);

            let host_addr: SocketAddr = format!("{}:{}", r.remote_host, r.remote_port)
                .to_socket_addrs()
                .unwrap()
                .next()
                .expect("No DNS results for session host");

            let request = Request::connect(session_id.to_lowercase());

            println!("sending followup connect to {}...", host_addr);
            socket.send((request, host_addr)).await.expect("failed to send connect");
            println!("got response:");

            let (response, _) = socket.next().await
                .expect("some connect result")
                .expect("no connect error");

            match response {
                Response::NatPunch(r) => println!("{:?}", r),
                Response::NatPunchError(r) => println!("{:?}", r),
                Response::Connect(r) => println!("{:?}", r),
                Response::Unknown(r) => println!("{:?}", r),
            }
        }
        Response::NatPunchError(r) => println!("{:?}", r),
        Response::Connect(r) => println!("{:?}", r),
        Response::Unknown(r) => println!("{:?}", r),
    }

    // check for additional response
    println!("waiting for additional response...");
    let (response, _) = socket.next().await
        .expect("some connect result")
        .expect("no connect error");

    match response {
        Response::NatPunch(r) => println!("{:?}", r),
        Response::NatPunchError(r) => println!("{:?}", r),
        Response::Connect(r) => println!("{:?}", r),
        Response::Unknown(r) => println!("{:?}", r),
    }


    let mut looping = true;
    while looping {
        // check for additional response
        println!("waiting for additional response...");
        let future = tokio::time::timeout(Duration::from_millis(750), socket.next());

        match future.await {
            Ok(result) => {
                let (response, _) = result
                    .expect("some connect result")
                    .expect("no connect error");

                match response {
                    Response::NatPunch(r) => println!("{:?}", r),
                    Response::NatPunchError(r) => println!("{:?}", r),
                    Response::Connect(r) => println!("{:?}", r),
                    Response::Unknown(r) => println!("{:?}", r),
                }
            }
            Err(_) => {
                // timeout occurred
                looping = false;
            }
        }
    }
    println!("Done.");
}
