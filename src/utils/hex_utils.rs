use rand::Rng;

pub fn generate_hex_address() -> String {
    // Required addresses should be 20 bytes long
    let mut rng = rand::thread_rng();
    let mut address_bytes: [u8; 20] = [0; 20];

    rng.fill(&mut address_bytes);

    // Convert the bytes to a hexadecimal string
    let hex_address = format!("0x{}", hex::encode(address_bytes));

    hex_address
}

pub fn generate_transaction_hash() -> String {
    // Required transaction hashes are 32 bytes long
    let mut rng = rand::thread_rng();
    let mut hash_bytes: [u8; 32] = [0; 32];

    rng.fill(&mut hash_bytes);

    // Convert the bytes to a hexadecimal string
    let hex_hash = format!("0x{}", hex::encode(hash_bytes));

    hex_hash
}

pub fn generate_block_hash() -> String {
    generate_transaction_hash()
}
