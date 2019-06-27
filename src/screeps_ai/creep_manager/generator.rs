use std::collections::{HashSet, HashMap};

use screeps::{find, prelude::*, ReturnCode};

use super::{Manager, NORMAL_CREEP_BODY_INFO};
use screeps_ai::get_offer_manager;

impl Manager {
    pub fn generator_init(&mut self) -> bool {
        self.init_source_info();

        true
    }

    fn init_source_info(&mut self) {
        let rooms: &Vec<screeps::objects::Room> = &screeps::game::rooms::values();
        for room in rooms {
            let mut worker_max = 5;
            let sources: &Vec<screeps::objects::Source> = &room.find(find::SOURCES);
            for source in sources {
                let spawn_name = source.pos().find_closest_by_range(find::MY_SPAWNS).unwrap().name();

                self.sources_info.insert(source.id(), super::EnergySourceInfo {
                    current_number: 0,
                    last_energy: 0,
                    worker_max,
                    id:source.id(),
                    spawn_name,
                });
                worker_max *= 3;
            }
        }
    }

    fn get_my_spawns() -> Vec<screeps::objects::StructureSpawn> {
        screeps::game::spawns::values()
    }

    fn cleanup_memory(&mut self) -> Result<(), Box<dyn(::std::error::Error)>> {
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
                get_offer_manager().fire_creep(&mem_name);
            }
        }

        Ok(())
    }

    fn check_clean_memory(&mut self) {
        if screeps::game::time() % 32 == 3 {
            self.cleanup_memory().expect("expected Memory.creeps format to be a regular memory object");
        }
    }

    fn is_need_workers(&self) ->bool{
        for (_, info) in &self.sources_info {
            if info.current_number < info.worker_max{
                return true;
            }
        }

        false
    }

    pub fn check_create_creep(&mut self) {
        self.check_clean_memory();

        if !self.is_need_workers() {
            return
        }

        let body_info = &NORMAL_CREEP_BODY_INFO[self.level];

        for spawn in &Manager::get_my_spawns() {

            if spawn.energy() >=body_info.1 {

                // create a unique name, spawn.
                let name = format!("{}-{}", spawn.name(), screeps::game::time());
                let res = spawn.spawn_creep(body_info.0, &name);

                if res != ReturnCode::Ok {
                    warn!("couldn't spawn: {:?}", res);
                }else{
                    get_offer_manager().offer_creep(&name);
                }
            }
        }
    }

}