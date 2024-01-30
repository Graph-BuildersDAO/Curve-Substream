use substreams::Hex;

use crate::pb::curve::types::v1::Token;

impl Token {
    pub fn address_vec(&self) -> Vec<u8> {
        Hex::decode(&self.address).unwrap()
    }
}
