use screeps::HasPosition;
use screeps_ai::get_object_manager;
use screeps_ai::object_manager::ObjectBasicInfo;
use screeps_ai::offer_manager::{ActionType, Manager};

impl Manager {
    pub fn is_invalid_action(target_id: &str, action: &ActionType) -> bool {

        match &get_object_manager().get_game_object(target_id) {
            None => true,
            Some(target) => {
                match action {
                    ActionType::Harvest => return target.energy_empty(),
                    ActionType::Transfer(_) => return target.energy_full(),
                    ActionType::UpgradeController => return false,
                    ActionType::Renew => {}
                    ActionType::Attack => {}
                    ActionType::AttackController => {}
                    ActionType::RangeAttack => {}
                    ActionType::RangedMassAttack => {}
                    ActionType::RangeHeal => {}
                    ActionType::Build => return target.build_over(),
                    ActionType::ClaimController => {}
                    ActionType::Dismantle => {}
                    ActionType::Drop => {}
                    ActionType::GenerateSafeMode => {}
                    ActionType::Heal => {}
                    ActionType::PickUp => {}
                    ActionType::Pull => {}
                    ActionType::Repair => {}
                    ActionType::ReserveController => {}
                    ActionType::Say => {}
                    ActionType::SignController => {}
                    ActionType::Suicide => {}
                    ActionType::WithDraw => {}
                }
                false
            },
        }
    }

    pub fn creep_do_work(
        creep: &screeps::objects::Creep,
        target_info: &ObjectBasicInfo,
        action: &ActionType,
    ) {
        let obj_manager = get_object_manager();
        let target = obj_manager.get_game_object(&target_info.id).expect("creep_do_work can't find obj");

        let mut result = screeps::ReturnCode::Other;

        match action {
            ActionType::Harvest
            | ActionType::Transfer(_)
            | ActionType::Attack
            | ActionType::AttackController
            | ActionType::Renew
            | ActionType::ClaimController
            | ActionType::Dismantle
            | ActionType::GenerateSafeMode
            | ActionType::Heal
            | ActionType::PickUp
            | ActionType::Pull
            | ActionType::WithDraw => {
                if !creep.pos().is_near_to(&*target) {
                    if creep.fatigue() <= 0 {
                        creep.move_to(&*target);
                    }
                    return;
                }
            }

            ActionType::UpgradeController
            | ActionType::Build
            | ActionType::RangeAttack
            | ActionType::RangeHeal
            | ActionType::RangedMassAttack
            | ActionType::Repair => {
                if !creep.pos().in_range_to(&*target, 3) {
                    if creep.fatigue() <= 0 {
                        creep.move_to(&*target);
                    }
                    return;
                }
            }

            _ => {}
        }

        match &action {
            ActionType::Harvest => {
                result = creep._harvest(&*target);
            }
            ActionType::Transfer(v) => {
                result = creep.transfer_all(&*target, v.clone());
            }
            ActionType::UpgradeController => {
                result = creep._upgrade_controller(&*target);
            }
            ActionType::Renew => {}
            ActionType::Attack => {
                result = creep.attack(&*target);
            }
            ActionType::AttackController => {
                result = creep._attack_controller(&*target);
            }
            ActionType::RangeAttack => {
                result = creep.ranged_attack(&*target);
            }
            ActionType::RangedMassAttack => {
                result = creep.ranged_mass_attack();
            }
            ActionType::RangeHeal => {
                result = creep._ranged_heal(&*target);
            }
            ActionType::Build => {
                result = creep._build(&*target);
            }
            ActionType::ClaimController => {
                result = creep._claim_controller(&*target);
            }
            ActionType::Dismantle => {
                result = creep.dismantle(&*target);
            }
            ActionType::Drop => {}
            ActionType::GenerateSafeMode => {
                result = creep._generate_safe_mode(&*target);
            }
            ActionType::Heal => {
                result = creep._heal(&*target);
            }
            ActionType::PickUp => {
                result = creep._pickup(&*target);
            }
            ActionType::Pull => {}
            ActionType::Repair => {
                result = creep.repair(&*target);
            }
            ActionType::ReserveController => {
                result = creep._reserve_controller(&*target);
            }
            ActionType::Say => {}
            ActionType::SignController => {}
            ActionType::Suicide => {
                result = creep.suicide();
            }
            ActionType::WithDraw => {}
        }

        match result {
            screeps::ReturnCode::Ok => {}
            screeps::ReturnCode::NoBodypart => {}
            screeps::ReturnCode::NotEnough | screeps::ReturnCode::Full => {}
            _ => {
                info!("action error :{:#?}", result);
            }
        }
    }
}
