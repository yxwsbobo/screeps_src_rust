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

    fn check_creep_contains(&self) {
        let mut flag = false;
        for (_, offers) in &self.offer_list {
            for offer_info in offers {
                for (name, v) in &offer_info.workers {
                    if !offer_info.workers.contains_key(name) {
                        info!("not contain key: {}", name);
                        flag = true;
                    }
                }
            }
        }

        if !flag {
            info!("all contain key ok");
        }
    }

    pub fn init_worker_action(&mut self) -> bool {
        self.init_workers();
        true
    }

    fn creeps_do_work_impl(
        workers: &mut HashMap<String, WorkerState>,
        work_type: &WorkType,
        lose_creeps: &mut Vec<String>,
        creeps: &HashMap<String, screeps::objects::Creep>,
    ) {
        match &work_type {
            WorkType::BuildAll(v) | WorkType::PointToPoint(v) => {
                for (name, state) in workers {
                    if let Some(creep) = creeps.get(&name.clone()) {
                        if creep.spawning() {
                            continue;
                        }
                        Manager::creep_do_p2p_work(creep, state, v);
                    } else {
                        warn!("not find creep name :{}, workers: ", name);
                        lose_creeps.push(name.clone());
                    }
                }
            }
            WorkType::CleanRoom => info!("offer CleanRoom not implement"),
            WorkType::UnKnown => info!("offer Unknown Type"),
        };
    }

    pub fn creeps_do_work(&mut self) {
        let creeps: HashMap<String, screeps::objects::Creep> = screeps::game::creeps::keys()
            .into_iter()
            .zip(screeps::game::creeps::values().into_iter())
            .collect();

        let mut lose_creeps = Vec::new();

        for offers in self.offer_list.values_mut() {
            for offer_info in offers {
                let workers = &mut offer_info.workers;
                let mut offer_type = &offer_info.offer_type;

                if offer_info.pausing {
                    let mut current_group = offer_info.next_offer;
                    loop {
                        match current_group {
                            None => break,
                            Some(v) => {
                                if v.pausing {
                                    current_group = v.next_offer;
                                } else {
                                    offer_type = &v.offer_type;
                                    break;
                                }
                            }
                        }
                    }
                    if let None = current_group {
                        continue;
                    }
                }

                Manager::creeps_do_work_impl(workers, offer_type, &mut lose_creeps, &creeps);
            }
        }

        if !lose_creeps.is_empty() {
            self.check_creep_contains();

            for name in &lose_creeps {
                self.fire_creep(name);
            }
        }
    }
}
