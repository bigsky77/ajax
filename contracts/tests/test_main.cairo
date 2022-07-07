%lang starknet

from src.main import array_sum

@external 
func test_sum() -> (sum: felt):
    let (r) = array_sum(2, 3)
    return (r)
    end