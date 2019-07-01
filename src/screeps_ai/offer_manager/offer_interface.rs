use screeps_ai::offer_manager::{GroupEmployInfo, Manager, WorkerState};
use screeps_ai::{creep_manager};
use std::collections::{BTreeMap};

impl Manager {
    pub fn new() -> Manager {
        Manager {
            offer_list: BTreeMap::new(),
            current_number: 0,
            max_number: 0,
        }
    }

    pub fn init(&mut self) -> bool {
        self.init_default_offers();
        self.init_worker_action();
        true
    }

    pub fn check_worker_full(&self) -> bool {
        self.current_number >= self.max_number
    }

    pub fn find_next_offer_from_exist(
        &mut self,
        creep: &screeps::objects::Creep,
    ) -> Option<&mut GroupEmployInfo> {
        if self.check_worker_full() {
            return None;
        }
        for (_, offers) in &mut self.offer_list {
            for offer in offers {
                if offer.workers.len() < offer.max_number {
                    return Some(offer);
                }
            }
        }
        None
    }

    pub fn find_next_offer(
        &mut self,
        spawn: &screeps::objects::StructureSpawn,
    ) -> Option<&mut GroupEmployInfo> {
        if self.check_worker_full() {
            return None;
        }
        for (_, offers) in &mut self.offer_list {
            for offer in offers {
                if offer.workers.len() < offer.max_number {
                    return Some(offer);
                }
            }
        }
        None
    }

    pub fn offer_creep(&mut self, name: &str, offer: &mut GroupEmployInfo) {
        offer
            .workers
            .insert(name.to_string(), WorkerState::DoSourceWork);
        self.current_number += 1;

        info!("new creep:{}, offer type:{:#?}", name, offer.offer_type);
    }

    pub fn fire_creep(&mut self, name: &str) {
        for (_, offers) in &mut self.offer_list {
            for offer in offers {
                if offer.workers.contains_key(name) {
                    offer.workers.remove(name);
                    self.current_number -= 1;
                    info!("died creep: {}", name);
                    creep_manager::Manager::cleanup_memory(name);
                    return;
                }
            }
        }

        warn!("fire creep error, not find the creep: {}!", name);
    }
}
