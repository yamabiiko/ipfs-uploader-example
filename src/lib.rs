use std::fs::File;
use std::error::Error;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use std::time;
use web3::contract::{Contract, Options};

pub struct Cfg {
    pub file_path: String,
}

pub async fn run(cfg: Cfg) -> Result<(), Box<dyn Error>> {
    let client = IpfsClient::default();
    let file = match File::open(&cfg.file_path) {
        Ok(file_handle) => file_handle,
        Err(e) => return Err(format!("couldn't open file {}: {e}", cfg.file_path).into()),

    };
    let cid = client.add(file).await?.hash;
    store_cid(cid).await?;
    Ok(())
}


async fn store_cid(cid: String) -> web3::contract::Result<()> {
    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(transport);
    let accounts = web3.eth().accounts().await?;

    let balance = web3.eth().balance(accounts[0], None).await?;

    
    println!("Using account {:?} with balance {} Wei", accounts[0], balance);

    // Load precompiled bytecode
    let bytecode = include_str!("./contract/IPFS.bin");
    // Deploying contract
    let contract = Contract::deploy(web3.eth(), include_bytes!("./contract/IPFS.abi"))?
        .confirmations(0)
        .poll_interval(time::Duration::from_secs(10))
        .options(Options::with(|opt| opt.gas = Some(3_000_000.into())))
        .execute(bytecode, (), accounts[0])
        .await?;

    println!("Contract succesfully deployed with address {:?}", contract.address());

    //  Use contract function to store CID
    let tx = contract.call("sendHash", (cid,), accounts[0], Options::with(|opt| opt.gas = Some(3_000_000.into()))).await?;
    println!("Transaction Hash: {:?}", tx);

    std::thread::sleep(std::time::Duration::from_secs(3));

    // Output the CID
    let result = contract.query("getHash", (), None, Options::default(), None);
    let stored_cid: String = result.await?;
    println!("The CID {stored_cid} was succesfully stored in the contract");

    Ok(())
}

impl Cfg {
    pub fn build(
    mut args: impl Iterator<Item = String>,
    ) -> Result<Cfg, &'static str> {
    args.next();
    let file_path = match args.next() {
        Some(arg) => arg,
        None => return Err("usage: cargo run -- file_path"),
    };
    Ok(Cfg { file_path })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test)]
}
