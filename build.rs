use vergen::EmitBuilder;

fn main() -> anyhow::Result<()>{
    EmitBuilder::builder().git_sha(true).git_sha(false).cargo_target_triple().emit()?;
    Ok(())
}