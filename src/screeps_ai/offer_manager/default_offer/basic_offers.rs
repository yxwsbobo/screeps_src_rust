use screeps_ai::object_manager::{ScreepsObjectType, ObjectBasicInfo};
use screeps_ai::offer_manager::offer_interface::get_offer_mut;
use screeps_ai::offer_manager::{
    ActionType, GroupEmployInfo, Manager, PointToPointWorkInfo, WorkType,
};
use screeps_ai::{get_object_manager, get_offer_manager};
use std::rc::Rc;

pub const BASIC_SPAWN_OFFER_LEVEL: i32 = -1000;
pub const BASIC_EXTENSION_OFFER_LEVEL: i32 = -999;
pub const BASIC_UPGRADE_OFFER_LEVEL: i32 = 1000;
pub const BASIC_REPAIR_OFFER_LEVEL: i32 = 990;
pub const BASIC_NORMAL_TRANSFER_OFFER_LEVEL: i32 = 991;
pub const BASIC_BUILD_OFFER_LEVEL: i32 = 992;

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

    pub fn new_target_to_all_source_employ(
        target_id: &str,
        target_action: &ActionType,
        worker_number:usize)->Vec<Rc<GroupEmployInfo>>{

        let target = &get_object_manager().get_object(target_id);
        let sources_list = &get_object_manager().get_object_to_source(target_id);
        let mut employ_lists = Vec::with_capacity(sources_list.len());

        for source in sources_list {
            let mut employ_info = GroupEmployInfo::new();
            let p2p = PointToPointWorkInfo {
                source:source.clone(),
                source_action: ActionType::Harvest,
                target: target.clone(),
                target_action:target_action.clone(),
            };

            let range = target.pool_diff_range(source) as usize;

            let mut need_number = range * worker_number + 3;
            employ_info.max_number = need_number / 5;

            employ_info.offer_type = WorkType::PointToPoint(p2p);
            employ_lists.push(Rc::new(employ_info));
        }

        employ_lists
    }

    fn basic_spawn_offer(&mut self) {
        let offers = self
            .offer_list
            .entry(BASIC_SPAWN_OFFER_LEVEL)
            .or_default();

        for spawn in get_object_manager().get_structures(ScreepsObjectType::Spawn) {
            let spawn_offer = Manager::new_target_to_all_source_employ(
                &spawn.id,
                &ActionType::Transfer(screeps::constants::ResourceType::Energy),
                1
            );

            for offer in spawn_offer {
                offers.push(offer.clone());
            }
        }
    }

    fn basic_upgrade_offer(&mut self) {
        let offers = self
            .offer_list
            .entry(BASIC_UPGRADE_OFFER_LEVEL)
            .or_default();

        for controller in &get_object_manager().get_structures(ScreepsObjectType::Controller) {
            let controller_offers = Manager::new_target_to_all_source_employ(
                &controller.id,
                &ActionType::UpgradeController,
                1
            );

            for offer in controller_offers {
                offers.push(offer.clone());
            }
        }
    }

    pub fn get_upgrade_offer_number(&self)-> usize{
        self.offer_list.get(&BASIC_UPGRADE_OFFER_LEVEL).unwrap().len()
    }

    fn basic_extension_offer(&mut self) {
        let count = self.get_upgrade_offer_number();
        for index in 0 .. count {
            let employ_info = Manager::new_extension_employ(0);

            let offers = self
                .offer_list
                .entry(BASIC_EXTENSION_OFFER_LEVEL)
                .or_default();
            offers.push(employ_info);
        }

    }

    fn basic_build_offer(&mut self) {
        let count = self.get_upgrade_offer_number();
        for index in 0 .. count {
            let employ_info = Manager::new_build_employ(0);
            let offers = self.offer_list.entry(BASIC_BUILD_OFFER_LEVEL).or_default();
            offers.push(employ_info);
        }
    }

    fn basic_normal_transfer(&mut self) {
        let count = self.get_upgrade_offer_number();
        for index in 0 .. count {
            let employ_info = Manager::new_normal_transfer_employ(0);

            let offers = self
                .offer_list
                .entry(BASIC_NORMAL_TRANSFER_OFFER_LEVEL)
                .or_default();
            offers.push(employ_info.clone());
        }
    }

    fn basic_repair_offer(&mut self) {
        let count = self.get_upgrade_offer_number();
        for index in 0 .. count {
            let employ_info = Manager::new_repair_employ(0);

            let offers = self.offer_list.entry(BASIC_REPAIR_OFFER_LEVEL).or_default();
            offers.push(employ_info);
        }
    }

    pub fn get_basic_employ_index(&self, basic_type:i32,index: usize)->Rc<GroupEmployInfo>{
        self.offer_list.get(&basic_type).expect("impossible in get_basic_employ")[index].clone()
    }

    pub fn get_basic_employ(&self, basic_type:i32)->Rc<GroupEmployInfo>{
        self.offer_list.get(&basic_type).expect("impossible in get_basic_employ")[0].clone()
    }

    pub fn connect_employ_on_pausing(source:&Rc<GroupEmployInfo>, target:&Rc<GroupEmployInfo>){
        get_offer_mut(source).next_offer = Rc::downgrade(target);
    }

    fn basic_on_pausing(&mut self) {
        let count = self.get_upgrade_offer_number();
        for index in 0 .. count {
            let upgrade_offer = self.get_basic_employ_index(BASIC_UPGRADE_OFFER_LEVEL,index);
            let build_offer = self.get_basic_employ_index(BASIC_BUILD_OFFER_LEVEL,index);
            let spawn_offer = self.get_basic_employ_index(BASIC_SPAWN_OFFER_LEVEL,index);

            let extension_offer = self.get_basic_employ_index(BASIC_EXTENSION_OFFER_LEVEL,index);
            let repair_offer = self.get_basic_employ_index(BASIC_REPAIR_OFFER_LEVEL,index);
            let normal_transfer_offer = self.get_basic_employ_index(BASIC_NORMAL_TRANSFER_OFFER_LEVEL,index);

            Manager::connect_employ_on_pausing(&spawn_offer, &upgrade_offer);
            Manager::connect_employ_on_pausing(&extension_offer, &normal_transfer_offer);
            Manager::connect_employ_on_pausing(&normal_transfer_offer, &repair_offer);
            Manager::connect_employ_on_pausing(&repair_offer, &build_offer);
            Manager::connect_employ_on_pausing(&build_offer, &upgrade_offer);
        }
    }

    pub fn init_basic_offer(&mut self) {
        //basic
        self.basic_spawn_offer();

        self.basic_upgrade_offer();

        self.basic_extension_offer();

        //pausing do
        self.basic_repair_offer();
        self.basic_normal_transfer();

        self.basic_build_offer();

        self.basic_on_pausing();
        info!("8");

    }
}
