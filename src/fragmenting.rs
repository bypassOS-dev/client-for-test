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
        self: Pin<&mut Self>,        //It's need because rust look for move of object
        cx: &mut Context<'_>,        //This "Context" is "waker" 
        buf: &mut ReadBuf<'_>        //Just buffer for data
    ) -> Poll<std::io::Result<()>>{
        let this = self.get_mut();
        Pin::new(&mut this.inner).poll_read(cx, buf)
    }
}
