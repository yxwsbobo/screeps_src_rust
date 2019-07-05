mod basic_offers;
mod level0_offer;
mod level1_offer;
mod level2_offer;
use screeps::{HasId, OwnedStructureProperties};
use screeps_ai::object_manager::{ObjectBasicInfo, ScreepsObjectType};
use screeps_ai::offer_manager::offer_interface::get_offer_mut;
use screeps_ai::offer_manager::{
    ActionType, GroupEmployInfo, Manager, PointToPointWorkInfo, WorkType,
};
use screeps_ai::{get_object_manager, get_offer_manager, object_manager, middle_time};
use std::rc::Rc;
use screeps_ai::offer_manager::default_offer::basic_offers::{BASIC_UPGRADE_OFFER_LEVEL, BASIC_BUILD_OFFER_LEVEL, BASIC_NORMAL_TRANSFER_OFFER_LEVEL};

impl Manager {
    fn add_target_offer_use_source(
        &mut self,
        level: i32,
        target_id: String,
        target_action: ActionType,
        max_number: usize,
    ) {
        let obj_manager = get_object_manager();

        let source = obj_manager.get_object_to_source(&target_id)[0].clone();

        let mut employ_info = GroupEmployInfo::new();
        let p2p = PointToPointWorkInfo {
            source,
            source_action: ActionType::Harvest,
            target: obj_manager.get_object(&target_id),
            target_action,
        };

        employ_info.offer_type = WorkType::PointToPoint(p2p);

        employ_info.max_number = max_number;

        let offers = self.offer_list.entry(level).or_default();
        offers.push(Rc::new(employ_info));
    }
//
//    fn init_spawn_offer(&mut self) {
//        for spawn in get_object_manager().get_structures(ScreepsObjectType::Spawn) {
//            self.add_target_offer_use_source(
//                11,
//                spawn.id.clone(),
//                ActionType::Transfer(screeps::constants::ResourceType::Energy),
//                6,
//            );
//        }
//    }
//
//    fn init_extensions_offer(&mut self) {
//        let employ_info = Manager::new_extension_employ(6);
//
//        let offers = self.offer_list.entry(1).or_default();
//        offers.push(employ_info);
//    }
//
//    fn init_transfer_offer(&mut self){
//        let employ_info = Manager::new_normal_transfer_employ(3);
//
//        let offers = self.offer_list.entry(20).or_default();
//        offers.push(employ_info);
//    }

    fn init_workers_number(&mut self) {
        self.max_number = 0;
        for offers in self.offer_list.values_mut() {
            for offer in offers {
                self.max_number += offer.max_number;
            }
        }
    }

//    fn init_pausing_do(&mut self) {
//        let spawn_level = 11;
//        let extensions_level = 1;
//        let transfer_level = 20;
//        for offer in self.offer_list.get(&spawn_level).expect("impossible in temp pausing") {
//            Manager::connect_employ_on_pausing(offer,&self.get_basic_employ(BASIC_UPGRADE_OFFER_LEVEL));
//        }
//
//        for offer in self.offer_list.get(&extensions_level).expect("impossible in temp pausing2") {
//            Manager::connect_employ_on_pausing(offer,&self.get_basic_employ(BASIC_NORMAL_TRANSFER_OFFER_LEVEL));
//        }
//
////        for offer in self.offer_list.get(&transfer_level).expect("impossible in temp pausing3") {
////            Manager::connect_employ_on_pausing(offer,&self.get_basic_employ(BASIC_BUILD_OFFER_LEVEL));
////        }
//    }

    pub fn init_default_offers(&mut self) {
        self.offer_list.clear();
        self.init_basic_offer();

        match self.offer_level {
            0 =>{
                info!("use offer 0");
                self.init_offer_level0();
            }
            1 =>{
                info!("use offer 1");
                self.init_offer_level1();
            }
            2 =>{
                info!("use offer 2");
                self.init_offer_level2();
            }

            _ =>{
                warn!("unknown offer");
            }
        }
        self.init_workers_number();
    }

    fn check_and_set_offer_pausing(
        p2p: &mut PointToPointWorkInfo,
        target: &Option<Rc<ObjectBasicInfo>>,
    ) -> bool {
        if Manager::is_invalid_action(&p2p.target.id, &p2p.target_action) {
            match target {
                None => {
                    return true;
                }
                Some(v_target) => {
                    p2p.target = v_target.clone();
                    p2p.source = get_object_manager().get_object_to_source(&v_target.id)[0].clone();
                }
            }
        }

        Manager::is_invalid_action(&p2p.source.id, &p2p.source_action)
    }

    fn check_work_need_pausing(work: &mut WorkType) -> bool {
        match work {
            WorkType::UnKnown => true,
            WorkType::PointToPoint(v) => {
                Manager::is_invalid_action(&v.source.id, &v.source_action)
                    || Manager::is_invalid_action(&v.target.id, &v.target_action)
            }
            WorkType::BuildAll(v) => {
                Manager::check_and_set_offer_pausing(v, &get_object_manager().get_building_object())
            }
            WorkType::CleanRoom => true,
            WorkType::ExtensionTransfer(v) => Manager::check_and_set_offer_pausing(
                v,
                &get_object_manager().get_extension_transfer_object(),
            ),
            WorkType::NormalTransfer(v) => Manager::check_and_set_offer_pausing(
                v,
                &get_object_manager().get_normal_transfer_object(),
            ),
            WorkType::RepairAll(v) => {
                Manager::check_and_set_offer_pausing(v, &get_object_manager().get_repair_object())
            }
        }
    }

    fn check_and_set_fight(&mut self){

    }

    pub fn set_offer_state(&mut self) {
        if middle_time() {
            self.check_and_set_fight();
        }
        for offers in self.offer_list.values_mut() {
            for offer in offers {
                let offer = get_offer_mut(offer);
                offer.pausing = Manager::check_work_need_pausing(&mut offer.offer_type);
            }
        }
    }
}
