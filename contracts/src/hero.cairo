%lang starknet 

from starkware.cairo.common.cairo_builtins import HashBuiltin

@storage_var
func hero_damage() -> (res : felt):
end

@storage_var 
func hero_health() -> (res : felt):
end

v@external
func set_damage{
        syscall_ptr : felt*,
        pedersen_ptr : HashBuiltin*,
        range_check_ptr, 
}(name : felt):
    hero_damage.write(damage)  
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
func view_damage{
        syscall_ptr : felt*,
        pedersen_ptr : HashBuiltin*,
        range_check_ptr,
}() -> (res : felt):
    let (res) = hero_damage.read()
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




