// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {TrustedForwarder} from "../src/TrustedForwarder.sol";
import {MockContract} from "../src/MockContract.sol";

contract DeployScript is Script {
    uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
    address owner = vm.addr(deployerPrivateKey);


    TrustedForwarder public trustedForwarder;
    MockContract public mockContract;

    function setUp() public {}

    function run() public {
        vm.startBroadcast(deployerPrivateKey);

        trustedForwarder = new TrustedForwarder("ERC2771Forwarder");
        mockContract = new MockContract(address(trustedForwarder));

        vm.stopBroadcast();
    }
}
