use std::collections::{HashSet, HashMap};

use screeps::{find, prelude::*, ReturnCode};

use super::{Manager, NORMAL_CREEP_BODY_INFO};
use screeps_ai::{get_offer_manager, object_manager, get_object_manager};
use screeps_ai::creep_manager::{EnergySourceInfo, EnergySourceType};

impl Manager {
    pub fn generator_init(&mut self) -> bool {
        self.init_source_info();

        true
    }

    fn insert_source(&mut self,source:&screeps::objects::Source, worker_max:usize){
        let spawn_id = source.pos().find_closest_by_range(find::MY_SPAWNS).unwrap().id();

        self.sources_info.insert(source.id(), EnergySourceInfo {
            current_number: 0,
            last_energy: 0,
            worker_max,
            id:source.id(),
            source_type: EnergySourceType::EnergySource,
            spawn_id,
        });
    }

    fn init_source_info(&mut self) {
        let rooms: &Vec<screeps::objects::Room> = &screeps::game::rooms::values();
        for room in rooms {
            let mut worker_max = 6;
            let sources: &Vec<screeps::objects::Source> = &room.find(find::SOURCES);
            for source in sources {
                self.insert_source(source, worker_max);
                worker_max *= 3;
            }
        }
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

        for spawn in &object_manager::Manager::get_my_spawns() {

            if spawn.energy() >=body_info.1 {

                // create a unique name, spawn.
                let name = format!("{}-{}", spawn.name(), screeps::game::time());
                let res = spawn.spawn_creep(body_info.0, &name);

                if res != ReturnCode::Ok {
                    warn!("couldn't spawn: {:?}", res);
                }else{
                    get_offer_manager().resume_group_offer(&spawn.id());
                    get_offer_manager().offer_creep(&name);
                }
            }
        }
    }

    pub fn get_closest_source(&self,id:&str)->String{
        let mut range = std::u32::MAX;
        let mut find_id = &String::new();
        let target = get_object_manager().get_object(id);
        for id in self.sources_info.keys() {
            let source = get_object_manager().get_object(id);
            let dif = source.pool_diff_range(target);
            if range > dif{
                range = dif;
                find_id = &source.id;
            }
        }

        find_id.clone()
    }

    pub fn get_sources(&self)->&HashMap<String,EnergySourceInfo>{
        &self.sources_info
    }

}