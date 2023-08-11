use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("GnosisSafe2", "abis/gnosis_safe_l2_v1_3_0.json")?
        .generate()?
        .write_to_file("src/abi/gnosis_safe.rs")?;

    Ok(())
}