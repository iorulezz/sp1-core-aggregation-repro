#![no_main]

sp1_zkvm::entrypoint!(main);

fn main() {
    let payload = sp1_zkvm::io::read::<u32>();
    let proof_vk_hash = sp1_zkvm::io::read::<Option<[u32; 8]>>();

    if let Some(vk_hash) = proof_vk_hash {
        let proof_pv_hash = sp1_zkvm::io::read::<[u8; 32]>();
        sp1_zkvm::lib::verify::verify_sp1_proof(&vk_hash, &proof_pv_hash);
        sp1_zkvm::io::commit(&payload);
        sp1_zkvm::io::commit(&Some(vk_hash));
        sp1_zkvm::io::commit(&proof_pv_hash);
    } else {
        sp1_zkvm::io::commit(&payload);
        sp1_zkvm::io::commit(&None::<[u32; 8]>);
    }
}
