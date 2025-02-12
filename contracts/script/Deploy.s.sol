// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {TrustedForwarder} from "../src/TrustedForwarder.sol";

contract DeployScript is Script {
    uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
    address owner = vm.addr(deployerPrivateKey);


    TrustedForwarder public trustedForwarder;

    function setUp() public {}

    function run() public {
        vm.startBroadcast(deployerPrivateKey);
        
        bytes32 salt = bytes32(0xdeadbeefbaddc0de1234babe8765feedfacec0ffee1337c0ffeecafeeaffeedc);
        trustedForwarder = new TrustedForwarder{salt: salt}("ERC2771Forwarder");

        vm.stopBroadcast();
    }
}
