use screeps_ai::creep_manager::{Manager, ObjectEmployInfo};
use std::collections::HashSet;

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

impl ObjectEmployInfo {
    pub fn new() -> ObjectEmployInfo{
        ObjectEmployInfo{
            nothing_to_do:false,
            worker:HashSet::new(),
            at_least_number:0,
            normal_number:2,
            max_number:4,
            employ_type:super::ObjectEmployType::Unknown,
            flag:None,
        }
    }
}

impl ObjectBasicInfo{
    pub fn pool_diff_range(&self, target:&ObjectBasicInfo)->u32{
        pool_calculate_range(&self.pos, &target.pos)
    }
}