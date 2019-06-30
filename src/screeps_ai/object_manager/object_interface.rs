use screeps::{find, HasId, HasPosition, SizedRoomObject, StructureProperties, StructureType};
use screeps_ai::common::Position;
use screeps_ai::object_manager::{Manager, ObjectBasicInfo, ScreepsObjectType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryInto;
use std::rc::Rc;

impl Manager {
    pub fn get_my_spawns() -> Vec<screeps::objects::StructureSpawn> {
        screeps::game::spawns::values()
    }

    pub fn new() -> Manager {
        Manager {
            objects: Default::default(),
            sources_lists: Default::default(),
            source_range: Default::default(),
            room_objects: Default::default(),
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

                    let basic_info = Rc::new(ObjectBasicInfo {
                        obj_type: ScreepsObjectType::Source,
                        id: id.clone(),
                        name: id.clone(),
                        pos: Position {
                            x: pos.x(),
                            y: pos.y(),
                        },
                    });
                    self.objects.insert(id.clone(), basic_info.clone());
                    self.sources_lists.push(basic_info);
                    //                    self.room_objects.insert(id.clone(),Rc::new(source.try_into()));
                }
            }

            {
                //controller
                if let Some(controller) = &room.controller() {
                    let id = controller.id();
                    let pos = &controller.pos();

                    let my_pos = Position {
                        x: pos.x(),
                        y: pos.y(),
                    };

                    let mut sl = self.sources_lists.clone();
                    sl.sort_by_cached_key(|v| v.pos.range_to(&my_pos));
                    self.source_range.insert(controller.id(), sl);

                    let basic_info = Rc::new(ObjectBasicInfo {
                        obj_type: ScreepsObjectType::Controller,
                        name: id.clone(),
                        id: id.clone(),
                        pos: my_pos,
                    });
                    self.objects.insert(id.clone(), basic_info);
                }
            }

            //next room
        }
    }

    pub fn init_objects_spawns(&mut self) {
        for spawn in &screeps::game::spawns::values() {
            let pos = &spawn.pos();

            let my_pos = Position {
                x: pos.x(),
                y: pos.y(),
            };

            let mut sl = self.sources_lists.clone();
            sl.sort_by_cached_key(|v| v.pos.range_to(&my_pos));
            self.source_range.insert(spawn.id(), sl);

            let basic_info = Rc::new(ObjectBasicInfo {
                obj_type: ScreepsObjectType::Spawn,
                name: spawn.name(),
                id: spawn.id(),
                pos: my_pos,
            });
            self.objects.insert(spawn.id(), basic_info);
        }
    }

    pub fn init_objects_constructions(&mut self) {
        //        for construction in &screeps::game::construction_sites::values() {
        //            let pos = &construction.pos();
        //            let id = construction.id();
        //
        //            let my_pos = Position {
        //                x: pos.x(),
        //                y: pos.y(),
        //            };
        //
        //            let mut sl = self.sources_lists.clone();
        //            sl.sort_by_cached_key(|v| v.pos.range_to(&my_pos));
        //            self.source_range.insert(construction.id(), sl);
        //
        //            self.objects.insert(
        //                id.clone(),
        //                ObjectBasicInfo {
        //                    obj_type: ScreepsObjectType::ConstructionSites,
        //                    name: id.clone(),
        //                    id,
        //                    pos: my_pos,
        //                },
        //            );
        //        }
    }

    pub fn init(&mut self) -> bool {
        self.init_objects_in_room();
        self.init_objects_spawns();
        self.init_objects_constructions();

        true
    }

    fn convert_type_from_game(o: &StructureType) -> ScreepsObjectType {
        match o {
            StructureType::Spawn => ScreepsObjectType::Spawn,
            StructureType::Extension => ScreepsObjectType::Extension,
            StructureType::Road => ScreepsObjectType::Road,
            StructureType::Wall => ScreepsObjectType::Wall,
            StructureType::Rampart => ScreepsObjectType::Rampart,
            StructureType::KeeperLair => ScreepsObjectType::KeeperLair,
            StructureType::Portal => ScreepsObjectType::Portal,
            StructureType::Controller => ScreepsObjectType::Controller,
            StructureType::Link => ScreepsObjectType::Link,
            StructureType::Storage => ScreepsObjectType::Storage,
            StructureType::Tower => ScreepsObjectType::Tower,
            StructureType::Observer => ScreepsObjectType::Observer,
            StructureType::PowerBank => ScreepsObjectType::PowerBank,
            StructureType::PowerSpawn => ScreepsObjectType::PowerSpawn,
            StructureType::Extractor => ScreepsObjectType::Extractor,
            StructureType::Lab => ScreepsObjectType::Lab,
            StructureType::Terminal => ScreepsObjectType::Terminal,
            StructureType::Container => ScreepsObjectType::Container,
            StructureType::Nuker => ScreepsObjectType::Nuker,
        }
    }

    pub fn get_object(&mut self, id: &str) -> Rc<ObjectBasicInfo> {
        //Todo add on not exist
        if let Some(v) = self.objects.get(id) {
            return v.clone();
        }

        let obj = self.get_game_object(id);
        let basic_info = Rc::new(ObjectBasicInfo {
            obj_type: Manager::convert_type_from_game(&obj.structure_type()),
            name: "".to_string(),
            id: "".to_string(),
            pos: Position {
                x: obj.pos().x(),
                y: obj.pos().y(),
            },
        });

        self.objects.insert(obj.id(), basic_info.clone());
        basic_info
    }

    pub fn get_object_to_source(&mut self, object_id: &str) -> Vec<Rc<ObjectBasicInfo>> {
        if let Some(v) = self.source_range.get(object_id) {
            return v.clone();
        }

        let my_pos = &self.get_object(object_id).pos;
        let mut sl = self.sources_lists.clone();
        sl.sort_by_cached_key(|v| v.pos.range_to(my_pos));
        self.source_range
            .insert(object_id.to_string(), sl)
            .expect("source_range insert failed")
    }

    pub fn get_game_object(self: &mut Self, id: &str) -> Rc<screeps::objects::RoomObject> {
        self.room_objects
            .entry(id.to_string())
            .or_insert(Rc::new(
                screeps::game::get_object_erased(id).expect("get_game_object not found"),
            ))
            .clone()
    }

    pub fn get_building_object(&mut self) -> Option<&ObjectBasicInfo> {
        let construction_sites = &screeps::game::construction_sites::values();
        if construction_sites.len() == 0 {
            None
        } else {
            None
            //            Some(self.get_object(&construction_sites[0].id()))
        }
    }
}

impl ObjectBasicInfo {
    pub fn pool_diff_range(&self, target: &ObjectBasicInfo) -> u32 {
        self.pos.range_to(&target.pos)
    }
}
