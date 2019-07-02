use screeps::{find, HasId, HasPosition, Structure, StructureType};
use screeps_ai::common::Position;
use screeps_ai::object_manager::{Manager, ObjectBasicInfo, ScreepsObjectType};
use screeps_ai::offer_manager;
use screeps_ai::offer_manager::ActionType;
use std::collections::HashMap;
use std::rc::Rc;

impl Manager {
    pub fn get_my_spawns() -> Vec<screeps::objects::StructureSpawn> {
        screeps::game::spawns::values()
    }

    pub fn new() -> Manager {
        Manager {
            objects: HashMap::with_capacity(100),
            source_range: Default::default(),
            room_objects: Default::default(),
            structures_lists: Default::default(),
        }
    }

    pub fn clean_invalid_object(&mut self) {
        self.room_objects.clear();
    }

    fn insert_object_info<T>(&mut self, obj: &T, obj_type: ScreepsObjectType)
    where
        T: screeps::objects::HasPosition + screeps::objects::HasId,
    {
        let id = obj.id();
        let pos = &obj.pos();

        let basic_info = Rc::new(ObjectBasicInfo {
            obj_type: obj_type.clone(),
            id: id.clone(),
            pos: Position {
                x: pos.x(),
                y: pos.y(),
            },
        });
        self.structures_lists[obj_type as usize].push(basic_info.clone());
        self.objects.insert(id, basic_info);
    }

    fn init_structures(&mut self) {
        let structures: &Vec<screeps::objects::Structure> = &screeps::game::structures::values();

        for structure in structures {
            match structure {
                Structure::Container(v) => self.insert_object_info(v, ScreepsObjectType::Container),
                Structure::Controller(v) => {
                    self.insert_object_info(v, ScreepsObjectType::Controller)
                }
                Structure::Extension(v) => self.insert_object_info(v, ScreepsObjectType::Extension),
                Structure::Extractor(v) => self.insert_object_info(v, ScreepsObjectType::Extractor),
                Structure::KeeperLair(v) => {
                    self.insert_object_info(v, ScreepsObjectType::KeeperLair)
                }
                Structure::Lab(v) => self.insert_object_info(v, ScreepsObjectType::Lab),
                Structure::Link(v) => self.insert_object_info(v, ScreepsObjectType::Link),
                Structure::Nuker(v) => self.insert_object_info(v, ScreepsObjectType::Nuker),
                Structure::Observer(v) => self.insert_object_info(v, ScreepsObjectType::Observer),
                Structure::PowerBank(v) => self.insert_object_info(v, ScreepsObjectType::PowerBank),
                Structure::PowerSpawn(v) => {
                    self.insert_object_info(v, ScreepsObjectType::PowerSpawn)
                }
                Structure::Portal(v) => self.insert_object_info(v, ScreepsObjectType::Portal),
                Structure::Rampart(v) => self.insert_object_info(v, ScreepsObjectType::Rampart),
                Structure::Road(v) => self.insert_object_info(v, ScreepsObjectType::Road),
                Structure::Spawn(v) => self.insert_object_info(v, ScreepsObjectType::Storage),
                Structure::Storage(v) => self.insert_object_info(v, ScreepsObjectType::Container),
                Structure::Terminal(v) => self.insert_object_info(v, ScreepsObjectType::Terminal),
                Structure::Tower(v) => self.insert_object_info(v, ScreepsObjectType::Tower),
                Structure::Wall(v) => self.insert_object_info(v, ScreepsObjectType::Wall),
            }
        }
    }

    fn init_objects_in_room(&mut self) {
        let rooms: &Vec<screeps::objects::Room> = &screeps::game::rooms::values();
        for room in rooms {
            //sources
            let sources: &Vec<screeps::objects::Source> = &room.find(find::SOURCES);
            for source in sources {
                let id = source.id();
                let pos = &source.pos();

                let basic_info = Rc::new(ObjectBasicInfo {
                    obj_type: ScreepsObjectType::Source,
                    id: id.clone(),
                    pos: Position {
                        x: pos.x(),
                        y: pos.y(),
                    },
                });
                self.objects.insert(id.clone(), basic_info.clone());
                self.structures_lists[ScreepsObjectType::Source as usize].push(basic_info);
            }
        }
    }

    fn init_objects_spawns(&mut self) {
        for spawn in &screeps::game::spawns::values() {
            let pos = &spawn.pos();

            let my_pos = Position {
                x: pos.x(),
                y: pos.y(),
            };

            let basic_info = Rc::new(ObjectBasicInfo {
                obj_type: ScreepsObjectType::Spawn,
                id: spawn.id(),
                pos: my_pos,
            });
            self.objects.insert(spawn.id(), basic_info.clone());
            self.structures_lists[ScreepsObjectType::Spawn as usize].push(basic_info);
        }
    }

