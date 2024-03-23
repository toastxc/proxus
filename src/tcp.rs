use crate::Result;
use futures::future::try_join_all;
use std::net::{SocketAddr, ToSocketAddrs};
use tokio::{
    io::{self, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener, TcpStream,
    },
};

/// Copies data between TCP sender and receiver
/// this has a simple 'try again' function
pub async fn future(
    mut read: OwnedReadHalf,
    mut write: OwnedWriteHalf,
) -> tokio::task::JoinHandle<Result<()>> {
    tokio::spawn(async move {
        io::copy(&mut read, &mut write).await?;
        write.shutdown().await?;
        Ok(())
    })
}
pub async fn proxy(src: TcpStream, dst_addr: SocketAddr) -> Result<()> {
    let dst = TcpStream::connect(dst_addr).await?;

    let (src_rd, src_wr) = src.into_split();
    let (dst_rd, dst_wr) = dst.into_split();

    try_join_all(vec![
        future(src_rd, dst_wr).await,
        future(dst_rd, src_wr).await,
    ])
    .await?;

    Ok(())
}

pub async fn connection(a1: impl Into<String>, a2: impl Into<String>) -> Result<()> {
    let src_addr = address(&a1.into())?;
    let dst_addr = address(&a2.into())?;

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
            if let Err(error) = proxy(client, dst_addr).await {
                println!("WARNING: {}", error);
            }
        });
    }
}

/// Resolves An domain name or IP address into a socket address
/// this is needed as SocketAddr does not support TLDs.
fn address(input: &str) -> Result<SocketAddr> {
    if let Ok(mut ip_iter) = input.to_socket_addrs() {
        if let Some(ip) = ip_iter.next() {
            return Ok(ip);
        }
    };
    Ok(input.parse::<SocketAddr>()?)
}
