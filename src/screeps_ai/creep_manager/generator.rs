
use screeps::{prelude::*, ReturnCode, Part};

use super::{Manager};
use screeps_ai::{get_offer_manager, object_manager};
use screeps_ai::creep_manager::NORMAL_CREEP_BODY_INFO;
//use screeps_ai::creep_manager::{EnergySourceInfo, EnergySourceType};

impl Manager {
    pub fn generator_init(&mut self) -> bool {
        true
    }

    pub fn cleanup_memory(name: &str) {
        match screeps::memory::root()
            .dict("creeps")
            .expect("no memory.creeps")
        {
            Some(v) => v.del(&name),
            None => {
                warn!("not cleaning game creep memory: no Memory.creeps dict");
            }
        };
    }

    fn get_room_build_body(room:&screeps::objects::Room) ->(bool,&'static [Part]){
        if room.energy_available() > NORMAL_CREEP_BODY_INFO[0].1 {
            (true,&NORMAL_CREEP_BODY_INFO[0].0)
        }else{
            (false, &NORMAL_CREEP_BODY_INFO[0].0)
        }
    }

    pub fn check_create_creep(&mut self) {
        if get_offer_manager().check_worker_full() {
            return;
        }

        for spawn in &object_manager::Manager::get_my_spawns() {
            let body = Manager::get_room_build_body(&spawn.room());
            if body.0 {
                match get_offer_manager().find_next_offer(spawn) {
                    None => return,
                    Some(v) => {
                        // create a unique name, spawn.
                        let name = format!("{}-{}", spawn.name(), screeps::game::time());
                        let res = spawn.spawn_creep(body.1, &name);

                        if res != ReturnCode::Ok {
                            warn!("couldn't spawn: {:?}", res);
                        } else {
                            get_offer_manager().offer_creep(&name, v);
                        }
                    }
                }
            }
        }
    }
}
