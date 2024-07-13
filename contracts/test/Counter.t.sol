// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {EthereumLightClient} from "../src/EthereumLightClient.sol";

contract EthereumLightClientTest is Test {
    EthereumLightClient public client;

    function setUp() public {
        client = new EthereumLightClient();
    }

    function test_updateBlockHeader() public {
    
    }
}
