mod action;
mod data_control;
mod default_offer;
mod offer_interface;

use screeps_ai::object_manager::ObjectBasicInfo;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone)]
pub struct PointToPointWorkInfo {
    source: &'static ObjectBasicInfo,
    source_action: ActionType,
    target: &'static ObjectBasicInfo,
    target_action: ActionType,
}

#[derive(Debug, Clone)]
pub enum ActionType {
    Harvest,
    Transfer(screeps::constants::ResourceType),
    UpgradeController,
    Renew,
    Attack,
    AttackController,
    RangeAttack,
    RangedMassAttack,
    RangeHeal,
    Build,
    ClaimController,
    Dismantle,
    Drop,
    GenerateSafeMode,
    Heal,
    PickUp,
    Pull,
    Repair,
    ReserveController,
    Say,
    SignController,
    Suicide,
    WithDraw,
}

#[derive(Clone, Debug)]
pub enum WorkerState {
    StupidWorker,
    //    MoveToSource,
    DoSourceWork,
    //    MoveToTarget,
    DoTargetWork,
}

#[derive(Debug, Clone)]
pub enum WorkType {
    UnKnown,
    PointToPoint(PointToPointWorkInfo),
    CleanRoom,
}

#[derive(Clone, Debug)]
pub struct GroupEmployInfo {
    pausing: bool,
    workers: HashMap<String, WorkerState>,
    max_number: usize,
    offer_type: WorkType,
    flag: Option<String>,
}

pub struct Manager {
    offer_list: BTreeMap<i32, Vec<GroupEmployInfo>>,
    current_number: usize,
    max_number: usize,

    spawn_offers: HashMap<String, Vec<&'static mut GroupEmployInfo>>,
}
