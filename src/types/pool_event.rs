use std::str::FromStr;

use substreams::{scalar::BigInt, Hex};

use crate::{
    constants::default_admin_fee,
    pb::curve::types::v1::events::{
        pool_event::{
            DepositEvent, LpTokenChange, SwapEvent, SwapUnderlyingLendingEvent,
            SwapUnderlyingMetaEvent, TokenAmount, WithdrawEvent,
        },
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

impl SwapUnderlyingMetaEvent {
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

    pub fn lp_token_change_ref(&self) -> &LpTokenChange {
        self.lp_token_change.as_ref().unwrap()
    }

    pub fn lp_token_change_amount_big(&self) -> BigInt {
        BigInt::from_str(self.lp_token_change_ref().amount.as_str()).unwrap()
    }
}

impl SwapUnderlyingLendingEvent {
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

    pub fn interest_bearing_token_in_action_ref(&self) -> &LpTokenChange {
        self.interest_bearing_token_in_action.as_ref().unwrap()
    }

    pub fn interest_bearing_token_in_action_amount_big(&self) -> BigInt {
        BigInt::from_str(self.interest_bearing_token_in_action_ref().amount.as_str()).unwrap()
    }

    pub fn interest_bearing_token_out_action_ref(&self) -> &LpTokenChange {
        self.interest_bearing_token_out_action.as_ref().unwrap()
    }

    pub fn interest_bearing_token_out_action_amount_big(&self) -> BigInt {
        BigInt::from_str(self.interest_bearing_token_out_action_ref().amount.as_str()).unwrap()
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
