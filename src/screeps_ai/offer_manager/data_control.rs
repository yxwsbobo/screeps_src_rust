
use std::collections::HashSet;
use screeps_ai::offer_manager::{Manager, GroupEmployInfo, WorkType};

impl Manager {
    pub fn delete_deploy_info_use_flag(&mut self,flag:String){
        //Todo
//        let level;
//        for office in &mut self.office_list {
//
//        }
    }

//    pub fn
}

impl GroupEmployInfo {
    pub fn new() -> GroupEmployInfo{
        GroupEmployInfo{
            pausing:false,
            workers:HashSet::new(),
            at_least_number:0,
            normal_number:2,
            max_number:4,
            offer_type:WorkType::UnKnown,
            flag:None,
        }
    }
//
//    pub fn turn_back_workers(&mut self){
//        for worker in &self.workers {
//
//        }
//    }
}

