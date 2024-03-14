use substreams::Hex;

use crate::pb::curve::types::v1::{Pool, Token};

impl Pool {
    pub fn address_vec(&self) -> Vec<u8> {
        Hex::decode(&self.address).unwrap()
    }

    pub fn registry_address_vec(&self) -> Vec<u8> {
        Hex::decode(&self.registry_address).unwrap()
    }

    pub fn output_token_ref(&self) -> &Token {
        self.output_token.as_ref().unwrap()
    }

    // Collate input and output tokens into a single vector.
    pub fn get_all_tokens(&self) -> Vec<Token> {
        std::iter::once(self.output_token_ref().clone())
            .chain(self.input_tokens.clone().into_iter())
            .collect()
    }
}
