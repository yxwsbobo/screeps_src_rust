use creep_manager::{Manager,NORMAL_CREEP_BODY_INFO};

use screeps::{prelude::*, ReturnCode};
use std::collections::HashSet;

impl Manager {
    pub fn generator_init(&self)-> i32{
        self.update_creep_number();
        0
    }

    fn update_creep_number(&self){

    }

    fn get_my_spawns(&self){

    }

    fn cleanup_memory(&self) -> Result<(), Box<dyn(::std::error::Error)>> {
        let alive_creeps: HashSet<String> = screeps::game::creeps::keys().into_iter().collect();

        let screeps_memory = match screeps::memory::root().dict("creeps")? {
            Some(v) => v,
            None => {
                warn!("not cleaning game creep memory: no Memory.creeps dict");
                return Ok(());
            }
        };

        for mem_name in screeps_memory.keys() {
            if !alive_creeps.contains(&mem_name) {
                info!("cleaning up creep memory of dead creep {}", mem_name);
                screeps_memory.del(&mem_name);
            }
        }

        Ok(())
    }

    fn check_clean_memory(&self){
        let time = screeps::game::time();

        if time % 32 == 3 {
            info!("running memory cleanup");
            self.cleanup_memory().expect("expected Memory.creeps format to be a regular memory object");
        }
    }

    pub fn check_create_creep(&mut self){
        self.check_clean_memory();
        let body_info = &NORMAL_CREEP_BODY_INFO[self.level];

        for spawn in screeps::game::spawns::values() {
            let spawn: &screeps::objects::StructureSpawn = &spawn;

            if spawn.energy() >=body_info.1 {
                // create a unique name, spawn.
                let name = format!("{}-{}", spawn.name(), screeps::game::time());
                let res = spawn.spawn_creep(body_info.0, &name);

                if res != ReturnCode::Ok {
                    warn!("couldn't spawn: {:?}", res);
                }
            }
        }
    }
}