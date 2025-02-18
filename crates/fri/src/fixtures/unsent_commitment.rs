use alloc::vec;
use funvec::FunVec;
use starknet_crypto::Felt;

use super::*;
use crate::types::UnsentCommitment;

pub fn get() -> UnsentCommitment {
    UnsentCommitment {
        inner_layers: FunVec::from_vec(vec![
            Felt::from_hex_unchecked(
                "0x31b917291bbb3d38f7bc196dee1f3638ca197512162a4bdeb1ce814619c1625",
            ),
            Felt::from_hex_unchecked(
                "0x6945b2895872a701b3451cdf93dca9cba3cad8f250d5866ca5c263e41c8f2b2",
            ),
            Felt::from_hex_unchecked(
                "0x786c3ebbd4cab0c782d36860cd51887712953c48ce72c8d10acf5698c5a1213",
            ),
            Felt::from_hex_unchecked(
                "0x1e9b0fa29ebe52b9c9a43a1d44e555ce42da3199370134d758735bfe9f40269",
            ),
        ]),
        last_layer_coefficients: FunVec::from_vec(last_layer_coefficients::get()),
    }
}
