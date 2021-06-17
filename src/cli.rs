use hex::FromHexError;
use std::path::PathBuf;
use structopt::StructOpt;
use web3::types::Address;

pub fn parse_address(address: &str) -> Result<Address, FromHexError> {
    hex::decode(address).map(|a| Address::from_slice(&a))
}

#[derive(Debug, StructOpt)]
pub struct Cli {
    /// Specify Filename to upload to IPFS.
    #[structopt()]
    pub filename: PathBuf,
    #[allow(missing_docs)]
    #[structopt(flatten)]
    pub run: RunCmd,
}

#[derive(Debug, StructOpt)]
pub struct RunCmd {
    /// Specify IPFS HTTP RPC server TCP port.
    #[structopt(long = "ipfs-port", value_name = "PORT")]
    pub ipfs_port: Option<u16>,

    /// Specify IPFS HTTP RPC server TCP Host IP.
    #[structopt(flatten)]
    pub ipfs_host_ip: structopt_flags::HostOpt,

    /// Specify ETHEREUM HTTP RPC server TCP port.
    #[structopt(long = "eth-port", value_name = "PORT")]
    pub eth_port: Option<u16>,

    /// Specify ETHEREUM HTTP RPC server TCP Host name.
    #[structopt(name = "eth-host", long = "ethhost", short = "-E", global = true)]
    pub eth_host_name: Option<String>,

    /// Specify ETHEREUM contract address to register CID.
    #[structopt(name = "contract", long = "contract", short = "-C", global = true, parse(try_from_str = parse_address))]
    pub contract: Option<Address>,
}
