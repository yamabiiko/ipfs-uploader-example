# ipfs-uploader-example
A simple proof of concept CLI tool that:
1. Takes a local file as an argument
2. Uploads the file to the IPFS network
3. Deploys a Solidity smart contract and stores CID of the file  in it
## Prerequisities
- Ipfs daemon running at the default address `localhost:8080`
- Ganache or some other Ethereum test enviroment running with the default address `127.0.0.1:8545`
## Usage
Example adding this README:
`cargo run -- README.md`
