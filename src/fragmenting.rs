use tokio::net::TcpStream;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
pub struct FragmentingStream {
    inner: TcpStream,
    first_write_done: bool,
}
impl FragmentingStream {
    pub fn new(inner: TcpStream) -> Self {
        Self{inner, first_write_done: false}
    }
}
impl AsyncRead for FragmentingStream {
    fn poll_read(
        self: Pin<&mut self>, 
        cx: Context<'_>, 
        buf: &[u8]
    ) -> Poll<std::io::Result<usize>>{
        todo!()
    }
}
