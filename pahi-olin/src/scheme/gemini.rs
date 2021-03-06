use crate::{error::Error, resource::Resource};
use log::error;
use openssl::ssl::{SslConnector, SslMethod, SslStream, SslVerifyMode};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use url::Url;

pub struct Gemini {
    stream: SslStream<TcpStream>,
}

impl Resource for Gemini {
    fn new(u: Url) -> Result<Gemini, Error> {
        if let None = u.host() {
            return Err(Error::InvalidArgument);
        }

        let host = u.host().unwrap().to_string();

        let port: u16 = match u.port() {
            Some(port_num) => port_num,
            None => 1965,
        };

        TcpStream::connect((host.as_str(), port))
            .or_else(|why| {
                error!(
                    "connection error to {}:{}: {:?}",
                    u.host().unwrap(),
                    port,
                    why
                );
                Err(Error::Unknown)
            })
            .and_then(|conn| {
                let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
                builder.set_verify(SslVerifyMode::NONE);
                let connector = builder.build();
                connector
                    .connect(host.as_str(), conn)
                    .or_else(|why| {
                        error!(
                            "error establishing TLS session for {}:{}: {:?}",
                            host, port, why
                        );
                        Err(Error::Unknown)
                    })
                    .and_then(|mut stream| {
                        stream.write(&u.to_string().as_bytes()).expect("send URL");
                        stream.write(&"\r\n".as_bytes()).expect("send /r/n");
                        Ok(Gemini { stream: stream })
                    })
            })
    }

    fn close(&mut self) {
        if let Err(why) = self.stream.shutdown() {
            error!("error closing TLS stream: {:?}", why);
        }

        drop(&self.stream);
    }
}

impl Read for Gemini {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf)
    }
}

impl Write for Gemini {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
