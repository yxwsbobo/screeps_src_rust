
mod generator;
mod action;
mod data_control;

use screeps::{Part};
use std::collections::{HashMap, BTreeMap, HashSet};

#[derive(Debug)]
enum ScreepsObjectType{
    Spawn,
    Creep,
    PowerCreep,
    Source,
    Mineral,
    Controller,
    ConstructedWall,
    Extension,
    Link,
    Storage,
    Tower,
    Observer,
    PowerSpawn,
    PowerBank,
    Lab,
    Terminal,
    Nuker,
}

#[derive(Debug)]
struct ObjectBasicInfo{
    obj_type:ScreepsObjectType,
    name:String,
    id:String,
}

struct PointToPointWorkInfo{
    source:ObjectBasicInfo,
    target:ObjectBasicInfo
}

enum WorkerState{
    StupidWorker,
//    MoveToSource,
    DoSourceWork,
//    MoveToTarget,
    DoTargetWork,
}

struct WorkerInfo{
    sr_info:PointToPointWorkInfo,
    state:WorkerState,
}

enum ObjectEmployType{
    //Type, name, id
    PointToPoint(PointToPointWorkInfo),
    CleanRoom,
//    SpawnEmploy(String),
//    ControllerEmploy(String),
//    ExtensionEmploy(String),
//    StorageEmploy(String),
//    TowerEmploy(String),
//    PowerSpawnEmploy(String),
//    PowerBankEmploy(String),
//    LabEmploy(String),
}

struct ObjectEmployInfo{
    nothing_to_do:bool,
    worker:HashSet<String>,
    at_least_number:usize,
    normal_number:usize,
    max_number:usize,
    employ_type:ObjectEmployType,
    flag:Option<String>,
}


#[derive(Debug)]
struct EnergySourceInfo{
    current_number:usize,
    worker_max:usize,
    last_energy:usize,
    basic_info:ObjectBasicInfo,
    spawn_name:String,
}

pub struct Manager {
    level:usize,

    //key is id,
    sources_info:HashMap<String,EnergySourceInfo>,
    workers_info:HashMap<String,WorkerInfo>,

    office_list:BTreeMap<i32,HashSet<ObjectEmployInfo>>,

    //BTreeMap<i32,String> cost,id
    cost_to_source:HashMap<String,BTreeMap<i32,String>>,
}

const NORMAL_CREEP_BODY:([Part;4],[Part;6]) = (
    [Part::Work, Part::Work, Part::Carry, Part::Move],
    [Part::Work,Part::Work, Part::Work, Part::Work, Part::Carry, Part::Move]);

const NORMAL_CREEP_BODY_INFO:[(&[Part],u32);2] =[
    (&NORMAL_CREEP_BODY.0,300),
    (&NORMAL_CREEP_BODY.1,500)
];


impl Manager {
    pub fn new() -> Manager{
        Manager{
            level:0,
            sources_info:HashMap::new(),
            workers_info:HashMap::new(),
            office_list:BTreeMap::new(),
            cost_to_source:HashMap::new(),
        }
    }

    pub fn init(&mut self) -> bool{
        self.generator_init() && self.action_init()
    }

}