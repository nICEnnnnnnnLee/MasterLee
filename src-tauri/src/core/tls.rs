use std::convert::TryFrom;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tokio::io::split;
use tokio::io::{ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;

use tokio_rustls::rustls::internal::msgs::codec::{ Reader};
use tokio_rustls::rustls::internal::msgs::handshake::{
      ConvertServerNameList, 
    HandshakePayload,  
};
use tokio_rustls::rustls::internal::msgs::message::{
    Message, MessagePayload, OpaqueMessage, 
};
use tokio_rustls::rustls::{
    self, ClientConfig, HandshakeType, OwnedTrustAnchor,  RootCertStore,
};

struct NoCertVerifier {}

impl rustls::client::ServerCertVerifier for NoCertVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}

pub async fn connect(
    dst_addr: &str,
    dst_port: u16,
    sni: &str,
    allow_insecure: bool,
) -> io::Result<(
    ReadHalf<tokio_rustls::client::TlsStream<tokio::net::TcpStream>>,
    WriteHalf<tokio_rustls::client::TlsStream<tokio::net::TcpStream>>,
)> {
    let addr = (dst_addr, dst_port)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;

    let mut root_store = RootCertStore::empty();
    root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));
    let mut config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    if allow_insecure {
        config
            .dangerous()
            .set_certificate_verifier(Arc::new(NoCertVerifier {}));
    }

    let connector = TlsConnector::from(Arc::new(config));
    let stream = TcpStream::connect(&addr).await?;

    let domain = rustls::ServerName::try_from(sni)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid dnsname"))?;

    let stream = connector.connect(domain, stream).await?;
    // stream.write_all(content.as_bytes()).await?;

    // let (mut reader, mut writer) = split(stream);

    Ok(split(stream))
}

pub fn get_sni(head: &[u8]) -> Option<String> {
    let mut rd = &mut Reader::init(head);
    let m = OpaqueMessage::read(&mut rd).ok()?;
    let m = Message::try_from(m.into_plain_message()).ok()?;
    if let MessagePayload::Handshake { parsed, .. } = m.payload {
        if parsed.typ == HandshakeType::ClientHello {
            if let HandshakePayload::ClientHello(chp) = parsed.payload {
                if let Some(server_names) = chp.get_sni_extension() {
                    if let Some(sni) = server_names.get_single_hostname() {
                        if let Ok(host) = String::from_utf8(sni.as_ref().to_vec()) {
                            return Some(host);
                        }
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn parse_sni() {
        let client_hello_bytes = [
            22, 3, 1, 2, 0, 1, 0, 1, 252, 3, 3, 24, 114, 181, 99, 0, 41, 224, 161, 129, 229, 127,
            232, 88, 47, 97, 13, 246, 151, 107, 206, 184, 84, 23, 157, 26, 56, 135, 46, 163, 129,
            5, 157, 32, 114, 228, 153, 252, 252, 37, 5, 166, 39, 12, 139, 115, 203, 250, 91, 104,
            94, 245, 96, 46, 230, 58, 81, 238, 8, 64, 199, 245, 152, 25, 30, 187, 0, 30, 19, 1, 19,
            3, 19, 2, 192, 43, 192, 47, 204, 169, 204, 168, 192, 44, 192, 48, 192, 19, 192, 20, 0,
            156, 0, 157, 0, 47, 0, 53, 1, 0, 1, 149, 0, 0, 0, 18, 0, 16, 0, 0, 13, 98, 108, 111,
            103, 46, 99, 115, 100, 110, 46, 110, 101, 116, 0, 23, 0, 0, 255, 1, 0, 1, 0, 0, 10, 0,
            14, 0, 12, 0, 29, 0, 23, 0, 24, 0, 25, 1, 0, 1, 1, 0, 11, 0, 2, 1, 0, 0, 35, 0, 0, 0,
            16, 0, 14, 0, 12, 2, 104, 50, 8, 104, 116, 116, 112, 47, 49, 46, 49, 0, 5, 0, 5, 1, 0,
            0, 0, 0, 0, 34, 0, 8, 0, 6, 4, 3, 5, 3, 6, 3, 0, 51, 0, 107, 0, 105, 0, 29, 0, 32, 63,
            215, 144, 139, 252, 47, 160, 28, 25, 194, 230, 0, 180, 116, 227, 209, 166, 80, 39, 18,
            15, 238, 118, 197, 240, 118, 110, 171, 158, 182, 116, 71, 0, 23, 0, 65, 4, 227, 111,
            199, 32, 90, 216, 32, 245, 43, 139, 210, 88, 142, 253, 166, 232, 202, 34, 101, 126,
            206, 116, 104, 99, 52, 158, 109, 248, 92, 116, 45, 82, 204, 118, 179, 43, 185, 11, 140,
            84, 136, 56, 205, 77, 110, 24, 18, 41, 4, 28, 83, 230, 33, 180, 51, 57, 123, 113, 93,
            167, 234, 43, 22, 148, 0, 43, 0, 5, 4, 3, 4, 3, 3, 0, 13, 0, 22, 0, 20, 4, 3, 5, 3, 6,
            3, 8, 4, 8, 5, 8, 6, 4, 1, 5, 1, 6, 1, 2, 1, 0, 45, 0, 2, 1, 1, 0, 28, 0, 2, 64, 1, 0,
            21, 0, 145, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ];
        let sni = get_sni(&client_hello_bytes[..]);
        assert_eq!(Some("blog.csdn.net".to_string()), sni);
    }
}
