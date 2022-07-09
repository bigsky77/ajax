%lang starknet 

from starkware.cairo.common.cairo_builtins import HashBuiltin

@storage_var
func damage() -> (hero_damage : felt):
end

@storage_var 
func health() -> (hero_health : felt):
end

@constructor 
func constructor{
        syscall_ptr : felt*,
        pedersen_ptr : HashBuiltin*,
        range_check_ptr,
    }(hero_health : felt):
        health.write(value=hero_health)
        return()
end

@external
func set_damage{
        syscall_ptr : felt*,
        pedersen_ptr : HashBuiltin*,
        range_check_ptr, 
}(hero_damage : felt):
    damage.write(hero_damage)  
    return()
end

@external
func set_health{
        syscall_ptr : felt*,
        pedersen_ptr : HashBuiltin*,
        range_check_ptr,        
}(hero_health : felt):
    health.write(hero_health)
    return()
end

@external
func take_damage{
        syscall_ptr : felt*,
        pedersen_ptr : HashBuiltin*, 
        range_check_ptr,
}(damage : felt):
    let (res) = health.read()
    health.write(res - damage)
    return()
end

@view 
func view_damage{
        syscall_ptr : felt*,
        pedersen_ptr : HashBuiltin*,
        range_check_ptr,
}() -> (res : felt):
    let (res) = damage.read()
    return (res=res)
end

@view 
func view_health{
        syscall_ptr : felt*,
        pedersen_ptr : HashBuiltin*,
        range_check_ptr,
}() -> (res : felt):
    let (res) = health.read()
    return (res=res)
end




