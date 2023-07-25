use sha2::{Digest, Sha256};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct IdHash([u8; 32]);

impl AsRef<[u8]> for IdHash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
// TODO: impl derefmut
impl std::ops::Deref for IdHash {
    // type Target = Rc<T>;
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct SmallHash(u64);

impl std::ops::Deref for SmallHash {
    // type Target = Rc<T>;
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IdHash {
    pub fn truncate(hash: &Self) -> SmallHash {
        SmallHash(truncate_(hash))
    }

    pub fn hash_element(element: &str) -> Self {
        Self(hash_element_(element))
    }

    pub fn order_invariant_hash(elements: &[&str]) -> Self {
        let sum: u64 = elements
            .iter()
            .map(|&element| truncate_(&hash_element_(element)))
            .sum();
        let mut hasher = Sha256::new();
        hasher.update(sum.to_le_bytes());
        Self(hasher.finalize().into())
    }
}

fn hash_element_(element: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(element.as_bytes());
    hasher.finalize().into()
}

fn truncate_(hash: &[u8; 32]) -> u64 {
    let mut value: u64 = 0;
    for i in 0..8 {
        value = (value << 8) | (hash[i] as u64);
    }
    value
}
