
mod wallet;

use ethers::core::rand;
use wallet::Wallet;
use eyre::Result;


fn main() -> Result<()> {
    let phrase = "work man father plunge mystery proud hollow address reunion sauce theory bonus";
    let index = 0u32;
    let password = "TREZOR123";

    let wallet = Wallet::new(phrase, index, password);

    match wallet.create_wallet() {
        Ok(wallet_info) => eprintln!("{}", wallet_info),
        Err(err) => eprintln!("Failed to create wallet: {}", err),
    }

    let mut rng = rand::thread_rng();
    match wallet.create_random_wallet(&mut rng, "m/44'/60'/0'/2/1") {
        Ok(wallet_info) => eprintln!("{}", wallet_info),
        Err(err) => eprintln!("Failed to create random wallet: {}", err),
    }

    Ok(())
}