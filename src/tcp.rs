use crate::Result;
use futures::future::try_join_all;
use std::net::SocketAddr;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;

pub type ResultFuture = tokio::task::JoinHandle<Result<()>>;

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

async fn future(mut read: OwnedReadHalf, mut write: OwnedWriteHalf) -> ResultFuture {
    tokio::spawn(async move {
        io::copy(&mut read, &mut write).await?;
        write.shutdown().await?;
        Ok(())
    })
}
