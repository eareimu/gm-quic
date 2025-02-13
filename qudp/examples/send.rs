use std::io::IoSlice;

use clap::{command, Parser};
use qudp::{PacketHeader, UdpSocketController};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = String::from("127.0.0.1:0"))]
    src: String,

    #[arg(long, default_value_t = String::from("127.0.0.1:12345"))]
    dst: String,

    #[arg(long, default_value_t = 1200)]
    msg_size: usize,

    #[arg(long, default_value_t = 64)]
    msg_count: usize,

    #[arg(long, default_value_t = true)]
    gso: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::level_filters::LevelFilter::INFO)
        // .with_max_level(tracing::level_filters::LevelFilter::TRACE)
        // .with_writer(
        //     std::fs::OpenOptions::new()
        //         .create(true)
        //         .write(true)
        //         .truncate(true)
        //         .open("/tmp/gm-quic.log")?,
        // )
        .with_ansi(false)
        .init();

    let args = Args::parse();
    let addr = args.src.parse().unwrap();
    let socket = UdpSocketController::new(addr).expect("failed to create socket");
    let dst = args.dst.parse().unwrap();

    let send_hdr = PacketHeader {
        src: socket.local_addr().expect("failed to get local addr"),
        dst,
        ttl: 64,
        ecn: Some(1),
        seg_size: args.msg_size as u16,
        gso: args.gso,
    };

    let payload = vec![8u8; args.msg_size];
    let batch = args.msg_count;
    for i in 0..batch {
        let payloads = vec![IoSlice::new(&payload[..]); 1];
        match socket.send(&payloads, send_hdr).await {
            Ok(n) => log::info!("sent {} packets, dest: {}", n, dst),
            Err(e) => log::error!("send failed: {}", e),
        }
    }

    let last = args.msg_count % 64;
    if last > 0 {
        let payloads = vec![IoSlice::new(&payload[..]); last];
        match socket.send(&payloads, send_hdr).await {
            Ok(n) => log::info!("sent {} packets, dest: {}", n, dst),
            Err(e) => log::error!("send failed: {}", e),
        }
    }
}
