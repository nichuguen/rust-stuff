pragma solidity ^0.4.19;

contract Truite
{
    event TruiteFired(uint32 truite);

    function Truite() public
    {
        emit TruiteFired(12);
    }


    function emitTruite () public
    {
        emit TruiteFired(42);
    }
    
}
