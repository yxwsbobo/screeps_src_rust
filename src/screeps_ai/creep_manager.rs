
mod generator;
mod action;
mod data_control;

use screeps::{Part};
use std::collections::{HashMap, BTreeMap, HashSet};



pub struct Manager {
    init_flag:bool,
    level:usize,
    //key is id,
    sources_info:HashMap<String,EnergySourceInfo>,
    workers_info:HashMap<String,WorkerInfo>,


    office_list:BTreeMap<i32,HashMap<String,ObjectEmployInfo>>,

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
            init_flag:false,
            level:0,
            sources_info:HashMap::new(),
            workers_info:HashMap::new(),
            office_list:BTreeMap::new(),
            cost_to_source:HashMap::new(),
        }
    }

    pub fn init(&mut self) -> bool{
        if self.init_flag {
            return true;
        }
        self.init_flag = self.generator_init() && self.action_init();

        self.init_flag
    }

}