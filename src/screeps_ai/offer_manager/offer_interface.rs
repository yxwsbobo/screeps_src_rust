use core::borrow::BorrowMut;
use screeps_ai::get_offer_manager;
use screeps_ai::offer_manager::WorkType::PointToPoint;
use screeps_ai::offer_manager::{GroupEmployInfo, Manager, WorkType, WorkerState};
use std::collections::{BTreeMap, HashMap};

impl Manager {
    pub fn new() -> Manager {
        Manager {
            offer_list: BTreeMap::new(),
            current_number: 0,
            max_number: 0,
            spawn_offers: HashMap::new(),
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

    pub fn offer_creep(&mut self, name: &String, offer: &mut GroupEmployInfo) {
        offer
            .workers
            .insert(name.clone(), WorkerState::DoSourceWork);
        self.current_number += 1;

        info!("new creep:{}, offer type:{:#?}", name, offer.offer_type);
    }

    pub fn fire_creep(&mut self, name: &String) {
        for (_, offers) in &mut self.offer_list {
            for offer in offers {
                if offer.workers.contains_key(name) {
                    offer.workers.remove(name);
                    self.current_number -= 1;
                    info!("died creep: {}", name);
                    return;
                }
            }
        }

        warn!("fire creep error, not find the creep: {}!", name);
    }

    //
    //    fn offer_creep_level(&mut self, name:&String, level:usize) -> bool{
    //        //Todo
    ////        for (offer_level,offers) in &mut self.offer_list {
    ////            for (_,offer) in offers {
    ////                let level = match level {
    ////                    1 => offer.at_least_number,
    ////                    2 => offer.normal_number,
    ////                    3 => offer.max_number,
    ////                    _ => offer.at_least_number,
    ////                };
    ////                if offer.workers.len() < level{
    ////
    ////                    self.workers_info.insert(name.clone(),WorkerInfo{
    ////                        info: offer.offer_type.clone(),
    ////                        state: WorkerState::DoSourceWork,
    ////                        offer_level: *offer_level,
    ////                    });
    ////                    offer.workers.insert(name.clone(),WorkerState::DoSourceWork);
    ////                    info!("new creep:{}, offer:{:#?}",name, offer.offer_type);
    ////                    return true;
    ////                }
    ////            }
    ////        }
    //
    //        false
    //    }
    //
    //    pub fn make_creep_stupid(&mut self, name:String){
    //    }
    //
    //
    //
    //
    //
    ////    fn each_target_offer_do(&mut self, id:&str, op:fn(&mut GroupEmployInfo)->bool){
    ////        for (_,current_list) in self.offer_list.borrow_mut() {
    ////            for (_,offer) in current_list {
    ////                if let PointToPoint(info) = &mut offer.offer_type{
    ////                    if info.target == id{
    ////                        if op(offer){
    ////                            return
    ////                        }
    ////                    }
    ////                }
    ////            }
    ////        }
    ////    }
    //
    //    fn find_can_use_offer(&self)-> &GroupEmployInfo{
    //        for (_,current_list) in &self.offer_list {
    //            for (_,offer) in current_list {
    //                if !offer.pausing {
    //                    return offer;
    //                }
    //            }
    //        }
    //        panic!("not find offer can use");
    //    }
    //
    //    pub fn pause_group_offer(&mut self, id:&str){
    ////        self.each_target_offer_do(id, |offer|{
    ////            if offer.pausing{
    ////                return true;
    ////            }
    ////            offer.pausing = true;
    ////            for name in &offer.workers {
    ////                let worker = get_offer_manager().workers_info.get_mut(name)
    ////                    .expect("not find worker");
    ////                info!("pause old :{:#?}", worker);
    ////                worker.info =get_offer_manager().find_can_use_offer().offer_type.clone();
    ////                info!("pause and change to: {:#?}",worker);
    ////            };
    ////
    ////            false
    ////        });
    //    }
    //
    //    pub fn resume_group_offer(&mut self, id:&str){
    ////        self.each_target_offer_do(id, |offer|{
    ////            if !offer.pausing {
    ////                return true;
    ////            }
    ////            offer.pausing = false;
    ////
    ////            //Todo how to fix this
    ////            for name in &offer.workers {
    ////                let worker = get_offer_manager().workers_info.get_mut(name)
    ////                    .expect("not find worker");
    ////                info!("resume old :{:#?}", worker);
    ////                worker.info =offer.offer_type.clone();
    ////                info!("resume and back to: {:#?}",worker);
    ////
    ////            };
    ////
    ////            false
    ////        });
    //    }
}
