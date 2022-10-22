// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;
contract IPFS {
    string ipfsHash = "sos";
    
    function sendHash(string memory x) public {
        ipfsHash = x;
    }
    
    function getHash() public view returns (string memory) {
        return ipfsHash;
    }
}
