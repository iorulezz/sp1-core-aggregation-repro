#![no_main]

sp1_zkvm::entrypoint!(main);

fn main() {
    let payload = sp1_zkvm::io::read::<u32>();
    let proof_vk_hash = sp1_zkvm::io::read::<[u32; 8]>();
    let proof_pv_hash = sp1_zkvm::io::read::<[u8; 32]>();

    sp1_zkvm::lib::verify::verify_sp1_proof(&proof_vk_hash, &proof_pv_hash);
    sp1_zkvm::io::commit(&payload);
    sp1_zkvm::io::commit(&proof_vk_hash);
    sp1_zkvm::io::commit(&proof_pv_hash);
}
