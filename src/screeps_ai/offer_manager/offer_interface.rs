use core::borrow::BorrowMut;
use screeps_ai::{creep_manager, get_object_manager};
use screeps_ai::offer_manager::{GroupEmployInfo, Manager, WorkerState};
use std::collections::BTreeMap;
use std::rc::Rc;
use screeps_ai::object_manager::ScreepsObjectType;

pub fn get_offer_mut(offer: &Rc<GroupEmployInfo>) -> &mut GroupEmployInfo {
    {
        unsafe { &mut *(&**offer as *const _ as *mut GroupEmployInfo) }
    }
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            offer_list: BTreeMap::new(),
            current_number: 0,
            max_number: 0,
            offer_level: 0,
            room_name: Default::default(),
            fight_flag: false
        }
    }

    fn compute_game_level(&self) -> i32{
        let extension_number = get_object_manager().structures_lists[ScreepsObjectType::Extension as usize].len();
        match extension_number {
            0 ..= 9 => 0,
            10 ..= 20 => 1,
            _ => 2,
        }
    }

    pub fn check_offer_level(&mut self) ->bool{
        let game_level = self.compute_game_level();
        if self.offer_level != game_level {
            self.offer_level = game_level;
            self.init_offer_and_worker();
            true
        }
        else{
            false
        }
    }

    fn init_offer_and_worker(&mut self){
        self.init_default_offers();
        self.init_worker_action();
    }

    pub fn init(&mut self) -> bool {
        for room in &screeps::game::rooms::values() {
            self.room_name = room.name();
            break;
        }
        self.offer_level = self.compute_game_level();

        self.init_offer_and_worker();
        true
    }

    pub fn check_in_survival(&self) -> bool {
        self.current_number <= 1
    }

    pub fn check_worker_full(&self) -> bool {
        self.current_number >= self.max_number
    }

    pub fn find_next_offer_from_exist(
        &mut self,
        creep: &screeps::objects::Creep,
    ) -> Option<&mut Rc<GroupEmployInfo>> {
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
    ) -> Option<&mut Rc<GroupEmployInfo>> {
        if self.check_worker_full() {
            return None;
        }
        for (level, offers) in &mut self.offer_list {
            for offer in offers {
                if offer.workers.len() < offer.max_number {
                    info!("new offer level: {}", level);
                    return Some(offer);
                }
            }
        }
        None
    }

    pub fn offer_creep(&mut self, name: &str, offer: &mut Rc<GroupEmployInfo>) {
        get_offer_mut(offer)
            .workers
            .insert(name.to_string(), WorkerState::DoSourceWork);
        self.current_number += 1;

        info!("new creep:{}, offer type:{:#?}", name, offer.offer_type);
    }

    pub fn fire_creep(&mut self, name: &str) {
        for (_, offers) in &mut self.offer_list {
            for offer in offers {
                if offer.workers.contains_key(name) {
                    get_offer_mut(offer).workers.remove(name);
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
