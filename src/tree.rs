use sha3::{Digest, Sha3_256};

const HASH_LENGTH: usize = 32;

pub type Hash = [u8; HASH_LENGTH];

/// A dynamically growable array represented merkle tree.
///
/// The left most branch of the tree consists of progressively increasing powers
/// of two. The right child of each power of two looks like a traditionally
/// indexed binary tree offset by its parent.
///
/// The underlying storage is a 1-indexed dynamically growable array that is
/// always a power of two in length. The tree is built succesively from the
/// bottom left to the top right.
///
/// The zeroth index of the underlying storage is used to store the number of
/// leaves in the tree.
///
/// ```markdown
///           8
///     4            9
///  2     5     10     11
/// 1  3  6  7  12 13 14 15
/// ```
#[derive(Debug)]
pub struct MerkleTree {
    nodes: Vec<Hash>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MerkleProof(pub Vec<(bool, Hash)>);

impl MerkleTree {
    pub fn root(&self) -> Hash {
        self.nodes[1]
    }

    pub fn new(depth: usize, initial_leaf_value: Hash) -> Self {
        let num_nodes = Self::num_nodes(depth);
        let mut nodes = vec![initial_leaf_value; num_nodes];

        let mut intial_value_at_depth = vec![initial_leaf_value; depth + 1];

        // Update from leaf up to root
        for i in (0..depth).rev() {
            intial_value_at_depth[i] =
                concat(&intial_value_at_depth[i + 1], &intial_value_at_depth[i + 1]);
        }

        // The values at every layer of the merkle tree will all be the same
        for i in 0..num_nodes {
            let depth = Self::depth_at_index(i);

            nodes[i] = intial_value_at_depth[depth];
        }

        MerkleTree { nodes }
    }

    // This hashing solution recomputes the hash for every leaf in the Merkle tree
    // For a tree of depth 20 that's 2^20 = 1048576 hashes! This is incredibly inefficient.
    // We can take advantage of the fact that all leaves are initalized with the same value.
    // We only need to do depth number of hashes
    fn hash_recursive(nodes: &mut [Hash], index: usize) {
        let left_child = Self::left_child(index);

        if left_child >= nodes.len() {
            return;
        }

        let right_child = Self::right_child(index);

        Self::hash_recursive(nodes, left_child);
        Self::hash_recursive(nodes, right_child);

        nodes[index] = concat(&nodes[left_child], &nodes[right_child]);
    }

    pub fn set(&mut self, leaf_index: usize, value: Hash) {
        let index = self.nodes.len() / 2 + leaf_index;

        self.nodes[index] = value;

        Self::set_recursively(&mut self.nodes, index);
    }

    fn set_recursively(nodes: &mut [Hash], index: usize) {
        let Some(parent) = Self::index_of_parent(index) else {
            return;
        };

        let sibling = Self::index_of_sibling(index).unwrap();
        let is_left = index % 2 == 0;

        if is_left {
            nodes[parent] = concat(&nodes[index], &nodes[sibling]);
        } else {
            nodes[parent] = concat(&nodes[sibling], &nodes[index]);
        }

        Self::set_recursively(nodes, parent);
    }

    pub fn proof(&self, leaf_index: usize) -> MerkleProof {
        let index = self.nodes.len() / 2 + leaf_index;
        let mut proof = vec![];
        Self::proof_recursively(&mut proof, &self.nodes, index);

        MerkleProof(proof)
    }

    fn proof_recursively(proof: &mut Vec<(bool, Hash)>, nodes: &[Hash], index: usize) {
        let Some(parent) = Self::index_of_parent(index) else {
            return;
        };

        let sibling = Self::index_of_sibling(index).unwrap();
        let is_left = index % 2 == 0;

        proof.push((is_left, nodes[sibling]));

        Self::proof_recursively(proof, nodes, parent)
    }

    pub fn verify(proof: &MerkleProof, leaf_value: Hash) -> Hash {
        let mut hash = leaf_value;

        for (is_left, sibling) in proof.0.iter() {
            if *is_left {
                hash = concat(&hash, sibling);
            } else {
                hash = concat(sibling, &hash);
            }
        }

        hash
    }

    pub fn num_leaves(&self) -> usize {
        self.nodes.len() / 2
    }

    fn num_leaves_from_depth(depth: usize) -> usize {
        1 << depth as u32
    }

    fn num_nodes(depth: usize) -> usize {
        // typically num nodes is equal to 2^(depth + 1) - 1
        // but we're using 1 based indexing in this case
        1 << (depth + 1) as u32
    }

    fn left_child(index: usize) -> usize {
        index * 2
    }

    fn right_child(index: usize) -> usize {
        (index * 2) + 1
    }

