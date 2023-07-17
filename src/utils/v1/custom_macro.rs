// Lmao macro
pub mod rules {
    macro_rules! initialize_route {
        ($server:ident, $service:ident) => {
            pub fn server() -> $server<$service> {
                $server::new($service::default())
                    .send_compressed(tonic::codec::CompressionEncoding::Gzip)
                    .accept_compressed(tonic::codec::CompressionEncoding::Gzip)
            }
        };
    }

    pub(crate) use initialize_route;
}
