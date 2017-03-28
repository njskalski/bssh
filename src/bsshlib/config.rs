pub trait CommonConfig {}

pub trait ClientConfig {}

pub trait ServerConfig {}

pub trait AvailableAlgorithms {
    fn get_available_kex_algorithms(&self) -> Vec<String>;
    fn get_available_server_host_key_algorithms(&self) -> Vec<String>;    
    fn get_available_encryption_algorithms_client_to_server(&self) -> Vec<String>;
    fn get_available_encryption_algorithms_server_to_client(&self) -> Vec<String>;
    fn get_available_mac_algorithms_client_to_server(&self) -> Vec<String>;
    fn get_available_mac_algorithms_server_to_client(&self) -> Vec<String>;

    fn get_available_compression_algorithms_client_to_server(&self) -> Vec<String>;
    fn get_available_compression_algorithms_server_to_client(&self) -> Vec<String>;
    fn get_available_languages_client_to_server(&self) -> Vec<String>;
    fn get_available_languages_server_to_client(&self) -> Vec<String>;
}

