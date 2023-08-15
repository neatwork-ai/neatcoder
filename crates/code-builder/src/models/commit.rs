use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub type JobID = HashID;
pub type Commit = SmallHash;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, Deserialize, Serialize)]
pub struct HashID(pub [u8; 32]);

// References interior
impl AsRef<[u8]> for HashID {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

// TODO: impl derefmut
impl std::ops::Deref for HashID {
    // type Target = Rc<T>;
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct SmallHash(u64);

// References interior
impl AsRef<u64> for SmallHash {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

impl std::ops::Deref for SmallHash {
    // type Target = Rc<T>;
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HashID {
    pub fn generate_random() -> Self {
        // Generate random data
        let random_data: [u8; 32] = rand::thread_rng().gen();

        // Hash the random data
        let hash = Sha256::digest(&random_data);

        // Convert the resulting hash into [u8; 32] and return
        HashID(hash.into())
    }
}

impl HashID {
    pub fn truncate(&self) -> SmallHash {
        SmallHash(truncate_(self))
    }

    pub fn hash_element(element: &str) -> Self {
        Self(hash_element_(element.as_bytes()))
    }

    // pub fn order_invariant_hash_str(elements: &[&str]) -> Self {
    //     let sum: u64 = elements
    //         .iter()
    //         .map(|&element| truncate_(&hash_element_(element)))
    //         .sum();
    //     let mut hasher = Sha256::new();
    //     hasher.update(sum.to_le_bytes());
    //     Self(hasher.finalize().into())
    // }

    pub fn order_invariant_hash(elements: &[&[u8]]) -> Self {
        let sum: u64 = elements
            .iter()
            .map(|&element| truncate_(&hash_element_(element)))
            .sum();
        let mut hasher = Sha256::new();
        hasher.update(sum.to_le_bytes());
        Self(hasher.finalize().into())
    }

    pub fn order_invariant_hash_vec(elements: &Vec<HashID>) -> Self {
        let sum: u64 = elements
            .iter()
            .map(|&element| truncate_(&hash_element_(element.as_ref())))
            .sum();
        let mut hasher = Sha256::new();
        hasher.update(sum.to_le_bytes());
        Self(hasher.finalize().into())
    }
}

fn hash_element_(element: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(element);
    hasher.finalize().into()
}

fn truncate_(hash: &[u8; 32]) -> u64 {
    let mut value: u64 = 0;
    for i in 0..8 {
        value = (value << 8) | (hash[i] as u64);
    }
    value
}
