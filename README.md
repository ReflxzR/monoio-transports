# monoio-transports-netreq-fork

Fork of the original [library](https://github.com/monoio-rs/monoio-transports/tree/master) with few changes (specific for netreq crate).

## Main Changes

- Re-exported the original monoio-transports crate, in case you want to use the default package source
- support for connection pool from user specified parameters for the default HttpConnector
- Feature flags to use monoio's legacy + tokio features or disable io-uring
- TLS support for Hyper connectors. Supports both native-tls and rustls
- Added build script to prevent incompatible feature flag combinations