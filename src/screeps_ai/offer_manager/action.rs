mod do_work_help;
mod my_hook_impl;
mod p2p_work;

use super::Manager;

use core::borrow::BorrowMut;
use screeps::{prelude::*, ReturnCode};
use screeps_ai::object_manager::{ObjectBasicInfo, ScreepsObjectType};
use screeps_ai::offer_manager::{
    ActionType, GroupEmployInfo, PointToPointWorkInfo, WorkType, WorkerState,
};
use screeps_ai::{get_creep_manager, get_object_manager, get_offer_manager};
use std::collections::HashMap;

impl Manager {
    fn init_workers(&mut self) {
        for creep in &screeps::game::creeps::values() {
            let offer = self.find_next_offer_from_exist(creep);
            match offer {
                None => return,
                Some(v) => {
                    //Todo How to fix it perfect
                    get_offer_manager().offer_creep(&creep.name(), v);
                }
            }
        }
    }

    pub fn init_worker_action(&mut self) -> bool {
        self.init_workers();
        true
    }
    //
    //    fn do_source_action(
    //        creep: &screeps::objects::Creep,
    //        target: &ObjectBasicInfo,
    //        action: &ActionType,
    //    ) {
    //        let source: &screeps::objects::Source = &screeps::game::get_object_typed(&target.id)
    //            .expect("get_object_typed with source failed")
    //            .unwrap();
    //
    //        match action {
    //            ActionType::Harvest => {
    //                if creep.pos().is_near_to(source) {
    //                    let r = creep.harvest(source);
    //                    match r {
    //                        ReturnCode::Ok => {}
    //                        _ => {
    //                            warn!("couldn't harvest: {:?}", r);
    //                        }
    //                    }
    //                } else {
    //                    creep.move_to(source);
    //                }
    //            }
    //
    //            _ => {
    //                warn!("not support source action :{:#?}", action);
    //            }
    //        }
    //    }
    //
    //    fn do_source_source_work(creep: &screeps::objects::Creep, target_info: &ObjectBasicInfo) {
    //        let source: &screeps::objects::Source = &screeps::game::get_object_typed(&target_info.id)
    //            .expect("get_object_typed with source failed")
    //            .unwrap();
    //
    //        if creep.pos().is_near_to(source) {
    //            let r = creep.harvest(source);
    //            match r {
    //                ReturnCode::Ok => {}
    //                _ => {
    //                    warn!("couldn't harvest: {:?}", r);
    //                }
    //            }
    //        } else {
    //            creep.move_to(source);
    //        }
    //    }
    //
    //    fn do_spawn_target_work(creep: &screeps::objects::Creep, target_info: &ObjectBasicInfo) {
    //        let spawn: &screeps::objects::StructureSpawn =
    //            &screeps::game::get_object_typed(&target_info.id)
    //                .expect("get_object_typed with controller failed")
    //                .unwrap();
    //
    //        if creep.pos().is_near_to(spawn) {
    //            let r = creep.transfer_all(spawn, screeps::constants::ResourceType::Energy);
    //            match r {
    //                ReturnCode::Ok => {}
    //                _ => {
    //                    warn!("couldn't harvest: {:?}", r);
    //                }
    //            }
    //        } else {
    //            creep.move_to(spawn);
    //        }
    //    }
    //
    //    fn do_controller_target_work(creep: &screeps::objects::Creep, target_info: &ObjectBasicInfo) {
    //        let controller: &screeps::objects::StructureController =
    //            &screeps::game::get_object_typed(&target_info.id)
    //                .expect("get_object_typed with controller failed")
    //                .unwrap();
    //
    //        if creep.pos().in_range_to(controller, 3) {
    //            let r = creep.upgrade_controller(controller);
    //            if r == ReturnCode::NotInRange {
    //                creep.move_to(controller);
    //            } else if r != ReturnCode::Ok {
    //                warn!("couldn't upgrade: {:?}", r);
    //            }
    //        } else {
    //            creep.move_to(controller);
    //        }
    //    }

    pub fn creeps_do_work(&mut self) {
        let creeps: HashMap<String, screeps::objects::Creep> = screeps::game::creeps::keys()
            .into_iter()
            .zip(screeps::game::creeps::values().into_iter())
            .collect();

        let mut lose_creeps = Vec::new();

        for (_, offers) in self.offer_list.borrow_mut() {
            for offer_info in offers {
                if offer_info.pausing {
                    //                    info!("pausing!");
                    continue;
                } else {
                    for (name, state) in &mut offer_info.workers {
                        info!("!current running creep: {}", name);

                        match &offer_info.offer_type {
                            WorkType::PointToPoint(v) => {
                                if let Some(creep) = creeps.get(&name.clone()) {
                                    if creep.spawning() {
                                        continue;
                                    }
                                    if !creep.saying() {
                                        creep.say("do work", false);
                                    }
                                    Manager::creep_do_p2p_work(creep, state, v);
                                } else {
                                    warn!("not find creep name :{}, workers: ", name);
                                    lose_creeps.push(name.clone());
                                }
                            }
                            WorkType::CleanRoom => info!("offer CleanRoom not implement"),
                            WorkType::UnKnown => info!("offer Unknown Type"),
                        }
                    }
                }
            }
        }

        if !lose_creeps.is_empty() {
            get_creep_manager().cleanup_memory();
            for lose_creep in lose_creeps {
                self.fire_creep(&lose_creep);
            }
        }
    }
}
