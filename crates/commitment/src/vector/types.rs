use cairo_felt::Felt252;
use serde::{Deserialize, Serialize};

// Commitment for a vector of field elements.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Commitment {
    config: Config,
    commitment_hash: Felt252,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    height: Felt252,
    n_verifier_friendly_commitment_layers: Felt252,
}

// A query to the vector commitment.
#[derive(Debug)]
pub struct Query {
    pub index: Felt252,
    pub value: Felt252,
}

// A query to the vector commitment that contains also the depth of the query in the Merkle tree.
#[derive(Debug)]
pub struct QueryWithDepth {
    pub index: Felt252,
    pub value: Felt252,
    pub depth: Felt252,
}

// Witness for a decommitment over queries.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Witness {
    // The authentication values: all the siblings of the subtree generated by the queried indices,
    // bottom layer up, left to right.
    authentications: Vec<Felt252>,
}
