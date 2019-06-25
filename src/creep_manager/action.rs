use creep_manager::Manager;

use screeps::{find, prelude::*, ReturnCode, RoomObjectProperties};

impl Manager {
    pub fn action_init(&self) ->i32{

        0
    }

    pub fn creep_do_work(&mut self){
        for creep in screeps::game::creeps::values() {
            let name = creep.name();
            debug!("running creep {}", name);
            if creep.spawning() {
                continue;
            }

            if creep.memory().bool("harvesting") {
                if creep.carry_total() == creep.carry_capacity() {
                    creep.memory().set("harvesting", false);
                }
            } else {
                if creep.carry_total() == 0 {
                    creep.memory().set("harvesting", true);
                }
            }

            if creep.memory().bool("harvesting") {
                let source = &creep.room().find(find::SOURCES)[0];
                if creep.pos().is_near_to(source) {
                    let r = creep.harvest(source);
                    if r != ReturnCode::Ok {
                        warn!("couldn't harvest: {:?}", r);
                    }
                } else {
                    creep.move_to(source);
                }
            } else {
                if let Some(c) = creep.room().controller() {
                    let r = creep.upgrade_controller(&c);
                    if r == ReturnCode::NotInRange {
                        creep.move_to(&c);
                    } else if r != ReturnCode::Ok {
                        warn!("couldn't upgrade: {:?}", r);
                    }
                } else {
                    warn!("creep room has no controller!");
                }
            }
        }
    }
}