//@compile-flags: --severity gas
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

contract PostIncrementDecrement {
    // should trigger - in for loop
    function bad(uint256[] memory arr) public pure {
        for (uint256 i = 0; i < arr.length; i++) { //~NOTE: use pre-increment/decrement instead of post-increment/decrement
        }
    }

    // should trigger - decrement in for loop
    function badDec(uint256[] memory arr) public pure {
        for (uint256 i = arr.length; i > 0; i--) { //~NOTE: use pre-increment/decrement instead of post-increment/decrement
        }
    }

    // should trigger - standalone post-increment
    function badStandalone() public pure returns (uint256 i) {
        i++; //~NOTE: use pre-increment/decrement instead of post-increment/decrement
    }

    // should not trigger - pre-increment
    function good(uint256[] memory arr) public pure {
        for (uint256 i = 0; i < arr.length; ++i) {
        }
    }

    // should not trigger - pre-decrement
    function alsoGood(uint256[] memory arr) public pure {
        for (uint256 i = arr.length; i > 0; --i) {
        }
    }
}