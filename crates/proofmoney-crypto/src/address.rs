use anyhow::{anyhow, Result};
use bech32::{self, Bech32, Hrp};

pub fn public_key_to_address(public_key: &[u8], prefix: &str) -> Result<String> {
    let hrp = Hrp::parse(prefix)?;
    let address = bech32::encode::<Bech32>(hrp, public_key)?;
    Ok(address)
}

pub fn is_valid_address(address: &str, expected_prefix: &str) -> bool {
    match bech32::decode(address) {
        Ok((hrp, _data)) => hrp.as_str() == expected_prefix,
        Err(_) => false,
    }
}

pub fn address_payload(address: &str) -> Result<Vec<u8>> {
    let (_hrp, data) = bech32::decode(address).map_err(|e| anyhow!("invalid bech32 address: {e}"))?;
    Ok(data)
}
