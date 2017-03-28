use std;

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

pub struct AvailableAlgorithmSet {
	pub kex_algorithms: Vec<String>,
    pub server_host_key_algorithms: Vec<String>,
    pub encryption_algorithms_client_to_server: Vec<String>,
    pub encryption_algorithms_server_to_client: Vec<String>,
    pub mac_algorithms_client_to_server: Vec<String>,
    pub mac_algorithms_server_to_client: Vec<String>,
    pub compression_algorithms_client_to_server: Vec<String>,
    pub compression_algorithms_server_to_client: Vec<String>,
    pub languages_client_to_server: Vec<String>,
    pub languages_server_to_client: Vec<String>
}

pub struct KexMessage {
	pub cookie : [u8; 16],
	pub available_algorithm_set : AvailableAlgorithmSet,
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

impl AvailableAlgorithms for AvailableAlgorithmSet {
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

pub fn vector_intersection<T : std::clone::Clone + std::hash::Hash + std::cmp::Eq + std::cmp::Ord>(vec1 : Vec<T>, vec2 : Vec<T>) -> Vec<T> {
	use std::collections::BTreeSet;
	let a: BTreeSet<_> = vec1.iter().cloned().collect();
	let b: BTreeSet<_> = vec2.iter().cloned().collect();

	a.intersection(&b).cloned().collect::<Vec<_>>() //btree sorts by default
}

pub fn intersect_available_algorithms(set1 : &AvailableAlgorithms, set2 : &AvailableAlgorithms) -> AvailableAlgorithmSet {
	AvailableAlgorithmSet {
		kex_algorithms : vector_intersection::<_>(set1.get_available_kex_algorithms(), set2.get_available_kex_algorithms()),
		server_host_key_algorithms : vector_intersection::<_>(set1.get_available_server_host_key_algorithms(), set2.get_available_server_host_key_algorithms()),
		encryption_algorithms_client_to_server : vector_intersection::<_>(set1.get_available_encryption_algorithms_client_to_server(), set2.get_available_encryption_algorithms_client_to_server()),
		encryption_algorithms_server_to_client : vector_intersection::<_>(set1.get_available_encryption_algorithms_server_to_client(), set2.get_available_encryption_algorithms_server_to_client()),
		mac_algorithms_client_to_server : vector_intersection::<_>(set1.get_available_mac_algorithms_client_to_server(), set2.get_available_mac_algorithms_client_to_server()),
		mac_algorithms_server_to_client : vector_intersection::<_>(set1.get_available_mac_algorithms_server_to_client(), set2.get_available_mac_algorithms_server_to_client()),
		compression_algorithms_client_to_server : vector_intersection::<_>(set1.get_available_compression_algorithms_client_to_server(), set2.get_available_compression_algorithms_client_to_server()),
		compression_algorithms_server_to_client : vector_intersection::<_>(set1.get_available_compression_algorithms_server_to_client(), set2.get_available_compression_algorithms_server_to_client()),
		languages_client_to_server : vector_intersection::<_>(set1.get_available_languages_client_to_server(), set2.get_available_languages_client_to_server()),
		languages_server_to_client : vector_intersection::<_>(set1.get_available_languages_server_to_client(), set2.get_available_languages_server_to_client())
	}
}

#[cfg(test)]
mod tests {
	
	use super::*;

    #[test]
    fn vector_intersection_works() {
    	assert_eq!(vector_intersection(vec![1,2,3,4,5], vec![13,11,7,5,3]), vec![3,5]);
    }
}