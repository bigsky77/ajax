%lang starknet 

from starkware.cairo.common.cairo_builtins import HashBuiltin

const XOROSHIRO_ADDR = 0x06c4cab9afab0ce564c45e85fe9a7aa7e655a7e0fd53b7aea732814f3a64fbee

@contract_interface
namespace IXoroshiro:
    func next() -> (rnd : felt):
    end
end

@external
func get_next_rnd{syscall_ptr : felt*, range_check_ptr}() -> (rnd : felt):
    let (rnd) = IXoroshiro.next(contract_address=XOROSHIRO_ADDR)
    return (rnd)
end

@contract_interface
namespace IHeroContract: 
    func set_health(hero_health : felt):
    end 

    func set_damage(hero_damage : felt):
    end

    func take_damage(damage : felt):
    end

    func view_health():
    end
end

@contract_interface
namespace IEnemyContract:
    func set_health(enemy_health : felt):
    end

    func set_damage(enemy_damage : felt):
    end

    func take_damage(damage : felt):
    end

    func view_health():
    end
end

@external
func call_set_hero_health{syscall_ptr : felt*, range_check_ptr}(
    contract_address : felt, hero_health : felt
):
    IHeroContract.set_health(
        contract_address=contract_address, hero_health=hero_health
    )
    return ()
end

@external
func call_set_enemy_health{syscall_ptr : felt*, range_check_ptr}(
    contract_address : felt, enemy_health : felt
):
    IEnemyContract.set_health(
        contract_address=contract_address, enemy_health=enemy_health
    )
    return ()
end

@external
func view_hero_health{syscall_ptr : felt*, range_check_ptr}(
    contract_address : felt, amount : felt
):
    IHeroContract.view_health(
        contract_address=contract_address
    )  
    return ()
end

@external
func view_enemy_health{syscall_ptr : felt*, range_check_ptr}(
    contract_address : felt, amount : felt
):
    IEnemyContract.view_health(
        contract_address=contract_address
    )  
    return ()
end

@external
func hero_attack{syscall_ptr : felt*, range_check_ptr}(
    contract_address : felt
):
    let (damage) = get_next_rnd()

    IEnemyContract.take_damage(
        contract_address=contract_address, damage=damage
    )
    return ()
end
    
@external
func enemy_attack{syscall_ptr : felt*, range_check_ptr}(
    contract_address : felt
):
    let (damage) = get_next_rnd()

    IHeroContract.take_damage(
        contract_address=contract_address, damage=damage
    )
    return ()
end


