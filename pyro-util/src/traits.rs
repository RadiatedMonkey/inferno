use bytes::{Bytes, BytesMut};
use crate::bytes::{ReadBuffer, WriteBuffer};

use crate::Result;

/// Trait that describes an object that can be serialised from raw bytes.
pub trait Serialize {
    /// Serializes the object into binary format.
    fn serialize(&self, buffer: &mut BytesMut);
}

/// Trait that describes an object that can be deserialised from raw bytes.
pub trait Deserialize {
    /// Deserializes the given buffer, returning the object.
    fn deserialize(buffer: Bytes) -> Result<Self>
    where
        Self: Sized;
}
