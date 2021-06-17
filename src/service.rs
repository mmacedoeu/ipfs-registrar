use anyhow::{Error, Result};
use cid::Cid;
use hex_literal::hex;
use ipfs_api::TryFromUri;
use ipfs_api::{response::AddResponse, IpfsClient};
use std::fs::File;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use structopt_flags::GetWithDefault;
use tokio::runtime::Runtime;
use web3::ethabi::Token;
use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::Address,
    Web3,
};

use crate::cli::Cli;

const CONTRACT_DEFAULT: [u8; 20] = hex!("FC9E6Fd004140f737FD422E384d203e634B7FeE7");

pub async fn upload_ipfs(cli: &IpfsClient, file: File) -> Result<AddResponse> {
    cli.add(file).await.map_err(Error::msg)
}

pub async fn set_contract(web3: &Web3<Http>, contract: &Contract<Http>, cid: Token) -> Result<()> {
    println!("cid: {}", cid);

    let accounts = web3.eth().accounts().await?;

    // Get current balance
    let balance = web3.eth().balance(accounts[0], None).await?;

    println!("Balance: {}", balance);

    // Change state of the contract
    let tx = contract
        .call("set", (cid,), accounts[0], Options::default())
        .await?;
    println!("TxHash: {}", tx);

    let _a = tokio::time::sleep(std::time::Duration::from_secs(5));

    // View changes made
    let result = contract.query("get", (), None, Options::default(), None);
    let storage: Token = result.await?;
    println!("Get: {}", storage);
    Ok(())
}

pub fn upload(cli: &Cli) -> Result<()> {
    let ipfs_port = cli.run.ipfs_port.unwrap_or(5001);
    let ipfs_host = cli
        .run
        .ipfs_host_ip
        .get_with_default(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let file = File::open(&cli.filename)?;

    // let ipfs_client = IpfsClient::default();
    let ip_addr = match ipfs_host {
        IpAddr::V4(i) => format!("/ip4/{}", i),
        IpAddr::V6(i) => format!("/ip6/{}", i),
    };
    let multiaddr_str = format!("{}/tcp/{}", ip_addr, ipfs_port);
    let ipfs_client = IpfsClient::from_multiaddr_str(&multiaddr_str)?;

    let eth_port = cli.run.eth_port.unwrap_or(7545);
    let eth_host = cli
        .run
        .eth_host_name
        .clone()
        .unwrap_or("localhost".to_owned());

    let contract_address = cli.run.contract.unwrap_or(Address::from(CONTRACT_DEFAULT));
    let eth_url = format!("http://{}:{}", eth_host, eth_port);
    println!("ETH url \t {}", eth_url);
    let http = web3::transports::Http::new(&eth_url)?;
    let web3 = web3::Web3::new(http);
    let contract = Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("../res/Registrar_abi.json"),
    )?;

    let rt = Runtime::new()?;
    rt.block_on(async move {
        let r = upload_ipfs(&ipfs_client, file).await;
        if let Ok(res) = r {
            let hash = Cid::from_str(&res.hash);
            if let Ok(h) = hash {
                let b = h.to_bytes();
                let h = Token::FixedBytes(b[2..].into());
                // Ok(())
                set_contract(&web3, &contract, h).await
            } else {
                hash.map(|_| ()).map_err(Error::msg)
            }
        } else {
            r.map(|_| ())
        }
    })
    .map_err(Error::msg)
}
