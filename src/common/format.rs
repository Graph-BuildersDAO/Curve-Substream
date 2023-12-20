use substreams::Hex;

pub fn format_address_vec(address: &Vec<u8>) -> String {
    format!("0x{}", Hex::encode(address))
}

pub fn format_address_string(address: &str) -> String {
    format!("0x{}", address)
}