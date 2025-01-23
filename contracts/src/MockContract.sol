// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {CallReceiverMockTrustingForwarder, CallReceiverMock} from "@openzeppelin/contracts/mocks/CallReceiverMock.sol";

contract MockContract is CallReceiverMockTrustingForwarder {
    uint256 public storeNum = 10000;


    constructor(address _trustedForwarder) CallReceiverMockTrustingForwarder(_trustedForwarder) {}


    function update(uint256 _newStore) external {
        storeNum = _newStore;
    }

    function updateWithError(uint256 _newStore) external {
        require(false, "An error occured");
    }
}
