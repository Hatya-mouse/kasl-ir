# kasl-ir

`kasl-ir` is an intermediate representation (IR) for KASL language. KASL language is first lowered into this KASL-IR, and then translated for various backends, such as Cranelift.
If you are considering to use KASL in your project, consider visiting the `kasl` crate, which is an umbrella crate and includes other re-exports of other related crates.
