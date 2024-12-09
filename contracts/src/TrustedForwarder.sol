// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ERC2771Forwarder} from "@openzeppelin/contracts/metatx/ERC2771Forwarder.sol";

contract TrustedForwarder is ERC2771Forwarder {
    constructor(string memory name) ERC2771Forwarder(name) {}
}


// 0xc845a056 ERC2771ForwarderInvalidSigner(address,address)
// 0x70647f79 ERC2771ForwarderMismatchedValue(uint256,uint256)
// 0x94eef58a ERC2771ForwarderExpiredRequest(uint48)
// 0x244aa570 ERC2771UntrustfulTarget(address,address);