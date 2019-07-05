use screeps::{find, HasId, HasPosition, Structure, StructureType};
use screeps_ai::common::Position;
use screeps_ai::object_manager::{Manager, ObjectBasicInfo, ScreepsObjectType};
use screeps_ai::{offer_manager, get_object_manager};
use screeps_ai::offer_manager::ActionType;
use std::collections::HashMap;
use std::rc::Rc;
use std::any::Any;

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
            new_structures_flag: 0,
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
        if self.objects.contains_key(&id){
            return;
        }

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
        info!("in init_structures");
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

        if !self.structures_lists[ScreepsObjectType::Source as usize].is_empty() {
            self.sort_extension_list();
        }
    }

    fn sort_extension_list(&mut self){
        self.structures_lists[ScreepsObjectType::Extension as usize].sort_by_cached_key(|v|{
            let source = get_object_manager().get_object_to_source(&v.id)[0].clone();
            20000 - v.pool_diff_range(&source)
        });
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

    fn init_construction_sites(&mut self) {
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
        self.init_construction_sites();
        self.init_empty_extension();
        self.init_objects_spawns();
        self.sort_extension_list();

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
        for tower in &self.structures_lists[ScreepsObjectType::Tower as usize] {
            if offer_manager::Manager::is_invalid_action(
                &tower.id,
                &ActionType::Transfer(screeps::constants::ResourceType::Energy),
            ) {
                continue;
            } else {
                return Some(tower.clone());
            }
        }

        for container in &self.structures_lists[ScreepsObjectType::Container as usize] {
            if offer_manager::Manager::is_invalid_action(
                &container.id,
                &ActionType::Transfer(screeps::constants::ResourceType::Energy),
            ) {
                continue;
            } else {
                return Some(container.clone());
            }
        }
        None
    }

    pub fn get_repair_object(&mut self) -> Option<Rc<ObjectBasicInfo>> {
        for container in self.structures_lists[ScreepsObjectType::Container as usize].clone() {
            let obj = self.get_game_object(&container.id).expect("not find object in repair");
            if obj.get_life_rate() < 0.1 {
                return Some(container.clone());
                continue;
            }
        }

        for road in self.structures_lists[ScreepsObjectType::Road as usize].clone() {
            let obj = self.get_game_object(&road.id).expect("not find object in repair");
            if obj.get_life_rate() < 0.1 {
                return Some(road.clone());
                continue;
            }
        }

        None
    }

    fn init_empty_extension_imp(&mut self){
        let mut temp_extensions:Vec<Rc<ObjectBasicInfo>> = Vec::new();
        for extension in &self.structures_lists[ScreepsObjectType::Extension as usize] {
            if offer_manager::Manager::is_invalid_action(
                &extension.id,
                &ActionType::Transfer(screeps::constants::ResourceType::Energy),
            ) {
                continue;
            } else {
                temp_extensions.push(extension.clone());
            }
        }
        self.structures_lists[ScreepsObjectType::EmptyExtensions as usize] = temp_extensions;
    }

    fn init_empty_extension(&mut self) -> bool{
        if self.structures_lists[ScreepsObjectType::EmptyExtensions as usize].is_empty() {
            if screeps::game::time() % 7 == 0 {
                self.init_empty_extension_imp();
            }
        }

        self.structures_lists[ScreepsObjectType::EmptyExtensions as usize].is_empty()
    }

    pub fn get_extension_transfer_object(&mut self) -> Option<Rc<ObjectBasicInfo>> {
        if self.init_empty_extension() {
//            info!("not find extension can transfer1");
            return None;
        }

        loop{
            if self.structures_lists[ScreepsObjectType::EmptyExtensions as usize].is_empty() {
//                info!("not find extension can transfer2");
                break;
            }

            let extension = self.structures_lists[ScreepsObjectType::EmptyExtensions as usize]
            [self.structures_lists[ScreepsObjectType::EmptyExtensions as usize].len()- 1].clone();

            if offer_manager::Manager::is_invalid_action(
                &extension.id,
                &ActionType::Transfer(screeps::constants::ResourceType::Energy),
            ) {
                self.structures_lists[ScreepsObjectType::EmptyExtensions as usize].pop();
                continue;
            } else {
                return Some(extension.clone());
            }
        }

        None
    }

    fn init_building_object(&mut self)-> bool{
        if self.structures_lists[ScreepsObjectType::ConstructionSites as usize].is_empty() {
            if screeps::game::time() % 30 == 0 {
                self.init_construction_sites();
            }
        }
        self.structures_lists[ScreepsObjectType::ConstructionSites as usize].is_empty()
    }

    pub fn get_building_object(&mut self) -> Option<Rc<ObjectBasicInfo>> {
        if self.init_building_object(){
            return None;
        }

        let mut updated_structures = false;
        loop {
            if self.structures_lists[ScreepsObjectType::ConstructionSites as usize].is_empty() {
                if !updated_structures{
                    self.init_structures();
                    self.new_structures_flag = 0;
                }
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
                    self.new_structures_flag +=1;
                    //Todo Build over
                    if self.new_structures_flag % 5 == 0{
                        if !updated_structures{
                            updated_structures = true;
                            self.init_structures();
                            self.new_structures_flag = 0;
                        }
                    }
                    info!("pop sturct: {}", self.new_structures_flag );
                    //Todo maybe need pop objects list
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
