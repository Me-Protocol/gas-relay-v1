// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {CallReceiverMockTrustingForwarder, CallReceiverMock} from "@openzeppelin/contracts/mocks/CallReceiverMock.sol";

contract MockContract is CallReceiverMockTrustingForwarder {
    constructor(address _trustedForwarder) CallReceiverMockTrustingForwarder(_trustedForwarder) {}
}
