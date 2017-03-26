pub trait CommonConfig {
    fn get_available_kex_algorithms(&self) -> Vec<&'static str>;
    fn get_available_host_key_algorithms(&self) -> Vec<&'static str>;
    fn get_available_encryption_algorithms(&self) -> Vec<&'static str>;    
    fn get_available_mac_algorithms(&self) -> Vec<&'static str>;

    fn get_available_compression_methods(&self) -> Vec<&'static str>;
    fn get_available_languages(&self) -> Vec<&'static str>;
}

pub trait ClientConfig {

}

pub trait ServerConfig {

}
