use screeps_ai::offer_manager::{Manager, ActionType, GroupEmployInfo, PointToPointWorkInfo, WorkType};
use screeps_ai::{get_object_manager, get_offer_manager};
use screeps_ai::object_manager::ScreepsObjectType;
use std::rc::Rc;

const BASIC_SPAWN_OFFER_LEVEL: i32 = -100;
const BASIC_EXTENSION_OFFER_LEVEL:i32 = -99;
const BASIC_UPGRADE_OFFER_LEVEL:i32 = 1000;
const BASIC_BUILD_OFFER_LEVEL:i32 = -50;
const BASIC_NORMAL_TRANSFER_OFFER_LEVEL:i32 = -49;
const BASIC_REPAIR_OFFER_LEVEL:i32 = -48;

impl Manager {

    fn basic_spawn_offer(&mut self){
        for spawn in get_object_manager().get_structures(ScreepsObjectType::Spawn) {
            self.add_target_offer_use_source(
                BASIC_SPAWN_OFFER_LEVEL,
                spawn.id.clone(),
                ActionType::Transfer(screeps::constants::ResourceType::Energy),
                1,
            );
        }
    }

    fn basic_upgrade_offer(&mut self){
        for controller in get_object_manager().get_structures(ScreepsObjectType::Controller) {
            self.add_target_offer_use_source(
                BASIC_UPGRADE_OFFER_LEVEL,
                controller.id.clone(),
                ActionType::UpgradeController,
                0,
            );
        }
    }

    fn basic_extension_offer(&mut self){
        let temp_target = get_object_manager().get_first_spawn();
        let mut employ_info = GroupEmployInfo::new();
        let p2p = PointToPointWorkInfo {
            source: temp_target.clone(),
            source_action: ActionType::Harvest,
            target: temp_target,
            target_action: ActionType::Transfer(screeps::constants::ResourceType::Energy),
        };

        employ_info.offer_type = WorkType::ExtensionTransfer(p2p);
        employ_info.max_number = 1;

        let offers = self.offer_list.entry(BASIC_EXTENSION_OFFER_LEVEL).or_default();
        offers.push(Rc::new(employ_info));
    }

    fn basic_build_offer(&mut self){
        let temp_target = get_object_manager().get_first_spawn();
        let mut employ_info = GroupEmployInfo::new();
        let p2p = PointToPointWorkInfo {
            source: temp_target.clone(),
            source_action: ActionType::Harvest,
            target: temp_target,
            target_action: ActionType::Build,
        };

        employ_info.offer_type = WorkType::BuildAll(p2p);
        employ_info.max_number = 0;

        let offers = self.offer_list.entry(BASIC_BUILD_OFFER_LEVEL).or_default();
        offers.push(Rc::new(employ_info));
    }

    fn basic_normal_transfer(&mut self){
        let temp_target = get_object_manager().get_first_spawn();
        let mut employ_info = GroupEmployInfo::new();
        let p2p = PointToPointWorkInfo {
            source: temp_target.clone(),
            source_action: ActionType::Harvest,
            target: temp_target,
            target_action: ActionType::Transfer(screeps::constants::ResourceType::Energy),
        };

        employ_info.offer_type = WorkType::NormalTransfer(p2p);
        employ_info.max_number = 1;

        let offers = self.offer_list.entry(BASIC_NORMAL_TRANSFER_OFFER_LEVEL).or_default();
        offers.push(Rc::new(employ_info));
    }

    fn basic_repair_offer(&mut self){
        let temp_target = get_object_manager().get_first_spawn();
        let mut employ_info = GroupEmployInfo::new();
        let p2p = PointToPointWorkInfo {
            source: temp_target.clone(),
            source_action: ActionType::Harvest,
            target: temp_target,
            target_action: ActionType::Repair,
        };

        employ_info.offer_type = WorkType::BuildAll(p2p);
        employ_info.max_number = 1;

        let offers = self.offer_list.entry(BASIC_REPAIR_OFFER_LEVEL).or_default();
        offers.push(Rc::new(employ_info));
    }

    fn basic_on_pausing(&mut self){
//        let upgrade_offer = &get_offer_manager().offer_list.get(&BASIC_UPGRADE_OFFER_LEVEL).expect("impossible 1")[0];

    }

    pub fn init_basic_offer(&mut self){

        //basic
        self.basic_spawn_offer();
        self.basic_extension_offer();

        //pausing do
        self.basic_repair_offer();
        self.basic_normal_transfer();

        self.basic_build_offer();

        self.basic_upgrade_offer();


        self.basic_on_pausing();
    }
}