use std::collections::{HashSet, HashMap};

use screeps::{find, prelude::*, ReturnCode};

use super::{Manager, NORMAL_CREEP_BODY_INFO};

impl Manager {
    pub fn generator_init(&mut self) -> bool {
        self.init_source_info();

        self.init_office_info();

        self.update_creep_number();

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
                    basic_info: super::ObjectBasicInfo {
                        obj_type: super::ScreepsObjectType::Source,
                        id: source.id(),
                        name: source.id(),
                        pos: super::Position {
                            x: source.pos().x(),
                            y: source.pos().y(),
                        },
                    },
                    spawn_name,
                });
                worker_max *= 3;
            }
        }
    }

    fn init_spawn_office(&mut self) {
        let office_level = 1;
        self.office_list.insert(office_level, HashMap::new());
        let offices = self.office_list.get_mut(&office_level).unwrap();
        let mut employ_info = super::ObjectEmployInfo::new();

        for spawn in &Manager::get_my_spawns() {

            let self_info = super::ObjectBasicInfo {
                obj_type: super::ScreepsObjectType::Spawn,
                name: spawn.name(),
                id: spawn.id(),
                pos: super::Position {
                    x: spawn.pos().x(),
                    y: spawn.pos().y(),
                },
            };

            let mut range = std::u32::MAX;
            let mut find_id = &String::new();

            for (id, source) in &self.sources_info {
                let diff = self_info.pool_diff_range(&source.basic_info);
                if range > diff {
                    range = diff;
                    find_id = id;
                }
            }


            employ_info.employ_type = super::ObjectEmployType::PointToPoint(PointToPointWorkInfo {
                source: self.sources_info[find_id].basic_info.clone(),
                target: self_info.clone(),
            });


            employ_info.at_least_number = 0;
            employ_info.max_number = 0;
            employ_info.normal_number = 0;

            offices.insert(String::from("spawn")+ find_id, employ_info.clone());

            for (id, source) in &self.sources_info {
                if find_id == id{
                    continue;
                }

                employ_info.employ_type = super::ObjectEmployType::PointToPoint(PointToPointWorkInfo {
                    source: source.basic_info.clone(),
                    target: self_info.clone(),
                });

                employ_info.at_least_number = 2;
                employ_info.max_number = 10;
                employ_info.normal_number = 8;

                offices.insert(String::from("spawn")+ id, employ_info.clone());
            }
        }
    }

    fn init_controller_office(&mut self) {
        let office_level = 2;
        self.office_list.insert(office_level, HashMap::new());
        let offices = self.office_list.get_mut(&office_level).unwrap();
        let mut employ_info = super::ObjectEmployInfo::new();
        let rooms: &Vec<screeps::objects::Room> = &screeps::game::rooms::values();
        for room in rooms {
            let controller:&screeps::objects::StructureController = &room.controller().unwrap();
            if controller.my(){
                let self_info = super::ObjectBasicInfo {
                    obj_type: super::ScreepsObjectType::Controller,
                    name: controller.id(),
                    id: controller.id(),
                    pos: super::Position {
                        x: controller.pos().x(),
                        y: controller.pos().y(),
                    },
                };

                let mut range = std::u32::MAX;
                let mut find_id = &String::new();

                for (id, source) in &self.sources_info {
                    let diff = self_info.pool_diff_range(&source.basic_info);
                    if range > diff {
                        range = diff;
                        find_id = id;
                    }
                }

                employ_info.employ_type = super::ObjectEmployType::PointToPoint(PointToPointWorkInfo {
                    source: self.sources_info[find_id].basic_info.clone(),
                    target: self_info.clone(),
                });

                employ_info.at_least_number = 4;
                employ_info.max_number = 8;
                employ_info.normal_number = 6;

                offices.insert(String::from("controller")+ find_id, employ_info.clone());

                for (id, source) in &self.sources_info {
                    if find_id == id{
                        continue;
                    }

                    employ_info.employ_type = super::ObjectEmployType::PointToPoint(PointToPointWorkInfo {
                        source: source.basic_info.clone(),
                        target: self_info.clone(),
                    });

                    employ_info.at_least_number = 0;
                    employ_info.max_number = 10;
                    employ_info.normal_number = 5;

                    offices.insert(String::from("controller")+ id, employ_info.clone());
                }
            }
        }
    }
    fn init_office_info(&mut self) {
        self.init_spawn_office();

        self.init_controller_office();

    }

    fn update_creep_number(&mut self) {

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
                self.died_clean_creep(&mem_name);
            }
        }

        Ok(())
    }

    fn check_clean_memory(&mut self) {
        if screeps::game::time() % 32 == 3 {
            self.cleanup_memory().expect("expected Memory.creeps format to be a regular memory object");
        }
    }

    pub fn check_create_creep(&mut self) {
        self.check_clean_memory();

        if self.workers_info.len() > 20{
            return;
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
                    self.born_init_creep(&name);
                }
            }
        }
    }

    pub fn died_clean_creep(&mut self, name:&String){
        for (_,offers) in &mut self.office_list {
            for (_, offer) in offers {
                if offer.worker.contains(name){
                    offer.worker.remove(name);
                    self.workers_info.remove(name);
                    info!("died creep: {}", name);
                }
            }
        }
    }

    fn offer_creep_level(&mut self, name:&String, level:usize) -> bool{
        for (_,offers) in &mut self.office_list{
            for (_,offer) in offers {
                let level = match level {
                    1 => offer.at_least_number,
                    2 => offer.normal_number,
                    3 => offer.max_number,
                    _ => offer.at_least_number,
                };
                if offer.worker.len() < level{
                    match &offer.employ_type {
                        super::ObjectEmployType::PointToPoint(sr_info)=>{
                            self.workers_info.insert(name.clone(),super::WorkerInfo{
                                sr_info:sr_info.clone(),
                                state: super::WorkerState::DoSourceWork
                            });
                            offer.worker.insert(name.clone());
                            info!("new creep:{}, source:{}, target:{}",name, sr_info.source.name, sr_info.target.name);
                            return true;
                        }

                        _ =>{
                            warn!("not support offer type");
                        }
                    }

                }
            }
        }

        false
    }

    pub fn offer_creep(&mut self, name:&String){
        if self.offer_creep_level(name,1){
            return
        }

        if self.offer_creep_level(name, 2){
            return
        }

        if self.offer_creep_level(name, 3){
            return
        }

        warn!("offer failed!!!!!!!!!!!!");

    }

    pub fn born_init_creep(&mut self, name:&String){

        self.offer_creep(name);
    }
}