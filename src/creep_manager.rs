
mod generator;
mod action;

use screeps::{Part};
use std::collections::HashMap;


static mut GLOBAL_MANAGER: Option<Manager> = None;

pub struct Manager {
    level:usize,
    worker_number:HashMap<String,usize>
}

pub fn get_manager() -> &'static mut Manager {
    unsafe {
        if let None = GLOBAL_MANAGER {
            GLOBAL_MANAGER = Some(Manager{
                level:0,
                worker_number:HashMap::new()
            })
        }

        GLOBAL_MANAGER.as_mut().unwrap()
    }
}

const NORMAL_CREEP_BODY:([Part;4],[Part;6]) = (
    [Part::Work, Part::Work, Part::Carry, Part::Move],
    [Part::Work,Part::Work, Part::Work, Part::Work, Part::Carry, Part::Move]);

const NORMAL_CREEP_BODY_INFO:[(&[Part],u32);2] =[
    (&NORMAL_CREEP_BODY.0,300),
    (&NORMAL_CREEP_BODY.1,500)
];


impl Manager {
    pub fn init(&self) -> i32{
        self.generator_init();
        self.action_init();
        0
    }

}