    fn init_objects_constructions(&mut self) {
        self.structures_lists[ScreepsObjectType::ConstructionSites as usize].clear();
        for construction in &screeps::game::construction_sites::values() {
            let id = construction.id();
            if let Some(v) = self.objects.get(&id) {
                self.structures_lists[ScreepsObjectType::ConstructionSites as usize]
                    .push(v.clone());
                continue;
            }

            let pos = &construction.pos();
            let my_pos = Position {
                x: pos.x(),
                y: pos.y(),
            };

            let basic_info = Rc::new(ObjectBasicInfo {
                obj_type: ScreepsObjectType::ConstructionSites,
                id: id.clone(),
                pos: my_pos,
            });
            self.structures_lists[ScreepsObjectType::ConstructionSites as usize]
                .push(basic_info.clone());
            self.objects.insert(id, basic_info);
        }
    }

    pub fn init(&mut self) -> bool {
        self.init_structures();
        self.init_objects_in_room();
        self.init_objects_constructions();
        self.init_objects_spawns();

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

    pub fn get_empty_basic_info(&self) -> Rc<ObjectBasicInfo> {
        Rc::new(ObjectBasicInfo {
            obj_type: ScreepsObjectType::Invalid,
            id: "".to_string(),
            pos: Position { x: 0, y: 0 },
        })
    }
    //
    //    pub fn get_empty_basic_info(&self) -> Rc<ObjectBasicInfo> {
    //        self.structures_lists[ScreepsObjectType::Spawn as usize][0].clone()
    //    }

    pub fn get_object(&mut self, id: &str) -> Rc<ObjectBasicInfo> {
        if let Some(v) = self.objects.get(id) {
            return v.clone();
        }

        let obj = self
            .get_game_object(id)
            .expect("get_object error in net game object");

        let basic_info = Rc::new(ObjectBasicInfo {
            obj_type: ScreepsObjectType::Unknown,
            id: id.to_string(),
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
        let mut sl = self.structures_lists[ScreepsObjectType::Source as usize].clone();
        sl.sort_by_cached_key(|v| v.pos.range_to(my_pos));

        self.source_range.insert(object_id.to_string(), sl.clone());
        sl
    }

    pub fn get_game_object(self: &mut Self, id: &str) -> Option<Rc<screeps::objects::RoomObject>> {
        if let Some(v) = self.room_objects.get(id) {
            return Some(v.clone());
        }

        match screeps::game::get_object_erased(id) {
            Some(v) => {
                let obj = Rc::new(v);
                self.room_objects.insert(id.to_string(), obj.clone());
                Some(obj)
            }
            None => None,
        }
    }

    pub fn get_structures(&self, obj_type: ScreepsObjectType) -> Vec<Rc<ObjectBasicInfo>> {
        self.structures_lists[obj_type as usize].clone()
    }

    pub fn get_structures_ref(&self, obj_type: ScreepsObjectType) -> &Vec<Rc<ObjectBasicInfo>> {
        &self.structures_lists[obj_type as usize]
    }

    pub fn get_normal_transfer_object(&self) -> Option<Rc<ObjectBasicInfo>> {
        None
    }

    pub fn get_repair_object(&self) -> Option<Rc<ObjectBasicInfo>> {
        None
    }

    pub fn get_extension_transfer_object(&self) -> Option<Rc<ObjectBasicInfo>> {
        let mut need_transfer = false;
        let rooms: &Vec<screeps::objects::Room> = &screeps::game::rooms::values();
        for room in rooms {
            if room.energy_available() != room.energy_capacity_available() {
                need_transfer = true;
                break;
            }
        }
        if !need_transfer {
            return None;
        }
        for extension in &self.structures_lists[ScreepsObjectType::Extension as usize] {
            if offer_manager::Manager::is_invalid_action(
                &extension.id,
                &ActionType::Transfer(screeps::constants::ResourceType::Energy),
            ) {
                continue;
            } else {
                return Some(extension.clone());
            }
        }
        None
    }

    pub fn get_building_object(&mut self) -> Option<Rc<ObjectBasicInfo>> {
        if self.structures_lists[ScreepsObjectType::ConstructionSites as usize].is_empty() {
            if screeps::game::time() % 30 == 0 {
                self.init_objects_constructions();
            }
        }

        loop {
            if self.structures_lists[ScreepsObjectType::ConstructionSites as usize].is_empty() {
                return None;
            } else {
                let site =
                    self.structures_lists[ScreepsObjectType::ConstructionSites as usize][self
                        .structures_lists[ScreepsObjectType::ConstructionSites as usize]
                        .len()
                        - 1]
                    .clone();
                let site_obj = self.get_game_object(&site.id);
                if let None = site_obj {
                    //Todo Build over
                    self.structures_lists[ScreepsObjectType::ConstructionSites as usize].pop();
                    continue;
                }

                let site_obj = &site_obj.unwrap();

                if site_obj.build_over() {
                    self.structures_lists[ScreepsObjectType::ConstructionSites as usize].pop();
                    continue;
                }
                return Some(site);
            }
        }
    }
}

impl ObjectBasicInfo {
    pub fn pool_diff_range(&self, target: &ObjectBasicInfo) -> u32 {
        self.pos.range_to(&target.pos)
    }
}
