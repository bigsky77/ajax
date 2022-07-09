%lang starknet 

from starkware.cairo.common.cairo_builtins import HashBuiltin

@storage_var
func damage() -> (enemy_damage : felt):
end

@storage_var 
func health() -> (enemy_health : felt):
end
  
@constructor 
func constructor{
        syscall_ptr : felt*,
        pedersen_ptr : HashBuiltin*,
        range_check_ptr,
    }(enemy_health : felt):
        health.write(value=enemy_health)
        return()
end

@external
func set_damage{
        syscall_ptr : felt*,
        pedersen_ptr : HashBuiltin*,
        range_check_ptr, 
}(enemy_damage : felt):
    damage.write(enemy_damage)  
    return()
end

@external
func set_health{
        syscall_ptr : felt*,
        pedersen_ptr : HashBuiltin*,
        range_check_ptr,        
}(enemy_health : felt):
    health.write(enemy_health)
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




