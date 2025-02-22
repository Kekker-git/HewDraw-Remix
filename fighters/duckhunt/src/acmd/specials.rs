
use super::*;

#[acmd_script( agent = "duckhunt", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_REQUEST_SPECIAL_HI_CANCEL);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        duckhunt_special_hi_game,
    );
}

