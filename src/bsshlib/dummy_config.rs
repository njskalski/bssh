use config;

struct DummyCommonConfig {}

impl config::CommonConfig for DummyCommonConfig {
    fn get_supported_mac_algorithms() -> Vec<&'static str> {
        vec!("hmac-sha1")
    }

    fn get_supported_key_exchange_methods() -> Vec<&'static str> {
        vec!("diffie-hellman-group1-sha1", "diffie-hellman-group14-sha1")
    }

    fn get_supported_compression_methods() -> Vec<&'static str> {
        vec!("none")
    }

    fn get_supported_key_certificate_formats() -> Vec<&'static str> {
        vec!("ssh-dss")
    }
}
