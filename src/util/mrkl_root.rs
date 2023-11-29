use crate::crypto::sha256;

const EMPTY: [u8; 32] = [0u8; 32];

fn hash_pair(lhs: &[u8; 32], rhs: &[u8; 32]) -> [u8; 32] {
    let mut buffer = [0u8; 64];

    buffer[0..32].copy_from_slice(lhs);
    buffer[32..64].copy_from_slice(rhs);

    sha256::hash(buffer)
}

pub fn calculate_mrkl_root(leaves: &[[u8; 32]]) -> [u8; 32] {
    let mut leaves = Vec::from(leaves);

    if leaves.len() == 0 {
        return EMPTY;
    }

    loop {
        match leaves[..] {
            [first] => return first,
            [.., last] if leaves.len() % 2 == 1 => {
                leaves.push(last);
                continue;
            }
            _ => {
                leaves = (0..leaves.len())
                    .filter(|i| i % 2 == 0)
                    .map(|i| hash_pair(&leaves[i], &leaves[i + 1]))
                    .into_iter()
                    .collect();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_root_on_predetermined_hashes() {
        let txn_hashes = vec![
            sha256::hash(b"aaa"),
            sha256::hash(b"bbb"),
            sha256::hash(b"ccc"),
            sha256::hash(b"ddd"),
        ];

        let mrkl_root = calculate_mrkl_root(&txn_hashes);
        let mrkl_root_digest = sha256::digest(&mrkl_root);

        let expected = "20d91ce8e5b46488788bee6b7b2dec6216168c5bf2e1dc484be420bad8462aa9";
        assert_eq!(mrkl_root_digest, expected);
    }
}