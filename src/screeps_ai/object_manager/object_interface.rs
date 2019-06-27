use screeps_ai::object_manager::{Manager, ObjectBasicInfo, ScreepsObjectType};
use std::collections::HashMap;
use screeps::{find, HasPosition,HasId};

impl Manager {
    pub fn new() -> Manager {
        Manager {
            objects:HashMap::new(),
//            cost_to_source:HashMap::new(),

        }
    }

    fn init_objects_in_room(&mut self){
        let rooms: &Vec<screeps::objects::Room> = &screeps::game::rooms::values();
        for room in rooms {
            {
                //controller
                if let Some(controller) = &room.controller(){
                    let id = controller.id();
                    let pos = &controller.pos();

                    self.objects.insert(id.clone(), ObjectBasicInfo {
                        obj_type:ScreepsObjectType::Controller,
                        name: id.clone(),
                        id: id.clone(),
                        pos: super::Position {
                            x: pos.x(),
                            y: pos.y(),
                        }
                    });
                }
            }

            {
                //sources
                let sources: &Vec<screeps::objects::Source> = &room.find(find::SOURCES);
                for source in sources {
                    let id = source.id();
                    let pos = &source.pos();

                    self.objects.insert(id.clone(),ObjectBasicInfo {
                        obj_type: super::ScreepsObjectType::Source,
                        id: id.clone(),
                        name: id.clone(),
                        pos: super::Position {
                            x: pos.x(),
                            y: pos.y(),
                        },
                    });
                }
            }

            //next room
        }
    }

    pub fn init_objects_spawns(&mut self){
        for spawn in &screeps::game::spawns::values() {
            let pos = &spawn.pos();

            self.objects.insert(spawn.id(),ObjectBasicInfo {
                obj_type: ScreepsObjectType::Spawn,
                name: spawn.name(),
                id: spawn.id(),
                pos: super::Position {
                    x: pos.x(),
                    y: pos.y(),
                },
            });
        }
    }

    pub fn init(&mut self) -> bool{
        self.init_objects_in_room();
        self.init_objects_spawns();

        true
    }

    pub fn get_objects(&self)->&HashMap<String,ObjectBasicInfo>{
        &self.objects
    }

    pub fn get_object(&self, id:&str)->&ObjectBasicInfo{
        //Todo add on not exist
        self.objects.get(id).unwrap()
    }
}


impl ObjectBasicInfo{
    pub fn pool_diff_range(&self, target:&ObjectBasicInfo)->u32{
        self.pos.range_to(&target.pos)
    }
}