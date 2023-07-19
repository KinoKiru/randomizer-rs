// Lmao macro
pub mod rules {
    /// Init a new route for api, pass the Server and then the implemented Service for it to work
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
