use dusk_plonk::prelude::*;

pub(crate) const LABEL_TRANSCRIPT: &[u8; 14] = b"maki-arguments";

#[derive(Debug, Default)]
pub(crate) struct MakiCircuit {
    // private inputs
    pub a: BlsScalar,
    pub b: BlsScalar,
    pub c: BlsScalar,
    pub d: BlsScalar,
    pub hashed_private_key: BlsScalar,
    // public inputs
    pub new_state_root: BlsScalar,
    pub public_key: BlsScalar,
}
// TODO : change checks on circuit and inputs (both public and private)
// Implement a circuit that checks:
// 1) a + b = c where C is a PI
// 2) a <= 2^6
// 3) b <= 2^5
// 4) a * b = d where D is a PI
// TODO

impl Circuit for MakiCircuit {
    fn circuit<C>(&self, composer: &mut C) -> Result<(), Error>
    where
        C: Composer,
    {
        let a = composer.append_witness(self.a);
        let b = composer.append_witness(self.b);

        // Make first constraint a + b = c
        let constraint = Constraint::new().left(1).right(1).public(-self.c).a(a).b(b);

        composer.append_gate(constraint);

        // Check that a and b are in range
        composer.component_range(a, 1 << 6);
        composer.component_range(b, 1 << 5);

        // Make second constraint a * b = d
        let constraint = Constraint::new().mult(1).public(-self.d).a(a).b(b);

        composer.append_gate(constraint);

        // let e = composer.append_witness(self.e);
        // let scalar_mul_result =
        //     composer.component_mul_generator(e, dusk_jubjub::GENERATOR_EXTENDED)?;

        // // Apply the constraint
        // composer.assert_equal_public_point(scalar_mul_result, self.f);

        let new_state_root = composer.append_public(self.new_state_root);
        let public_key = composer.append_public(self.public_key);

        Ok(())
    }
}
