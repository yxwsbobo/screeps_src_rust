
mod generator;

use screeps::{Part};
use std::collections::{HashMap, BTreeMap, HashSet};

#[derive(Debug,Clone)]
struct EnergySourceInfo{
    id:String,
    current_number:usize,
    worker_max:usize,
    last_energy:usize,
    spawn_name:String,
}

pub struct Manager {
    init_flag:bool,
    level:usize,
    //key is id,
    sources_info:HashMap<String,EnergySourceInfo>,
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
            init_flag:false,
            level:0,
            sources_info:HashMap::new(),

        }
    }

    pub fn init(&mut self) -> bool{
        if self.init_flag {
            return true;
        }
        self.init_flag = self.generator_init();

        self.init_flag
    }

}