    fn depth_at_index(index: usize) -> usize {
        if index <= 1 {
            return 0;
        }

        index.ilog2() as usize
    }

    fn index_of_sibling(index: usize) -> Option<usize> {
        if index <= 1 {
            return None;
        }

        if index % 2 == 0 {
            Some(index + 1)
        } else {
            Some(index - 1)
        }
    }

    fn index_of_parent(index: usize) -> Option<usize> {
        if index <= 1 {
            return None;
        }

        Some(index / 2)
    }
}

fn concat(a: &Hash, b: &Hash) -> Hash {
    let mut hasher = Sha3_256::new();

    hasher.update(a);
    hasher.update(b);

    hasher.finalize().into()
}

mod tests {
    use super::*;
    use const_hex;
    use hex_literal::hex;
    use test_case::test_case;

    #[test_case(5 => hex!("b8b1810f54c4048913090d78983712bd54cd4bae4e236be1f294122388abef6b"))]
    #[test_case(7 => hex!("90029acbe3254c63bc9dd4a8f1e4b8e27b4445bb5e5a5897af9251ec744f6f68"))]
    #[test_case(19 => hex!("d4490f4d374ca8a44685fe9471c5b8dbe58cdffd13d30d9aba15dd29efb92930"))]
    fn initial_root(depth: usize) -> Hash {
        let initial_leaf =
            hex_literal::hex!("abababababababababababababababababababababababababababababababab");
        let tree = MerkleTree::new(depth, initial_leaf);

        let root = tree.root();

        println!("root = {}", const_hex::encode(root));

        root
    }

    fn u32_to_hash(u: u32) -> Hash {
        let mut hash = [0u8; 32];
        hash[0..4].copy_from_slice(&u.to_le_bytes());

        hash
    }

    #[test]
    fn set_root() {
        const INITIAL: Hash =
            hex_literal::hex!("0000000000000000000000000000000000000000000000000000000000000000");

        let mut tree = MerkleTree::new(2, INITIAL);

        const NEW_VALUE: Hash =
            hex!("1111111111111111111111111111111111111111111111111111111111111111");

        tree.set(0, NEW_VALUE);

        // Manually calculated root Hash
        let hash_1 = concat(&NEW_VALUE, &INITIAL);
        let hash_2 = concat(&INITIAL, &INITIAL);
        let hash_0 = concat(&hash_1, &hash_2);

        assert_eq!(tree.root(), hash_0);
    }

    #[test]
    fn proof_of_first_leaf() {
        const INITIAL: Hash =
            hex_literal::hex!("0000000000000000000000000000000000000000000000000000000000000000");
        let mut tree = MerkleTree::new(2, INITIAL);

        let initial_2 = concat(&INITIAL, &INITIAL);
        let proof = tree.proof(0);

        let manual_proof = MerkleProof(vec![(true, INITIAL), (true, initial_2)]);
        assert_eq!(proof, manual_proof);
    }

    #[test]
    fn proof_of_last_leaf() {
        const INITIAL: Hash =
            hex_literal::hex!("0000000000000000000000000000000000000000000000000000000000000000");
        let mut tree = MerkleTree::new(2, INITIAL);

        let initial_2 = concat(&INITIAL, &INITIAL);
        let proof = tree.proof(3);

        let manual_proof = MerkleProof(vec![(false, INITIAL), (false, initial_2)]);
        assert_eq!(proof, manual_proof);
    }

    #[test]
    fn proof() {
        const INITIAL: Hash =
            hex_literal::hex!("0000000000000000000000000000000000000000000000000000000000000000");
        let leaves = [
            u32_to_hash(0),
            u32_to_hash(1),
            u32_to_hash(2),
            u32_to_hash(3),
        ];

        let mut tree = MerkleTree::new(2, INITIAL);

        for (i, leaf) in leaves.iter().enumerate() {
            tree.set(i, *leaf);
        }

        let proof = tree.proof(2);

        let sibling_1 = leaves[3];
        let sibling_2 = concat(&leaves[0], &leaves[1]);

        let manual_proof = MerkleProof(vec![(true, sibling_1), (false, sibling_2)]);
        assert_eq!(proof, manual_proof);
    }

    #[test]
    fn proof_and_verify() {
        const INITIAL: Hash =
            hex_literal::hex!("0000000000000000000000000000000000000000000000000000000000000000");

        let mut tree = MerkleTree::new(10, INITIAL);

        for i in 0..tree.num_leaves() {
            tree.set(i, u32_to_hash(i as u32));
        }

        let proof = tree.proof(tree.num_leaves() / 2);

        let root = tree.root();
        let verify_root = MerkleTree::verify(&proof, u32_to_hash(tree.num_leaves() as u32 / 2));

        assert_eq!(root, verify_root);
    }
}
