pub version : Vec<u8> = b"0.0.0a".to_vec();

pub get_version_byte_string() -> Vec<u8> {
    ("SSH-2.0-bssh_" + version)
}
