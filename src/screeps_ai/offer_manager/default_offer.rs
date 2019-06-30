use core::borrow::BorrowMut;
use screeps::{CanStoreEnergy, HasId, OwnedStructureProperties};
use screeps_ai::offer_manager::{
    ActionType, GroupEmployInfo, Manager, PointToPointWorkInfo, WorkType,
};
use screeps_ai::{get_object_manager, get_offer_manager, object_manager};
use std::collections::HashMap;

impl Manager {
    fn add_offer_info(
        &mut self,
        level: i32,
        target_id: String,
        target_action: ActionType,
        number1: usize,
        number2: usize,
        add_to: i32,
    ) {
        let offers = self.offer_list.entry(level).or_default();
        let obj_manager = get_object_manager();
        let mut first = true;

        for source in obj_manager.get_object_to_source(&target_id) {
            let mut employ_info = GroupEmployInfo::new();
            let p2p = PointToPointWorkInfo {
                source,
                source_action: ActionType::Harvest,
                target: obj_manager.get_object(&target_id),
                target_action: target_action.clone(),
            };

            employ_info.offer_type = WorkType::PointToPoint(p2p);

            employ_info.max_number = if first {
                first = false;
                number1
            } else {
                number2
            };
            offers.push(employ_info);

            if add_to == 1 {
                let temp_list = self.spawn_offers.entry(target_id.clone()).or_default();
                temp_list.push(
                    &mut get_offer_manager()
                        .offer_list
                        .get_mut(&level)
                        .expect("add_offer_info")[offers.len() - 1],
                );
            }
        }
    }

    fn init_spawn_offer(&mut self) {
        for spawn in &object_manager::Manager::get_my_spawns() {
            self.add_offer_info(
                1,
                spawn.id(),
                ActionType::Transfer(screeps::constants::ResourceType::Energy),
                1,
                2,
                1,
            );
            self.add_offer_info(
                10,
                spawn.id(),
                ActionType::Transfer(screeps::constants::ResourceType::Energy),
                0,
                5,
                1,
            );
            self.max_number += 8;
        }
    }

    fn init_controller_offer(&mut self) {
        for room in &screeps::game::rooms::values() {
            let controller: &screeps::objects::StructureController = &room.controller().unwrap();
            if !controller.my() {
                continue;
            }
            self.add_offer_info(2, controller.id(), ActionType::UpgradeController, 8, 17, 0);
            self.max_number += 25;
        }
    }

    pub fn init_default_offers(&mut self) {
        self.init_spawn_offer();

        self.init_controller_offer();
    }

    pub fn set_offer_state(&mut self) {
        for spawn in &object_manager::Manager::get_my_spawns() {
            let offers = self
                .spawn_offers
                .get_mut(&spawn.id())
                .expect("not found spawn in spawn offers");

            let need_pausing = spawn.energy() >= spawn.energy_capacity();
            for offer in offers {
                if need_pausing == offer.pausing {
                    break;
                } else {
                    offer.pausing = need_pausing;
                }
            }
        }
    }
}
