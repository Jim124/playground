use ethers::types::Address;
use std::str::FromStr;

trait EthereumAddress {
    fn convert_address(&self) -> Result<Address, &'static str>;
}

impl EthereumAddress for &str {
    fn convert_address(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid ethereum address String"),
        }
    }
}

impl EthereumAddress for Address {
    fn convert_address(&self) -> Result<Address, &'static str> {
        Ok(*self)
    }
}

fn get_ethereum_date<T: EthereumAddress>(address: T) -> Address {
    let convert_address: Address = address.convert_address().unwrap();
    convert_address
}

#[cfg(test)]

mod test {
    use super::*;
    #[test]
    fn tests_poly() {
        let addr: Address =
            Address::from_str("0x45a3e755036d21bdc04ac6dae5090ded3acf3cf7079dfeccedf923f4dfb97058")
                .unwrap();

        // let new_addr: Address = get_ethereum_date(addr);
        assert_eq!(
            addr,
            Address::from_str("0x45a3e755036d21bdc04ac6dae5090ded3acf3cf7079dfeccedf923f4dfb97058")
                .unwrap()
        );
    }
}
