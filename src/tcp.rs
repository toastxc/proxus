use futures::future::try_join_all;
use std::net::SocketAddr;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::result::{Error, ErrorConvert};

pub type ResultFuture = tokio::task::JoinHandle<Result<(), std::io::Error>>;

pub async fn proxy(src: TcpStream, dst_addr: SocketAddr) -> Result<(), Error> {
    let dst = TcpStream::connect(dst_addr).await.res()?;
    let (mut src_rd, mut src_wr) = src.into_split();
    let (mut dst_rd, mut dst_wr) = dst.into_split();

    let a: ResultFuture = tokio::spawn(async move {
        io::copy(&mut src_rd, &mut dst_wr).await?;
        dst_wr.shutdown().await?;
        Ok(())
    });

    let b: ResultFuture = tokio::spawn(async move {
        io::copy(&mut dst_rd, &mut src_wr).await?;
        src_wr.shutdown().await?;
        Ok(())
    });

    try_join_all(vec![a, b]).await.unwrap();

    Ok(())
}
