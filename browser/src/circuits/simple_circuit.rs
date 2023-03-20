//! This circuit exists only for the purpose of benchmarking.

use halo2_proofs::arithmetic::{Field, FieldExt};
use halo2_proofs::circuit::{Chip, Layouter, SimpleFloorPlanner, Value};
use halo2_proofs::pasta::group::{Curve, Group};
use halo2_proofs::pasta::pallas;
use halo2_proofs::plonk::{Advice, Circuit, Column, ConstraintSystem, Error};
use rand_core::OsRng;

#[derive(Clone)]
pub struct SimpleCircuit {}

#[derive(Clone)]
pub struct SimpleConfig {
    advices: [Column<Advice>; 1],
}

impl Circuit<pallas::Base> for SimpleCircuit {
    type Config = SimpleConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {}
    }
    fn configure(meta: &mut ConstraintSystem<pallas::Base>) -> Self::Config {
        let advices = [meta.advice_column()];

        SimpleConfig { advices }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<pallas::Base>,
    ) -> Result<(), Error> {
        Ok(())
    }
}
