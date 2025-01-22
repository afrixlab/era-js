use std::fmt::Display;
use std::fmt;

use super::ReedSolomon;

#[derive(Debug)]
pub enum Error {
    FragmentationError,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FragmentationError => write!(f, "Fragmentation error"),
        }
    }
}

pub fn encode_fragment(data: Vec<u8>, data_shards: usize, parity_shards: usize) -> Result<Vec<Vec<u8>>, Error> {
     if data.len() % data_shards != 0 {
        panic!("fragmentation error")
    }
    let reed_solomon = ReedSolomon::new(data_shards, parity_shards).unwrap();
    let size_per_shard = data.len() / data_shards;
    let mut shards = Vec::new();
    for i in 0..data_shards{
        shards.push(data[(size_per_shard * i)..size_per_shard * (i + 1)].to_vec());
    }
    (0..parity_shards).for_each(|_| shards.push(vec![0; size_per_shard]));
    reed_solomon.encode(&mut shards).map_err(|_| Error::FragmentationError)?;
    Ok(shards)
   
}
