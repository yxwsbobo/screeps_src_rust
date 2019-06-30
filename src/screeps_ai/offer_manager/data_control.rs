use screeps_ai::offer_manager::{GroupEmployInfo, Manager, WorkType};
use std::collections::HashMap;

impl Manager {
    pub fn delete_deploy_info_use_flag(&mut self, flag: String) {
        //Todo
        //        let level;
        //        for office in &mut self.office_list {
        //
        //        }
    }

    //    pub fn
}

impl GroupEmployInfo {
    pub fn new() -> GroupEmployInfo {
        GroupEmployInfo {
            pausing: false,
            workers: HashMap::new(),
            max_number: 0,
            next_offer: None,
            offer_type: WorkType::UnKnown,
        }
    }
    //
    //    pub fn turn_back_workers(&mut self){
    //        for worker in &self.workers {
    //
    //        }
    //    }
}
