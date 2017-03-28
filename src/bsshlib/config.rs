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

//TODO move it from here
pub struct KexMessage {
	pub cookie : [u8; 16],
    pub kex_algorithms: Vec<String>,
    pub server_host_key_algorithms: Vec<String>,
    pub encryption_algorithms_client_to_server: Vec<String>,
    pub encryption_algorithms_server_to_client: Vec<String>,
    pub mac_algorithms_client_to_server: Vec<String>,
    pub mac_algorithms_server_to_client: Vec<String>,
    pub compression_algorithms_client_to_server: Vec<String>,
    pub compression_algorithms_server_to_client: Vec<String>,
    pub languages_client_to_server: Vec<String>,
    pub languages_server_to_client: Vec<String>,
    pub first_kex_packet_follows : bool
}

impl KexMessage {
	fn get_cookie(&self) -> [u8; 16] {
		self.cookie
	}
	fn get_first_kex_packet_follows(&self) -> bool {
		self.first_kex_packet_follows
	}
}

impl AvailableAlgorithms for KexMessage {
	fn get_available_kex_algorithms(&self) -> Vec<String> {
		self.kex_algorithms.clone()
	}
    fn get_available_server_host_key_algorithms(&self) -> Vec<String> {
    	self.server_host_key_algorithms.clone()
    }
    fn get_available_encryption_algorithms_client_to_server(&self) -> Vec<String> {
    	self.encryption_algorithms_client_to_server.clone()
    }
    fn get_available_encryption_algorithms_server_to_client(&self) -> Vec<String> {
    	self.encryption_algorithms_server_to_client.clone()
    }
    fn get_available_mac_algorithms_client_to_server(&self) -> Vec<String> {
    	self.mac_algorithms_client_to_server.clone()
    }
    fn get_available_mac_algorithms_server_to_client(&self) -> Vec<String> {
    	self.mac_algorithms_server_to_client.clone()
    }
    fn get_available_compression_algorithms_client_to_server(&self) -> Vec<String> {
    	self.compression_algorithms_client_to_server.clone()
    }
    fn get_available_compression_algorithms_server_to_client(&self) -> Vec<String> {
    	self.compression_algorithms_server_to_client.clone()
    }
    fn get_available_languages_client_to_server(&self) -> Vec<String> {
    	self.languages_client_to_server.clone()
    }
    fn get_available_languages_server_to_client(&self) -> Vec<String> {
    	self.languages_server_to_client.clone()
    }
}