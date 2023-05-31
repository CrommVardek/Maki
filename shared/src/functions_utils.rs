use dusk_bls12_381::BlsScalar;

use crate::types::{PublicKey, TreeRoot};

pub fn bytes_to_u64(bytes: [u8; 32]) -> [u64; 4] {
    let mut result = [0; 4];

    for i in 0..4 {
        let bytes_array = <&[u8; 8]>::try_from(&bytes[i * 8..(i + 1) * 8]).unwrap();
        result[i] = u64::from_be_bytes(*bytes_array);
    }

    result
}

pub fn bytes_to_scalar(bytes: [u8; 32]) -> BlsScalar {
    BlsScalar(bytes_to_u64(bytes))
}

pub fn generate_public_parameters(
    new_tree_root: &TreeRoot,
    public_key: &PublicKey,
    coordinator_public_key: &PublicKey,
) -> [u8; 96] {
    let mut public_parameters: [u8; 96] = [0; 96];

    let mut index: usize = 0;
    public_parameters[index..new_tree_root.len()].copy_from_slice(new_tree_root);
    index += new_tree_root.len();
    public_parameters[index..index + public_key.len()].copy_from_slice(public_key);
    index += public_key.len();
    public_parameters[index..index + coordinator_public_key.len()]
        .copy_from_slice(coordinator_public_key);

    public_parameters
}
