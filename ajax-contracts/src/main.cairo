%lang starknet 

func array_sum(arr: felt*, size) -> (sum: felt):
    if size == 0:
        return (sum=0)
    end 

    let (sum_of_rest) = array_sum(arr=arr + 1, size=size -1)
    return (sum=[arr] + sum_of_rest)
end



