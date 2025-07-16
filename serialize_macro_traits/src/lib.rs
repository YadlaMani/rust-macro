use std::fmt::Error;

pub trait Serialize {
	fn serialize(&self) -> Vec<u8>;
}

pub trait Deserialize: Sized {
	fn deserialize(data: &[u8]) -> Result<Self, Error>;
}