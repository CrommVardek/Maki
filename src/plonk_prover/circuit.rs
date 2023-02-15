use dusk_plonk::prelude::*;

#[derive(Debug, Default)]
pub(crate) struct MakiCircuit {
    // public inputs
    a: BlsScalar,
    b: BlsScalar,

    // private inputs
    c: BlsScalar,
    d: BlsScalar,
}

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

        let e = composer.append_witness(self.e);
        let scalar_mul_result =
            composer.component_mul_generator(e, dusk_jubjub::GENERATOR_EXTENDED)?;

        // Apply the constraint
        composer.assert_equal_public_point(scalar_mul_result, self.f);

        Ok(())
    }
}
