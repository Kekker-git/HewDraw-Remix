// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn special_cancels(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {
    if status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_HI1_2 {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            // Check for shield inputs during Soaring Axe Kick
            if frame > 19.0 {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                    ControlModule::clear_command(boma, true);
                    VarModule::on_flag(boma.object(), vars::common::UP_SPECIAL_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                }
            }
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    // Magic Series
    special_cancels(boma, id, status_kind, frame);
}

#[utils::macros::opff(FIGHTER_KIND_MIIFIGHTER )]
pub fn miifighter_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		miifighter_frame(fighter)
    }
}

pub unsafe fn miifighter_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}