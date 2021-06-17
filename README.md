# IPFS Smart contract integration

Showcase for adding a file to localhost 5001 IPFS node and push it's CID to a smart contract on localhost ganache ethereum

## Build and deploy contract

```
npm install -g truffle
truffle compile
truffle migrate
```

## Usage

```
ipfs-registrar 0.1.0

USAGE:
    ipfs-registrar [OPTIONS] <filename>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -C, --contract <contract>    Specify ETHEREUM contract address to register CID
    -E, --ethhost <eth-host>     Specify ETHEREUM HTTP RPC server TCP Host name
        --eth-port <PORT>        Specify ETHEREUM HTTP RPC server TCP port
    -H, --host <host>            Set the host IP (both IpV4 and IpV6 are supported)
        --ipfs-port <PORT>       Specify IPFS HTTP RPC server TCP port

ARGS:
    <filename>    Specify Filename to upload to IPFS
```