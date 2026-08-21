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
impl AsyncWrite for FragmentingStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let this = self.get_mut();
 
        if !this.first_write_done && buf.len() > 1 {
            this.first_write_done = true;
 
            let half = buf.len() / 2;
            let first_part = &buf[..half];
 
            Pin::new(&mut this.inner).poll_write(cx, first_part)
        } else {
            Pin::new(&mut this.inner).poll_write(cx, buf)
        }
    }
}
