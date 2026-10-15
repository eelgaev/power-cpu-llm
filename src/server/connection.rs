use std::{
    io,
    net::SocketAddr,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    task::{Context, Poll},
    time::Duration,
};
use tokio::{
    io::{AsyncRead, AsyncWrite, ReadBuf},
    net::{TcpListener, TcpStream},
};

#[derive(Debug, Clone)]
pub struct Connection {
    pub cancelled: Arc<AtomicBool>,
}
impl axum::extract::connect_info::Connected<axum::serve::IncomingStream<'_, CancelListener>>
    for Connection
{
    fn connect_info(stream: axum::serve::IncomingStream<'_, CancelListener>) -> Self {
        stream.remote_addr().clone()
    }
}

pub struct ConnectionIo {
    stream: TcpStream,
    cancelled: Arc<AtomicBool>,
}
impl Drop for ConnectionIo {
    fn drop(&mut self) {
        self.cancelled.store(true, Ordering::Release);
    }
}
impl AsyncRead for ConnectionIo {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.stream).poll_read(cx, buf)
    }
}
impl AsyncWrite for ConnectionIo {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.stream).poll_write(cx, buf)
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.stream).poll_flush(cx)
    }
    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.stream).poll_shutdown(cx)
    }
}
pub struct CancelListener(pub(crate) TcpListener);
impl CancelListener {
    pub async fn bind(bind: SocketAddr) -> io::Result<Self> {
        Ok(Self(TcpListener::bind(bind).await?))
    }
}
impl axum::serve::Listener for CancelListener {
    type Io = ConnectionIo;
    type Addr = Connection;
    async fn accept(&mut self) -> (Self::Io, Self::Addr) {
        loop {
            let accept = self.0.accept().await.and_then(|(stream, _)| {
                let original = stream.into_std()?;
                let monitor = original.try_clone()?;
                Ok((TcpStream::from_std(original)?, monitor))
            });
            match accept {
                Ok((stream, monitor)) => {
                    let cancelled = Arc::new(AtomicBool::new(false));
                    let flag = cancelled.clone();
                    tokio::spawn(async move {
                        while !flag.load(Ordering::Acquire) {
                            match monitor.peek(&mut [0u8; 1]) {
                                Ok(0) => {
                                    flag.store(true, Ordering::Release);
                                    break;
                                }
                                Err(error)
                                    if !matches!(
                                        error.kind(),
                                        io::ErrorKind::WouldBlock | io::ErrorKind::Interrupted
                                    ) =>
                                {
                                    flag.store(true, Ordering::Release);
                                    break;
                                }
                                _ => {}
                            }
                            tokio::time::sleep(Duration::from_millis(100)).await;
                        }
                    });
                    return (
                        ConnectionIo {
                            stream,
                            cancelled: cancelled.clone(),
                        },
                        Connection { cancelled },
                    );
                }
                Err(error) => {
                    eprintln!("HTTP accept error: {error}");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }
    fn local_addr(&self) -> io::Result<Self::Addr> {
        self.0.local_addr()?;
        Ok(Connection {
            cancelled: Arc::new(AtomicBool::new(false)),
        })
    }
}
