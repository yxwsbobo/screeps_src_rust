mod generator;

use screeps::Part;
//use std::collections::{HashMap};

//#[derive(Debug,Clone)]
//pub enum EnergySourceType{
//    EnergySource,
//    Extension,
//    Container,
//}
//
//#[derive(Debug,Clone)]
//pub struct EnergySourceInfo{
//    pub id:String,
//    pub source_type: EnergySourceType,
//    pub current_number:usize,
//    pub worker_max:usize,
//    pub spawn_id:String,
//}

pub struct Manager {
    init_flag: bool,
    normal_body: &'static (&'static [Part], u32),
//    total_creep_cost
    //key is id,
    //    sources_info:HashMap<String,EnergySourceInfo>,
    //    level:usize,
}

const NORMAL_CREEP_BODY: ([Part; 4], [Part; 6], [Part; 14]) = (
    [Part::Work, Part::Work, Part::Carry, Part::Move],
    [
        Part::Work,
        Part::Work,
        Part::Work,
        Part::Work,
        Part::Carry,
        Part::Move,
    ],
    [
        Part::Work,
        Part::Work,
        Part::Work,
        Part::Work,
        Part::Work,
        Part::Work,
        Part::Carry,
        Part::Carry,
        Part::Carry,
        Part::Move,
        Part::Move,
        Part::Move,
        Part::Move,
        Part::Move,
    ],
);

const NORMAL_CREEP_BODY_INFO: [(&[Part], u32); 3] = [
    (&NORMAL_CREEP_BODY.0, 300),
    (&NORMAL_CREEP_BODY.1, 500),
    (&NORMAL_CREEP_BODY.2, 1000),
];

impl Manager {
    pub fn new() -> Manager {
        Manager {
            init_flag: false,
            normal_body: &NORMAL_CREEP_BODY_INFO[0],
            //            level:0,
            //            sources_info:HashMap::new(),
        }
    }

    pub fn init(&mut self) -> bool {
        if self.init_flag {
            return true;
        }
        self.init_flag = self.generator_init();

        self.init_flag
    }
}
