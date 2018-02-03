use sha1::{Sha1, Digest};

//TODO rewrite?
pub fn hmac_sha1(inp: &Vec<u8>) -> Vec<u8> {
  let mut sh = Sha1::default();
  sh.input(inp);
  sh.result().to_vec()
}
