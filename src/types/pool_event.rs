use std::str::FromStr;

use substreams::scalar::BigInt;

use crate::pb::curve::types::v1::events::pool_event::{
    DepositEvent, SwapEvent, SwapUnderlyingEvent, TokenAmount, WithdrawEvent,
};

impl DepositEvent {
    pub fn output_token_ref(&self) -> &TokenAmount {
        self.output_token.as_ref().unwrap()
    }

    pub fn output_token_amount_big(&self) -> BigInt {
        BigInt::from_str(self.output_token_ref().amount.as_str()).unwrap()
    }
}

impl WithdrawEvent {
    pub fn output_token_ref(&self) -> &TokenAmount {
        self.output_token.as_ref().unwrap()
    }

    pub fn output_token_amount_big(&self) -> BigInt {
        BigInt::from_str(self.output_token_ref().amount.as_str()).unwrap()
    }
}

impl SwapEvent {
    pub fn token_in_ref(&self) -> &TokenAmount {
        self.token_in.as_ref().unwrap()
    }

    pub fn token_in_amount_big(&self) -> BigInt {
        BigInt::from_str(self.token_in_ref().amount.as_str()).unwrap()
    }

    pub fn token_out_ref(&self) -> &TokenAmount {
        self.token_out.as_ref().unwrap()
    }

    pub fn token_out_amount_big(&self) -> BigInt {
        BigInt::from_str(self.token_out_ref().amount.as_str()).unwrap()
    }
}

impl SwapUnderlyingEvent {
    pub fn token_in_ref(&self) -> &TokenAmount {
        self.token_in.as_ref().unwrap()
    }

    pub fn token_in_amount_big(&self) -> BigInt {
        BigInt::from_str(self.token_in_ref().amount.as_str()).unwrap()
    }

    pub fn token_out_ref(&self) -> &TokenAmount {
        self.token_out.as_ref().unwrap()
    }

    pub fn token_out_amount_big(&self) -> BigInt {
        BigInt::from_str(self.token_out_ref().amount.as_str()).unwrap()
    }

    pub fn lp_token_burnt_ref(&self) -> &TokenAmount {
        self.lp_token_burnt.as_ref().unwrap()
    }

    pub fn lp_token_burnt_amount_big(&self) -> BigInt {
        BigInt::from_str(self.lp_token_burnt_ref().amount.as_str()).unwrap()
    }
}

impl TokenAmount {
    pub fn amount_big(&self) -> BigInt {
        BigInt::from_str(self.amount.as_str()).unwrap()
    }
}
