use screeps_ai::common::Position;
use std::collections::HashMap;
use std::rc::Rc;

mod object_interface;

#[derive(Debug, Clone)]
pub enum ScreepsObjectType {
    Invalid,
    Unknown,
    Spawn,
    Creep,
    PowerCreep,
    Source,
    Mineral,
    Extractor,
    Controller,
    Container,
    ConstructionSites,
    ConstructedWall,
    Rampart,
    KeeperLair,
    Wall,
    Extension,
    Road,
    Link,
    Storage,
    Tower,
    Observer,
    PowerSpawn,
    PowerBank,
    Lab,
    Terminal,
    Portal,
    Nuker,


    EmptyExtensions,
    LastFlag,
}

#[derive(Debug, Clone)]
pub struct ObjectBasicInfo {
    pub obj_type: ScreepsObjectType,
    pub id: String,
    pub pos: Position,
}

pub struct Manager {
    objects: HashMap<String, Rc<ObjectBasicInfo>>,
    room_objects: HashMap<String, Rc<screeps::objects::RoomObject>>,

    pub structures_lists: [Vec<Rc<ObjectBasicInfo>>; ScreepsObjectType::LastFlag as usize],

    source_range: HashMap<String, Vec<Rc<ObjectBasicInfo>>>,

    new_structures_flag:i32,
}
