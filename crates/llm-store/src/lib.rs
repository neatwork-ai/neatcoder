pub mod job;
pub mod msg;
pub mod msg_node;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Id([u8; 32]);

impl AsRef<[u8]> for Id {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

type MsgId = Id;
type JobId = Id;
