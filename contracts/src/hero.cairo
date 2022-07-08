%lang starknet 

from starkware.cairo.common.cairo_builtins import HashBuiltin

@storage_var
func hero_name() -> (res : felt):
end

@storage_var 
func hero_health() -> (res : felt):
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

@external
func set_health{
        syscall_ptr : felt*,
        pedersen_ptr : HashBuiltin*,
        range_check_ptr,        
}(health : felt):
    hero_health.write(health)
    return()
end

@view 
func view_name{
        syscall_ptr : felt*,
        pedersen_ptr : HashBuiltin*,
        range_check_ptr,
}() -> (res : felt):
    let (res) = hero_name.read()
    return (res=res)
end

@view 
func view_health{
        syscall_ptr : felt*,
        pedersen_ptr : HashBuiltin*,
        range_check_ptr,
}() -> (res : felt):
    let (res) = hero_health.read()
    return (res=res)
end




