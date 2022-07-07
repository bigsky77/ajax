<<<<<<< HEAD
%builtins output  

from starkware.cairo.common.serialize import serialize_word 

func main{output_ptr : felt*}():
    serialize_word(1234)
    serialize_word(4321)
    return ()
end


=======
%lang starknet 

func array_sum(arr: felt*, size) -> (sum: felt):
    if size == 0:
        return (sum=0)
    end 

    let (sum_of_rest) = array_sum(arr=arr + 1, size=size -1)
    return (sum=[arr] + sum_of_rest)
end



>>>>>>> f64c185678d0bf4f5b5e4994698ec477857c78d3
