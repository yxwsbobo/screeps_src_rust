use screeps::{find, HasId, HasPosition, SizedRoomObject};
use screeps_ai::get_object_manager;
use screeps_ai::object_manager::{Manager, ObjectBasicInfo, ScreepsObjectType};
use std::collections::HashMap;
use std::path::Prefix::Verbatim;

impl Manager {
    pub fn get_my_spawns() -> Vec<screeps::objects::StructureSpawn> {
        screeps::game::spawns::values()
    }

    pub fn new() -> Manager {
        Manager {
            objects: HashMap::new(),
            sources_lists: Vec::new(),
            source_range: HashMap::new(),
            room_objects: HashMap::new(),
        }
    }

    pub fn clean_invalid_object(&mut self) {
        self.room_objects.clear();
    }

    fn init_objects_in_room(&mut self) {
        let rooms: &Vec<screeps::objects::Room> = &screeps::game::rooms::values();
        for room in rooms {
            {
                //sources
                let sources: &Vec<screeps::objects::Source> = &room.find(find::SOURCES);
                for source in sources {
                    let id = source.id();
                    let pos = &source.pos();

                    let basic_info = ObjectBasicInfo {
                        obj_type: super::ScreepsObjectType::Source,
                        id: id.clone(),
                        name: id.clone(),
                        pos: super::Position {
                            x: pos.x(),
                            y: pos.y(),
                        },
                    };
                    self.objects.insert(id.clone(), basic_info.clone());
                    self.sources_lists.push(basic_info);
                }
            }

            {
                //controller
                if let Some(controller) = &room.controller() {
                    let id = controller.id();
                    let pos = &controller.pos();

                    let my_pos = super::Position {
                        x: pos.x(),
                        y: pos.y(),
                    };

                    let mut sl = self.sources_lists.clone();
                    sl.sort_by_cached_key(|v| v.pos.range_to(&my_pos));
                    self.source_range.insert(controller.id(), sl);

                    self.objects.insert(
                        id.clone(),
                        ObjectBasicInfo {
                            obj_type: ScreepsObjectType::Controller,
                            name: id.clone(),
                            id: id.clone(),
                            pos: my_pos,
                        },
                    );
                }
            }

            //next room
        }
    }

    pub fn init_objects_spawns(&mut self) {
        for spawn in &screeps::game::spawns::values() {
            let pos = &spawn.pos();

            let my_pos = super::Position {
                x: pos.x(),
                y: pos.y(),
            };

            let mut sl = self.sources_lists.clone();
            sl.sort_by_cached_key(|v| v.pos.range_to(&my_pos));
            self.source_range.insert(spawn.id(), sl);

            self.objects.insert(
                spawn.id(),
                ObjectBasicInfo {
                    obj_type: ScreepsObjectType::Spawn,
                    name: spawn.name(),
                    id: spawn.id(),
                    pos: my_pos,
                },
            );
        }
    }

    pub fn init_objects_constructions(&mut self) {
        for construction in &screeps::game::construction_sites::values() {
            let pos = &construction.pos();
            let id = construction.id();

            let my_pos = super::Position {
                x: pos.x(),
                y: pos.y(),
            };

            let mut sl = self.sources_lists.clone();
            sl.sort_by_cached_key(|v| v.pos.range_to(&my_pos));
            self.source_range.insert(construction.id(), sl);

            self.objects.insert(
                id.clone(),
                ObjectBasicInfo {
                    obj_type: ScreepsObjectType::ConstructionSites,
                    name: id.clone(),
                    id,
                    pos: my_pos,
                },
            );
        }
    }

    pub fn init(&mut self) -> bool {
        self.init_objects_in_room();
        self.init_objects_spawns();
        self.init_objects_constructions();

        true
    }

    pub fn get_objects(&self) -> &HashMap<String, ObjectBasicInfo> {
        &self.objects
    }

    pub fn get_object(&self, id: &str) -> &ObjectBasicInfo {
        //Todo add on not exist
        self.objects.get(id).unwrap()
    }

    pub fn get_object_to_source(&self, object_id: &String) -> &Vec<ObjectBasicInfo> {
        &self
            .source_range
            .get(object_id)
            .expect("not find object to source!")
    }

    pub fn get_game_object(self: &mut Self, id: &String) -> &screeps::objects::RoomObject {
        //        screeps::game::get_object_erased(id).expect("get_game_object not found")

        self.room_objects
            .entry(id.clone())
            .or_insert(screeps::game::get_object_erased(id).expect("get_game_object not found"))
    }
}

impl ObjectBasicInfo {
    pub fn pool_diff_range(&self, target: &ObjectBasicInfo) -> u32 {
        self.pos.range_to(&target.pos)
    }
}
