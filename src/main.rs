use futures::future::try_join_all;
use proxus::data::Conf;
use proxus::Result;
use std::{
    env,
    net::{SocketAddr, ToSocketAddrs},
};
use tokio::net::TcpListener;
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        &args[1]
    } else {
        println!("you need to specify a path for the configuraion");
        return;
    };

    let conf = Conf::from_file(path).unwrap();

    println!("generating connections...");

    let mut tasks = Vec::new();

    if let Some(reconnect) = conf.reconnect {
        let mut counter = 0;
        let mut check = true;
        conf.data.into_iter().for_each(|a| {
            tasks.push(tokio::spawn(async move {
                while check {
                    if let Err(err) = connection(a.a1.clone(), a.a2.clone()).await {
                        println!("Error processing item\nerror: {}\nconfig: {:?}", err, a);
                    } else if reconnect.reset_after_success.is_some_and(|i| i) {
                        counter = 0;
                    };

                    let retry = std::time::Duration::from_secs(reconnect.retry_time.unwrap_or(5));
                    let retry_s = retry.as_secs();

                    println!("retrying in {} second{}", retry_s, {
                        if retry_s > 1 {
                            "s"
                        } else {
                            ""
                        }
                    });
                    tokio::time::sleep(retry).await;

                    if let Some(count) = reconnect.watchdog_timer {
                        counter += 1;
                        if counter >= count {
                            println!("failed reattempts, exiting thread");
                            check = false;
                        }
                    }
                }
            }));
        });
    } else {
        conf.data.into_iter().for_each(|a| {
            tasks.push(tokio::spawn(async move {
                if let Err(err) = connection(a.a1.clone(), a.a2.clone()).await {
                    println!("Error processing item\nerror: {}\nconfig: {:?}", err, a);
                };
            }));
        });
    }

    // Await the completion of all tasks
    if let Err(err) = try_join_all(tasks).await {
        println!("THREAD ERROR: {:?}", err);
    }
}
async fn connection(a1: String, a2: String) -> Result<()> {
    let src_addr = address(&a1)?;

    let dst_addr = address(&a2)?;

    let listener = TcpListener::bind(src_addr).await?;

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
        let (client, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(womp) = proxus::tcp::proxy(client, dst_addr).await {
                println!("WARNING: {}", womp);
            }
        });
    }
}


/// Resolves An domain name or IP address into a socket address
/// this is needed as SocketAddr does not support TLDs.
fn address(input: &str) -> Result<SocketAddr> {
    if let Ok(mut ip_iter) = input.to_socket_addrs() {
        while let Some(ip) = ip_iter.next() {
            return Ok(ip);
        };
    };
    Ok(input.parse::<SocketAddr>()?)
}
