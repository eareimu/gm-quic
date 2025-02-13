use clap::Parser;
use qudp::UdpSocketController;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short,long, default_value_t = String::from("127.0.0.1:12345"))]
    bind: String,
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
    let addr = args.bind.parse().unwrap();

    let socket = UdpSocketController::new(addr).expect("failed to create socket");
    let mut receiver = socket.receiver();
    loop {
        match receiver.recv().await {
            Ok(n) => {
                log::info!(
                    "received {} packets, dst {}, src {}",
                    n,
                    receiver.headers[0].dst,
                    receiver.headers[0].src
                );
            }
            Err(e) => {
                log::error!("receive failed: {}", e);
            }
        }
    }
}
