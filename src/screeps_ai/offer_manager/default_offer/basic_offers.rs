use screeps_ai::object_manager::ScreepsObjectType;
use screeps_ai::offer_manager::offer_interface::get_offer_mut;
use screeps_ai::offer_manager::{
    ActionType, GroupEmployInfo, Manager, PointToPointWorkInfo, WorkType,
};
use screeps_ai::{get_object_manager, get_offer_manager};
use std::rc::Rc;

pub const BASIC_SPAWN_OFFER_LEVEL: i32 = -1000;
pub const BASIC_EXTENSION_OFFER_LEVEL: i32 = -999;
pub const BASIC_UPGRADE_OFFER_LEVEL: i32 = 1000;
pub const BASIC_REPAIR_OFFER_LEVEL: i32 = -500;
pub const BASIC_NORMAL_TRANSFER_OFFER_LEVEL: i32 = -499;
pub const BASIC_BUILD_OFFER_LEVEL: i32 = -498;

impl Manager {
    pub fn new_extension_employ(max_num:usize) ->Rc<GroupEmployInfo>{
        let temp_target = get_object_manager().get_empty_basic_info();
        let mut employ_info = GroupEmployInfo::new();
        let p2p = PointToPointWorkInfo {
            source: temp_target.clone(),
            source_action: ActionType::Harvest,
            target: temp_target,
            target_action: ActionType::Transfer(screeps::constants::ResourceType::Energy),
        };

        employ_info.offer_type = WorkType::ExtensionTransfer(p2p);
        employ_info.max_number = max_num;
        Rc::new(employ_info)
    }

    pub fn new_build_employ(max_num:usize) ->Rc<GroupEmployInfo>{
        let temp_target = get_object_manager().get_empty_basic_info();
        let mut employ_info = GroupEmployInfo::new();
        let p2p = PointToPointWorkInfo {
            source: temp_target.clone(),
            source_action: ActionType::Harvest,
            target: temp_target,
            target_action: ActionType::Build,
        };

        employ_info.offer_type = WorkType::BuildAll(p2p);
        employ_info.max_number = max_num;
        Rc::new(employ_info)
    }

    pub fn new_normal_transfer_employ(max_num:usize) ->Rc<GroupEmployInfo>{
        let temp_target = get_object_manager().get_empty_basic_info();
        let mut employ_info = GroupEmployInfo::new();
        let p2p = PointToPointWorkInfo {
            source: temp_target.clone(),
            source_action: ActionType::Harvest,
            target: temp_target,
            target_action: ActionType::Transfer(screeps::constants::ResourceType::Energy),
        };

        employ_info.offer_type = WorkType::NormalTransfer(p2p);
        employ_info.max_number = max_num;
        Rc::new(employ_info)
    }


    pub fn new_repair_employ(max_num:usize) ->Rc<GroupEmployInfo>{
        let temp_target = get_object_manager().get_empty_basic_info();
        let mut employ_info = GroupEmployInfo::new();
        let p2p = PointToPointWorkInfo {
            source: temp_target.clone(),
            source_action: ActionType::Harvest,
            target: temp_target,
            target_action: ActionType::Repair,
        };

        employ_info.offer_type = WorkType::RepairAll(p2p);
        employ_info.max_number = max_num;
        Rc::new(employ_info)
    }

    fn basic_spawn_offer(&mut self) {
        for spawn in get_object_manager().get_structures(ScreepsObjectType::Spawn) {
            self.add_target_offer_use_source(
                BASIC_SPAWN_OFFER_LEVEL,
                spawn.id.clone(),
                ActionType::Transfer(screeps::constants::ResourceType::Energy),
                1,
            );
        }
    }

    fn basic_upgrade_offer(&mut self) {
        for controller in get_object_manager().get_structures(ScreepsObjectType::Controller) {
            self.add_target_offer_use_source(
                BASIC_UPGRADE_OFFER_LEVEL,
                controller.id.clone(),
                ActionType::UpgradeController,
                0,
            );
        }
    }

    fn basic_extension_offer(&mut self) {
        let employ_info = Manager::new_extension_employ(1);

        let offers = self
            .offer_list
            .entry(BASIC_EXTENSION_OFFER_LEVEL)
            .or_default();
        offers.push(employ_info);
    }

    fn basic_build_offer(&mut self) {
        let employ_info = Manager::new_build_employ(0);
        let offers = self.offer_list.entry(BASIC_BUILD_OFFER_LEVEL).or_default();
        offers.push(employ_info);
    }

    fn basic_normal_transfer(&mut self) {
        let employ_info = Manager::new_normal_transfer_employ(1);

        let offers = self
            .offer_list
            .entry(BASIC_NORMAL_TRANSFER_OFFER_LEVEL)
            .or_default();
        offers.push(employ_info);
    }

    fn basic_repair_offer(&mut self) {
        let employ_info = Manager::new_repair_employ(1);

        let offers = self.offer_list.entry(BASIC_REPAIR_OFFER_LEVEL).or_default();
        offers.push(employ_info);
    }


    pub fn get_basic_employ(&self, basic_type:i32)->Rc<GroupEmployInfo>{
        self.offer_list.get(&basic_type).expect("impossible in get_basic_employ")[0].clone()
    }

    pub fn connect_employ_on_pausing(source:&Rc<GroupEmployInfo>, target:&Rc<GroupEmployInfo>){
        get_offer_mut(source).next_offer = Rc::downgrade(target);
    }

    fn basic_on_pausing(&mut self) {
        let upgrade_offer= self.get_basic_employ(BASIC_UPGRADE_OFFER_LEVEL);
        let build_offer= self.get_basic_employ(BASIC_BUILD_OFFER_LEVEL);
        let spawn_offer= self.get_basic_employ(BASIC_SPAWN_OFFER_LEVEL);

        let extension_offer= self.get_basic_employ(BASIC_EXTENSION_OFFER_LEVEL);
        let repair_offer = self.get_basic_employ(BASIC_REPAIR_OFFER_LEVEL);
        let normal_transfer_offer = self.get_basic_employ(BASIC_NORMAL_TRANSFER_OFFER_LEVEL);

        Manager::connect_employ_on_pausing(&spawn_offer, &upgrade_offer);
        Manager::connect_employ_on_pausing(&extension_offer, &normal_transfer_offer);
        Manager::connect_employ_on_pausing(&normal_transfer_offer, &repair_offer);
        Manager::connect_employ_on_pausing(&repair_offer, &build_offer);
        Manager::connect_employ_on_pausing(&build_offer,&upgrade_offer);
    }

    pub fn init_basic_offer(&mut self) {
        //basic
        self.basic_spawn_offer();
        self.basic_extension_offer();

        //pausing do
        self.basic_repair_offer();
        self.basic_normal_transfer();

        self.basic_build_offer();

        self.basic_upgrade_offer();

        self.basic_on_pausing();
    }
}
