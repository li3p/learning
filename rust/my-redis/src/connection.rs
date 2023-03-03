use bytes::BytesMut;
use tokio::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            buffer: BytesMut::with_capacity(4096),
        }
    }

    use tokio::io::AsyncReadExt;
    use bytes::Buf;
    use mini_redis::Result;

    pub async fn read_frame(&mut self) ->Result<Option<Frame>> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await?{
                if self.buffer.is_empty(){
                    return Ok(None);
                } else {
                    return Err("connection reseted by peer".into());
                }
            }
        }
    }

    use mini_redis::{Frame, Result};
    use mini_redis::frame::Error::Incomplete;
    use std::io::Cursor;

    fn parse_frame(&mut self){
        let mut buf = Cursor::new(&self.buffer[..]);

        match Frame::check(&mut buf){
            Ok(_) => {
                let len = buf.position() as usize;
                buf.set_position(0);
                let frame = Frame::parse(&mut buf)?;
                self.buffer.advance(len);
                Ok(Some(frame))
            }
            Err(Incomplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
