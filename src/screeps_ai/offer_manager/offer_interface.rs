use screeps_ai::offer_manager::{Manager, WorkerInfo, WorkType, WorkerState};
use std::collections::{BTreeMap, HashMap};

impl Manager {
    pub fn new()-> Manager{
        Manager{
            office_list:BTreeMap::new(),
            workers_info:HashMap::new(),
        }
    }

    pub fn init(&mut self)->bool{
        self.init_default_offers();
        self.init_worker_action();

        true
    }


    fn offer_creep_level(&mut self, name:&String, level:usize) -> bool{
        //Todo
        for (_,offers) in &mut self.office_list{
            for (_,offer) in offers {
                let level = match level {
                    1 => offer.at_least_number,
                    2 => offer.normal_number,
                    3 => offer.max_number,
                    _ => offer.at_least_number,
                };
                if offer.workers.len() < level{

                    self.workers_info.insert(name.clone(),WorkerInfo{
                        info: offer.offer_type.clone(),
                        state: WorkerState::DoSourceWork,
                    });
                    info!("new creep:{}, offer:{:#?}",name, offer.offer_type);
                    return true;
                }
            }
        }

        false
    }

    pub fn offer_creep(&mut self, name:&String){
        if self.offer_creep_level(name,1){
            return
        }

        if self.offer_creep_level(name, 2){
            return
        }

        if self.offer_creep_level(name, 3){
            return
        }

        warn!("offer failed!!!!!!!!!!!!");
    }

    pub fn fire_creep(&mut self, name:&String){
        for (_,offers) in &mut self.office_list {
            for (_, offer) in offers {
                if offer.workers.contains(name){
                    offer.workers.remove(name);
                    self.workers_info.remove(name);
                    info!("died creep: {}", name);
                }
            }
        }
    }
}