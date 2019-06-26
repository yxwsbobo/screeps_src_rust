use super::{Manager,NORMAL_CREEP_BODY_INFO};

use screeps::{find, prelude::*, ReturnCode};
use std::collections::HashSet;

impl Manager {
    pub fn generator_init(&mut self)-> bool{
        self.init_work_info();
        self.update_creep_number();
        true
    }

    fn init_work_info(&self){

    }

    fn update_creep_number(&mut self){
//        let rooms:&Vec<screeps::objects::Room> = &screeps::game::rooms::values();
//        for room in rooms{
//            let sources:&Vec<screeps::objects::Source> = &room.find(find::SOURCES);
//            for source in sources{
//                let spawn_name = source.pos().find_closest_by_range(find::MY_SPAWNS).unwrap().name();
//
//                self.source_info.insert(source.id(),super::EnergySourceInfo{
//                    worker_number:0,
//                    worker_max:4,
//                    spawn_name,
//                });
//            }
//
//            let controller:&screeps::objects::StructureController = &room.controller().unwrap();
//            if controller.my(){
//                let source_id:String = controller.pos().find_closest_by_range(find::SOURCES).unwrap().id();
//                self.source_info.get_mut(&source_id).unwrap().worker_max +=2;
//
//                self.controller_info.insert(controller.id(), super::StructureInfo{
//                    worker_number:0,
//                    worker_max:2,
//                    source_id
//                });
//            }
//
//        }
//
//        self.worker_max = 10;
//        self.worker_count = screeps::game::creeps::values().len();
//
//        info!("new init");

    }

    fn get_my_spawns(&self)-> Vec<screeps::objects::StructureSpawn>{
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
            }
        }

        Ok(())
    }

    fn check_clean_memory(&mut self){
        if screeps::game::time() % 32 == 3 {
            info!("running memory cleanup");
            self.cleanup_memory().expect("expected Memory.creeps format to be a regular memory object");
        }
    }

    pub fn check_create_creep(&mut self){
//        self.check_clean_memory();
//
//        if self.worker_count >= self.worker_max{
//            return;
//        }
//        info!("current: {}, max: {}", self.worker_count, self.worker_max);
//
//
//        let body_info = &NORMAL_CREEP_BODY_INFO[self.level];
//
//        for spawn in &self.get_my_spawns() {
//
//            if spawn.energy() >=body_info.1 {
//
//                // create a unique name, spawn.
//                let name = format!("{}-{}", spawn.name(), screeps::game::time());
//                let res = spawn.spawn_creep(body_info.0, &name);
//                self.worker_count +=1;
//
//                if res != ReturnCode::Ok {
//                    warn!("couldn't spawn: {:?}", res);
//                }else{
//                    if self.worker_count > 2 {
//                        self.worker_info.insert(name,super::WorkerInfo{
//                            target:format!("controller")
//                        });
//                    }else{
//                        self.worker_info.insert(name,super::WorkerInfo{
//                            target:spawn.name()
//                        });
//                    }
//                }
//            }
//        }
    }
}