use std::str::FromStr;

use substreams::{scalar::BigInt, Hex};

use crate::{
    constants::default_admin_fee,
    pb::curve::types::v1::events::{
        pool_event::{DepositEvent, SwapEvent, SwapUnderlyingEvent, TokenAmount, WithdrawEvent},
        FeeChangeEvent,
    },
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

impl FeeChangeEvent {
    pub fn pool_address_vec(&self) -> Vec<u8> {
        Hex::decode(&self.pool_address).unwrap()
    }

    pub fn fee_big(&self) -> BigInt {
        BigInt::from_str(self.fee.as_str()).unwrap()
    }

    pub fn admin_fee_big(&self) -> BigInt {
        match &self.admin_fee {
            Some(fee) => BigInt::from_str(fee.as_str()).unwrap(),
            None => default_admin_fee(),
        }
    }
}
