pub fn version() -> Vec<u8> {
    b"0.0.0a".to_vec()
}

pub fn get_version_byte_string() -> Vec<u8> {
    return [b"SSH-2.0-bssh_".to_vec(), version()].concat();
}
