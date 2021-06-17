// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.5.16;

contract Registrar {
    bytes32 storedData;

    function set(bytes32 x) public {
        storedData = x;
    }

    function get() public view returns (bytes32) {
        return storedData;
    }
}
