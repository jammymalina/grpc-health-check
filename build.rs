fn main() -> anyhow::Result<()> {
    tonic_prost_build::compile_protos("proto/health.proto")?;
    Ok(())
}
