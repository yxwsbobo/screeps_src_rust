mod offer_interface;
mod data_control;
mod action;
mod default_offer;

use std::collections::{BTreeMap, HashMap};
use screeps_ai::object_manager::ObjectBasicInfo;

#[derive(Debug,Clone)]
pub struct PointToPointWorkInfo{
    source:&'static ObjectBasicInfo,
    target:&'static ObjectBasicInfo,
}

#[derive(Clone, Debug)]
enum WorkerState{
    StupidWorker,
    //    MoveToSource,
    DoSourceWork,
    //    MoveToTarget,
    DoTargetWork,
}

#[derive(Debug,Clone)]
enum WorkType{
    UnKnown,
    PointToPoint(PointToPointWorkInfo),
    CleanRoom,
}

#[derive(Clone, Debug)]
pub struct WorkerInfo{
    info:WorkType,
    state:WorkerState,
    offer_level:i32,
}

#[derive(Clone,Debug)]
struct GroupEmployInfo{
    pausing:bool,
    workers:HashMap<String,WorkerState>,
    at_least_number:usize,
    normal_number:usize,
    max_number:usize,
    offer_type:WorkType,
    flag:Option<String>,
}


pub struct Manager{
    offer_list:BTreeMap<i32,HashMap<String,GroupEmployInfo>>,
    pub workers_info:HashMap<String,WorkerInfo>,

}