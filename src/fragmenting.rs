use tokio::net::TcpStream;

pub struct FragmentingStream {
    inner: TcpStream,
    first_write_done: bool,
}
impl FragmentingStream {
    pub fn new(inner: TcpStream) -> Self {
        Self{inner, first_write_done: false}
    }
}
