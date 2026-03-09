use anyhow::{
    anyhow,
    Result,
};
use sha2::{
    Digest,
    Sha256,
};
use sp1_sdk::{
    blocking::{
        ProveRequest as _,
        Prover as _,
        ProverClient,
    },
    include_elf,
    utils::setup_logger,
    HashableKey,
    ProvingKey as _,
    SP1Proof,
    SP1Stdin,
};

const FIBONACCI_ELF: sp1_sdk::Elf = include_elf!("repro_fibonacci_program");
const AGGREGATOR_ELF: sp1_sdk::Elf = include_elf!("repro_aggregator_program");
const FIBONACCI_N: u32 = 20;
const DUMMY_PAYLOAD: u32 = 7;

fn main() -> Result<()> {
    setup_logger();

    tracing::info!("initializing cpu prover");
    let client = ProverClient::builder().cpu().build();

    tracing::info!("setting up fibonacci proving key");
    let fibonacci_pk = client.setup(FIBONACCI_ELF)?;

    tracing::info!("setting up aggregator proving key");
    let aggregator_pk = client.setup(AGGREGATOR_ELF)?;

    let mut fibonacci_stdin = SP1Stdin::new();
    fibonacci_stdin.write(&FIBONACCI_N);

    tracing::info!(
        n = FIBONACCI_N,
        "proving inner fibonacci program in compressed mode"
    );
    let fibonacci_proof = client
        .prove(&fibonacci_pk, fibonacci_stdin)
        .compressed()
        .run()?;

    let fibonacci_vk_hash = fibonacci_pk.verifying_key().hash_u32();
    let fibonacci_pv_hash: [u8; 32] = Sha256::digest(fibonacci_proof.public_values.to_vec()).into();
    let fibonacci_vk = fibonacci_pk.verifying_key().vk.clone();

    let SP1Proof::Compressed(fibonacci_reduce_proof) = fibonacci_proof.proof else {
        return Err(anyhow!("expected inner proof to be compressed"));
    };

    let mut aggregator_stdin = SP1Stdin::new();
    aggregator_stdin.write(&DUMMY_PAYLOAD);
    aggregator_stdin.write(&fibonacci_vk_hash);
    aggregator_stdin.write(&fibonacci_pv_hash);
    aggregator_stdin.write_proof(*fibonacci_reduce_proof, fibonacci_vk);

    tracing::info!("attempting outer aggregation proof in core mode");
    let _outer_proof = client
        .prove(&aggregator_pk, aggregator_stdin)
        .core() // if this is compressed, proving will work
        .run()?;

    Ok(())
}
