// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn special_s_article_fix(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_LINK_STATUS_KIND_SPECIAL_S2].contains(&status_kind) {
        if frame <= 1.0 {
            VarModule::off_flag(boma.object(), vars::common::SPECIAL_PROJECTILE_SPAWNED);
        }
    }
}

// Young Link Fire Arrow fast fall
unsafe fn fire_arrow_ff(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat2::FallJump)
                && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

// Young Link Bomb pull B-Reverse
unsafe fn bomb_b_reverse(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if frame < 5.0 {
            if stick_x * facing < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
                if frame > 1.0 && frame < 5.0 &&  !VarModule::is_flag(boma.object(), vars::common::B_REVERSED) {
                    let b_reverse = Vector3f{x: -1.0, y: 1.0, z: 1.0};
                    KineticModule::mul_speed(boma, &b_reverse, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    VarModule::on_flag(boma.object(), vars::common::B_REVERSED);
                }
            }
        }
    }
}


extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}


// Bombchu Timer Count
unsafe fn bombchu_timer(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize) {
    let gimmick_timerr = VarModule::get_int(fighter.battle_object, vars::common::GIMMICK_TIMER);
    if gimmick_timerr > 0 && gimmick_timerr < 721 {
        // Bombchu Timer Reset
        if gimmick_timerr > 719 {
            VarModule::set_int(fighter.battle_object, vars::common::GIMMICK_TIMER, 0);
            gimmick_flash(boma);
        } else {
            VarModule::set_int(fighter.battle_object, vars::common::GIMMICK_TIMER, gimmick_timerr + 1);
        }
    }
}

// Bombchu Timer Death Reset
unsafe fn bombchu_reset(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) {
        VarModule::set_int(fighter.battle_object, vars::common::GIMMICK_TIMER, 0);
    }
}

// Training Mode Bombchu Timer taunt reset
unsafe fn bombchu_training(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if is_training_mode() {
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            VarModule::set_int(fighter.battle_object, vars::common::GIMMICK_TIMER, 0);
        }
    }
}

// Lengthen sword
unsafe fn sword_length(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
	let long_sword_scale = Vector3f{x: 1.0, y: 1.1, z: 1.0};
	ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("sword"), &long_sword_scale);
}


unsafe fn holdable_dair(boma: &mut BattleObjectModuleAccessor, motion_kind: u64, frame: f32) {
    // young link dair hold
    if motion_kind == hash40("attack_air_lw")
        && frame > 20.0 && frame < 65.0 
        && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_ATTACK)
    {
        
        MotionModule::set_frame_sync_anim_cmd(boma, 68.0, true, true, false);
        
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    special_s_article_fix(fighter, boma, id, status_kind, situation_kind, frame);
    fire_arrow_ff(fighter, boma, status_kind, situation_kind, cat[1], stick_y);
    bomb_b_reverse(fighter, boma, id, status_kind, stick_x, facing, frame);
    bombchu_timer(fighter, boma, id);
    bombchu_reset(fighter, id, status_kind);
    bombchu_training(fighter, id, status_kind);
	sword_length(fighter, boma);
    holdable_dair(boma, motion_kind,frame);
}

// symbol-based call for the links' common opff
extern "Rust" {
    fn links_common(fighter: &mut smash::lua2cpp::L2CFighterCommon);
}

#[utils::macros::opff(FIGHTER_KIND_YOUNGLINK )]
pub fn younglink_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		younglink_frame(fighter);
        links_common(fighter);
    }
}

pub unsafe fn younglink_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}