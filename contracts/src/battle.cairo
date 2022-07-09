%lang starknet 

from starkware.cairo.common.cairo_builtins import HashBuiltin

@contract_interface
namespace IHeroContract: 
    func set_health(hero_health : felt):
    end 

    func set_damage(hero_damage : felt):
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






