
mod generator;
mod action;

use screeps::{Part};
use std::collections::HashMap;

struct WorkerInfo{
    target:Vec<String>
}

struct StructureInfo{
    worker_number:usize,
    source:String,
}

struct EnergySourceInfo{
    worker_number:usize,
    spawn_name:String,
}

pub struct Manager {
    level:usize,
//    building_info:HashMap<String,StructureInfo>,
//    controller_info:HashMap<String,StructureInfo>,
//    storage_info:HashMap<String,StructureInfo>,
    source_info:HashMap<String,EnergySourceInfo>,
    worker_info:HashMap<String,WorkerInfo>,
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
            source_info:HashMap::new(),
            worker_info:HashMap::new(),
        }
    }

    pub fn init(&self) -> bool{
        self.generator_init() && self.action_init()
    }

}