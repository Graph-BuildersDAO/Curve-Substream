use crate::pb::curve::types::v1::{Pool, Token};

impl Pool {
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
