// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

contract EthereumLightClient {
    bytes32 public currentBlockHeader;

    function updateBlockHeader(bytes32 newBlockHeader) public {
        currentBlockHeader = newBlockHeader;
    }
}
