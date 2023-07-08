use bip44;


#[test]
fn new_wallet() {
    let phrase = "work man father plunge mystery proud hollow address reunion sauce theory bonus";
    let index = 0u32;
    let password = "TREZOR123";

    let wallet = bip44::wallet::Wallet::new(phrase, index, password);
}