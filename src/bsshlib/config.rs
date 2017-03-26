pub trait CommonConfig {
    fn get_supported_mac_algorithms() -> Vec<&'static str>;
    fn get_supported_key_exchange_methods() -> Vec<&'static str>;
    fn get_supported_compression_methods() -> Vec<&'static str>;
    fn get_supported_key_certificate_formats() -> Vec<&'static str>;
}

pub trait ClientConfig {

}

pub trait ServerConfig {

}
