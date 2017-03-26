use config;

struct DummyCommonConfig {}

impl config::CommonConfig for DummyCommonConfig {

    fn get_available_kex_algorithms(&self) -> Vec<&'static str> {
        vec!("diffie-hellman-group1-sha1", "diffie-hellman-group14-sha1")
    }

    fn get_available_host_key_algorithms(&self) -> Vec<&'static str> {
        vec!("ssh-dss")
    }

    fn get_available_encryption_algorithms(&self) -> Vec<&'static str> {
        vec!("3des-cbc")
    }

    fn get_available_mac_algorithms(&self) -> Vec<&'static str> {
        vec!("hmac-sha1")
    }

    fn get_available_compression_methods(&self) -> Vec<&'static str> {
        vec!("none")
    }

    fn get_available_languages(&self) -> Vec<&'static str> {
        vec!("en_US")
    }
}
