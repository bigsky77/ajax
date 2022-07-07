%lang starknet 

from starkware.cairo.common.cairo_builtins import HashBuiltin

@storage_var
func hero_name() -> (re : felt):
end

@external
func set_name{
       syscall_ptr : felt*,
       pedersen_ptr : HashBuiltin*,
       range_check_ptr, 
}(name : felt):
    hero_name.write(name)  
    return()
end
