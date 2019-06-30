use screeps_ai::common::Position;
use std::collections::HashMap;

mod object_interface;

#[derive(Debug, Clone)]
pub enum ScreepsObjectType {
    Unknown,
    Spawn,
    Creep,
    PowerCreep,
    Source,
    Mineral,
    Controller,
    ConstructionSites,
    ConstructedWall,
    Extension,
    Link,
    Storage,
    Tower,
    Observer,
    PowerSpawn,
    PowerBank,
    Lab,
    Terminal,
    Nuker,
}

#[derive(Debug, Clone)]
pub struct ObjectBasicInfo {
    pub obj_type: ScreepsObjectType,
    pub name: String,
    pub id: String,
    pub pos: Position,
}

pub struct Manager {
    objects: HashMap<String, ObjectBasicInfo>,

    sources_lists: Vec<ObjectBasicInfo>,

    source_range: HashMap<String, Vec<ObjectBasicInfo>>,

    room_objects: HashMap<String, screeps::objects::RoomObject>,
}
