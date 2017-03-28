use config;

struct DummyCommonConfig {}

impl config::AvailableAlgorithms for DummyCommonConfig {
    fn get_available_kex_algorithms(&self) -> Vec<String> {
        vec!["diffie-hellman-group1-sha1".to_string(), "diffie-hellman-group14-sha1".to_string()]
    }

    fn get_available_server_host_key_algorithms(&self) -> Vec<String> {
        vec!["ssh-dss".to_string()]
    }

    fn get_available_encryption_algorithms_client_to_server(&self) -> Vec<String> {
        vec!["3des-cbc".to_string()]
    }

    fn get_available_encryption_algorithms_server_to_client(&self) -> Vec<String> {
        self.get_available_encryption_algorithms_client_to_server()
    }

    fn get_available_mac_algorithms_client_to_server(&self) -> Vec<String> {
        vec!["hmac-sha1".to_string()]
    }

    fn get_available_mac_algorithms_server_to_client(&self) -> Vec<String> {
        self.get_available_mac_algorithms_client_to_server()
    }

    fn get_available_compression_algorithms_client_to_server(&self) -> Vec<String> {
        vec!["none".to_string()]
    }

    fn get_available_compression_algorithms_server_to_client(&self) -> Vec<String> {
        self.get_available_compression_algorithms_client_to_server()
    }

    fn get_available_languages_client_to_server(&self) -> Vec<String> {
        vec!["en_US".to_string()]
    }

    fn get_available_languages_server_to_client(&self) -> Vec<String> {
        self.get_available_languages_client_to_server()
    }
}
