use ethers::{
    core::rand,
    signers::{coins_bip39::English, MnemonicBuilder},
};
use eyre::Result;

fn gen_bip44() -> Result<()> {
    // Generate a random wallet (24 word phrase) at custom derivation path
    let mut rng = rand::thread_rng();
    // the phrase to use is in English
    let wallet = MnemonicBuilder::<English>::default()
        .word_count(24)
        // "m/44'/60'/0'/2/1" 是一个 BIP44 路径，这个路径用于派生出一个特定的加密货币钱包地址。这个路径中每个部分的含义如下：
        //  m: 表示主节点（master）
        //  44': 这个数字是 BIP44 规范的编号。BIP (Bitcoin Improvement Proposals) 是比特币的改进提案，
        //      它们用于引入新的功能或标准。带有撇号的数字（例如 44'）表示这是一个“硬派生”，这意味着无法从子
        //      节点回溯到父节点，除非你知道父节点的私钥。
        //  60': 表示代币类型。60 是以太坊（Ethereum）的代币类型编号。因此，这个路径是用来生成以太坊地址的。
        //  0': 代表账户编号，0 是默认的第一个账户。
        //  2: 这个数字表示了内部链和外部链的区别，通常的取值是 0 或 1。但在这个路径中，这个数字是 2。按照BIP44
        //      的规范，0 代表外部链（用于接收付款），1 代表内部链（用于找零）。在这个例子中，数字 2 可能表示一种
        //      自定义的用途，不过这并非 BIP44 规范中定义的标准用法。
        //  1: 这个数字是地址索引，用来从同一个父节点派生出多个子节点。增加这个索引值，可以生成新的唯一地址。
        .derivation_path("m/44'/60'/0'/2/1")?
        // Optionally add this if you want the generated mnemonic to be written
        // to a file
        // .write_to(path)
        .build_random(&mut rng)?;
        
    eprintln!("Random wallet: {wallet:?}");

    Ok(())

}

fn rec_bip44() -> Result<()> {
    let phrase = "work man father plunge mystery proud hollow address reunion sauce theory bonus";
    let index = 0u32;
    let password = "TREZOR123";

    // Access mnemonic phrase with password
    // Child key at derivation path: m/44'/60'/0'/0/{index}
    let wallet = MnemonicBuilder::<English>::default()
        .phrase(phrase)
        .index(index)?
        // Use this if your mnemonic is encrypted
        .password(password)
        .build()?;

    eprintln!("Wallet: {wallet:?}");

    Ok(())
}

fn main() -> Result<()> {
    gen_bip44()?;
    rec_bip44()?;


    Ok(())


}
