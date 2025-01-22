use std::vec;

use super::ReedSolomon;



pub fn encode_fragment(data: Vec<u8>, data_shards: usize, parity_shards: usize) {
    let mut reed_solomon = ReedSolomon::new(data_shards, parity_shards).unwrap();
    let size_per_shard = data.len() / data_shards;
    let mut shards = Vec::new();
    let mut counter = 0;
    for i in 0..data_shards{
        let mut bound = size_per_shard * (i + 1);
        shards.push(data[counter..bound].to_vec());
        counter += size_per_shard ;
    }

    // let mut data = vec![
    //     data[0..32].to_vec(),
    //     data[32..].to_vec(),
    //     vec![0;32],
    //     vec![0; 32],
    //     vec![0; 32],
       
    // ];
   
}
