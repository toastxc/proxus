use futures::future::try_join_all;
use proxus::data::Conf;
use proxus::result::{Error, ErrorConvert};
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let path = "conf.toml";

    use std::env;

    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        &args[1]
    } else {
        println!("you need to specify a path for the configuraion");
        return;
    };

    let conf = Conf::from_file(path)
        .resm(&format!("invalid config, are you sure {path} is real?"))
        .unwrap();

    println!("generating connections...");

    println!("{:#?}", conf);

    let tasks = conf.data.into_iter().map(|a| {
        println!("new thread...");
        tokio::spawn(async move {
            if let Err(err) = connection(a.a1, a.a2).await {
                println!("Error processing item: {}", err);
            }
        })
    });

    // Await the completion of all tasks
    if let Err(err) = try_join_all(tasks).await {
        println!("THREAD ERROR: {:?}", err);
    }
}
async fn connection(a1: String, a2: String) -> Result<(), Error> {
    let src_addr = address(&a1)?;
    println!("target IP valid");

    let dst_addr = address(&a2)?;
    println!("connection IP valid");

    let listener = TcpListener::bind(src_addr).await.res()?;

    if src_addr.ip() == dst_addr.ip() {
        println!(
            "{} <-> {} | on {}",
            src_addr.port(),
            dst_addr.port(),
            src_addr.ip()
        )
    } else {
        println!("casting connection {} to {}", src_addr, dst_addr);
    }
    loop {
        let (client, _) = listener.accept().await.res()?;
        tokio::spawn(async move {
            if let Err(womp) = proxus::tcp::proxy(client, dst_addr).await {
                println!("WARNING: {}", womp);
            }
        });
    }
}

fn address(input: &str) -> Result<SocketAddr, Error> {
    if let Ok(mut ip_iter) = input.to_socket_addrs() {
        if let Some(ip) = ip_iter.next() {
            return Ok(ip);
        };
    }
    input.parse::<SocketAddr>().res()
}
