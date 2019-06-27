mod offer_interface;

use std::collections::HashSet;

#[derive(Clone)]
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

#[derive(Clone)]
enum WorkType{
    _UnKnown,
    PointToPoint(PointToPointWorkInfo),
    CleanRoom,
}

#[derive(Clone)]
struct WorkerInfo{
    info:WorkType,
    state:WorkerState,
}

#[derive(Clone)]
struct ObjectEmployInfo{
    nothing_to_do:bool,
    worker:HashSet<String>,
    at_least_number:usize,
    normal_number:usize,
    max_number:usize,
    employ_type:ObjectEmployType,
    flag:Option<String>,
}


#[derive(Debug,Clone)]
struct EnergySourceInfo{
    id:String,
    current_number:usize,
    worker_max:usize,
    last_energy:usize,
    spawn_name:String,
}

pub struct Manager{

}