
use ethers:: {
    core::rand,
    signers::{coins_bip39::English, MnemonicBuilder},
};
use eyre::{eyre, Result};

pub struct Wallet {
    phrase: String,
    index: u32,
    password: String,
}

impl Wallet {

    pub fn new(phrase: &str, index: u32, password: &str) -> Self {
        Self {
            phrase: phrase.into(),
            index,
            password: password.into(),
        }
    }

    pub fn create_wallet(&self) -> Result<String> {
        let wallet = MnemonicBuilder::<English>::default()
        .phrase(self.phrase.as_str())
        .index(self.index)?
        .password(self.password.as_str())
        .build()
        .map_err(|err| eyre!("Failed to create wallet: {}", err))?;

        Ok(format!("Wallet: {:?}", wallet))
    }

    pub fn create_random_wallet<R: rand::Rng>(&self, rng: &mut R, derivation_path: &str) -> Result<String> {
        let wallet = MnemonicBuilder::<English>::default()
        .word_count(24)
        .derivation_path(derivation_path)?
        .build_random(rng)
        .map_err(|err| eyre!("Failed to create random waller: {}", err))?;

        Ok(format!("Random wallet: {:?}", wallet))
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_wallet() {
        let phrase = "work man father plunge mystery proud hollow address reunion sauce theory bonus";
        let index = 0u32;
        let password = "TREZOR123";

        let wallet = Wallet::new(phrase, index, password);
    }
}

