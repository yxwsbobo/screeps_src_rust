use screeps_ai::object_manager::ScreepsObjectType;
use screeps_ai::offer_manager::offer_interface::get_offer_mut;
use screeps_ai::offer_manager::{
    ActionType, GroupEmployInfo, Manager, PointToPointWorkInfo, WorkType,
};
use screeps_ai::{get_object_manager, get_offer_manager};
use std::rc::Rc;

const BASIC_SPAWN_OFFER_LEVEL: i32 = -1000;
const BASIC_EXTENSION_OFFER_LEVEL: i32 = -999;
const BASIC_UPGRADE_OFFER_LEVEL: i32 = 1000;
const BASIC_REPAIR_OFFER_LEVEL: i32 = -500;
const BASIC_NORMAL_TRANSFER_OFFER_LEVEL: i32 = -499;
const BASIC_BUILD_OFFER_LEVEL: i32 = -498;

impl Manager {
    fn basic_spawn_offer(&mut self) {
        for spawn in get_object_manager().get_structures(ScreepsObjectType::Spawn) {
            self.add_target_offer_use_source(
                BASIC_SPAWN_OFFER_LEVEL,
                spawn.id.clone(),
                ActionType::Transfer(screeps::constants::ResourceType::Energy),
                1,
            );
        }
    }

    fn basic_upgrade_offer(&mut self) {
        for controller in get_object_manager().get_structures(ScreepsObjectType::Controller) {
            self.add_target_offer_use_source(
                BASIC_UPGRADE_OFFER_LEVEL,
                controller.id.clone(),
                ActionType::UpgradeController,
                8,
            );
        }
    }

    fn basic_extension_offer(&mut self) {
        let temp_target = get_object_manager().get_empty_basic_info();
        let mut employ_info = GroupEmployInfo::new();
        let p2p = PointToPointWorkInfo {
            source: temp_target.clone(),
            source_action: ActionType::Harvest,
            target: temp_target,
            target_action: ActionType::Transfer(screeps::constants::ResourceType::Energy),
        };

        employ_info.offer_type = WorkType::ExtensionTransfer(p2p);
        employ_info.max_number = 5;

        let offers = self
            .offer_list
            .entry(BASIC_EXTENSION_OFFER_LEVEL)
            .or_default();
        offers.push(Rc::new(employ_info));
    }

    fn basic_build_offer(&mut self) {
        let temp_target = get_object_manager().get_empty_basic_info();
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

    fn basic_normal_transfer(&mut self) {
        let temp_target = get_object_manager().get_empty_basic_info();
        let mut employ_info = GroupEmployInfo::new();
        let p2p = PointToPointWorkInfo {
            source: temp_target.clone(),
            source_action: ActionType::Harvest,
            target: temp_target,
            target_action: ActionType::Transfer(screeps::constants::ResourceType::Energy),
        };

        employ_info.offer_type = WorkType::NormalTransfer(p2p);
        employ_info.max_number = 1;

        let offers = self
            .offer_list
            .entry(BASIC_NORMAL_TRANSFER_OFFER_LEVEL)
            .or_default();
        offers.push(Rc::new(employ_info));
    }

    fn basic_repair_offer(&mut self) {
        let temp_target = get_object_manager().get_empty_basic_info();
        let mut employ_info = GroupEmployInfo::new();
        let p2p = PointToPointWorkInfo {
            source: temp_target.clone(),
            source_action: ActionType::Harvest,
            target: temp_target,
            target_action: ActionType::Repair,
        };

        employ_info.offer_type = WorkType::RepairAll(p2p);
        employ_info.max_number = 1;

        let offers = self.offer_list.entry(BASIC_REPAIR_OFFER_LEVEL).or_default();
        offers.push(Rc::new(employ_info));
    }

    fn basic_on_pausing(&mut self) {
        let upgrade_offer = Rc::downgrade(
            &self
                .offer_list
                .get(&BASIC_UPGRADE_OFFER_LEVEL)
                .expect("impossible 1")[0],
        );
        let build_offer = &self
            .offer_list
            .get(&BASIC_BUILD_OFFER_LEVEL)
            .expect("impossible 4")[0];
        get_offer_mut(&build_offer).next_offer = upgrade_offer.clone();
        let build_offer = Rc::downgrade(build_offer);

        let spawn_offer = &self
            .offer_list
            .get(&BASIC_SPAWN_OFFER_LEVEL)
            .expect("impossible 2")[0];

        get_offer_mut(&spawn_offer).next_offer = upgrade_offer;

        let extension_offer = &self
            .offer_list
            .get(&BASIC_EXTENSION_OFFER_LEVEL)
            .expect("impossible 6")[0];

        get_offer_mut(&extension_offer).next_offer = build_offer.clone();

        let repair_offer = &self
            .offer_list
            .get(&BASIC_REPAIR_OFFER_LEVEL)
            .expect("impossible 5")[0];

        get_offer_mut(&repair_offer).next_offer = build_offer;
        let repair_offer = Rc::downgrade(&repair_offer);

        let normal_transfer_offer = &self
            .offer_list
            .get(&BASIC_NORMAL_TRANSFER_OFFER_LEVEL)
            .expect("impossible transfer 6")[0];
        get_offer_mut(&normal_transfer_offer).next_offer = repair_offer;
    }

    pub fn init_basic_offer(&mut self) {
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
