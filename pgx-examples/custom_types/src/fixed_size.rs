use pgx::cstr_core::CStr;
use pgx::*;
use std::str::FromStr;

#[derive(Copy, Clone, PostgresType)]
#[pgvarlena_inoutfuncs]
pub struct FixedF32Array {
    array: [f32; 91],
}

impl PgVarlenaInOutFuncs for FixedF32Array {
    fn input(input: &CStr) -> PgVarlena<Self> {
        let mut result = PgVarlena::<Self>::new();

        for (i, value) in input.to_bytes().split(|b| *b == b',').enumerate() {
            result.array[i] =
                f32::from_str(unsafe { std::str::from_utf8_unchecked(value) }).expect("invalid f32")
        }

        result
    }

    fn output(&self, buffer: &mut StringInfo) {
        self.array.iter().for_each(|v| {
            if buffer.len() > 0 {
                buffer.push(',');
            }
            buffer.push_str(&v.to_string());
        });
    }
}

#[pg_operator(immutable, parallel_safe)]
#[opname(<#>)]
fn fixedf32array_distance(left: PgVarlena<FixedF32Array>, right: PgVarlena<FixedF32Array>) -> f64 {
    left.array
        .iter()
        .zip(right.array.iter())
        .map(|(a, b)| ((a - b) * (a - b)) as f64)
        .sum()
}

#[pg_operator(immutable, parallel_safe)]
#[opname(+)]
fn fixedf32array_add(
    left: PgVarlena<FixedF32Array>,
    right: PgVarlena<FixedF32Array>,
) -> PgVarlena<FixedF32Array> {
    let mut new = PgVarlena::<FixedF32Array>::new();
    left.array
        .iter()
        .zip(right.array.iter())
        .map(|(a, b)| a + b)
        .zip(new.array.iter_mut())
        .for_each(|(a, b)| *b = a);

    new
}
