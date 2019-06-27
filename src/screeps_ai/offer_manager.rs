mod offer_interface;
mod data_control;
mod action;
mod default_offer;

use std::collections::{HashSet, BTreeMap, HashMap};

#[derive(Debug,Clone)]
struct PointToPointWorkInfo{
    source:String,
    target:String
}

#[derive(Clone)]
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

#[derive(Clone)]
pub(crate) struct WorkerInfo{
    info:WorkType,
    state:WorkerState,
}

#[derive(Clone)]
struct GroupEmployInfo{
    nothing_to_do:bool,
    workers:HashSet<String>,
    at_least_number:usize,
    normal_number:usize,
    max_number:usize,
    offer_type:WorkType,
    flag:Option<String>,
}


pub struct Manager{
    office_list:BTreeMap<i32,HashMap<String,GroupEmployInfo>>,
    workers_info:HashMap<String,WorkerInfo>,

}