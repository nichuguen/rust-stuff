pragma solidity ^0.4.19;

contract Truite
{
    event TruiteFired(uint32 truite);
    event Transfer(address,address,uint256);

    function Truite() public
    {
        emit TruiteFired(12);
        emit Transfer(msg.sender, msg.sender, 12);
    }


    function emitTruite () public
    {
        emit TruiteFired(42);
        emit Transfer(msg.sender, msg.sender, 42);
        emit TruiteFired(43);
        emit Transfer(msg.sender, msg.sender, 43);

    }
    
}
