use starknet_crypto::Felt;
use swiftness_air::{domains::StarkDomains, layout::LayoutTrait, public_memory::PublicInput};
use swiftness_commitment::{table::decommit::table_decommit, CacheCommitment};
use swiftness_fri::{
    fri::{self, fri_verify},
    types,
};

use crate::{
    oods::{eval_oods_boundary_poly_at_points, OodsEvaluationInfo},
    queries::queries_to_points,
    types::{CacheStark, StarkCommitment, StarkWitness},
};

// STARK verify phase.
#[inline(always)]
pub fn stark_verify<Layout: LayoutTrait>(
    cache: &mut CacheStark,
    n_original_columns: u32,
    n_interaction_columns: u32,
    public_input: &PublicInput,
    queries: &[Felt],
    commitment: &StarkCommitment<Layout::InteractionElements>,
    witness: &mut StarkWitness,
    stark_domains: &StarkDomains,
) -> Result<(), Error> {
    // First layer decommit.
    Layout::traces_decommit(
        &mut cache.commitment,
        queries,
        &commitment.traces,
        &witness.traces_decommitment,
        &witness.traces_witness,
    )?;

    table_decommit(
        &mut cache.commitment,
        &commitment.composition,
        queries,
        &witness.composition_decommitment,
        &witness.composition_witness,
    )?;

    let CacheCommitment { points, eval_oods, .. } = &mut cache.commitment;

    // Compute query points.
    let points =
        queries_to_points(points.unchecked_slice_mut(queries.len()), queries, stark_domains);

    // Evaluate the FRI input layer at query points.
    let eval_info = OodsEvaluationInfo {
        oods_values: &commitment.oods_values,
        oods_point: &commitment.interaction_after_composition,
        trace_generator: &stark_domains.trace_generator,
        constraint_coefficients: &commitment.interaction_after_oods,
    };
    let oods_poly_evals = eval_oods_boundary_poly_at_points::<Layout>(
        eval_oods,
        n_original_columns,
        n_interaction_columns,
        public_input,
        &eval_info,
        &points,
        &witness.traces_decommitment,
        &witness.composition_decommitment,
    );

    // Decommit FRI.
    let fri_decommitment = types::DecommitmentRef { values: oods_poly_evals, points };
    fri_verify(
        &mut cache.fri,
        queries,
        &commitment.fri,
        &fri_decommitment,
        &mut witness.fri_witness,
    )?;

    Ok(())
}

#[cfg(feature = "std")]
use thiserror::Error;

#[cfg(feature = "std")]
#[derive(Error, Debug)]
pub enum Error {
    #[error("Fri Error")]
    FriError(#[from] fri::Error),

    #[error("TraceDecommit Error")]
    TraceDecommitError(#[from] swiftness_air::trace::decommit::Error),

    #[error("TableDecommit Error")]
    TableDecommitError(#[from] swiftness_commitment::table::decommit::Error),
}

#[cfg(not(feature = "std"))]
use thiserror_no_std::Error;

#[cfg(not(feature = "std"))]
#[derive(Error, Debug)]
pub enum Error {
    #[error("Fri Error")]
    FriError(#[from] fri::Error),

    #[error("TraceDecommit Error")]
    TraceDecommitError(#[from] swiftness_air::trace::decommit::Error),

    #[error("TableDecommit Error")]
    TableDecommitError(#[from] swiftness_commitment::table::decommit::Error),
}
