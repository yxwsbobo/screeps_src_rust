mod basic_offers;
use screeps::{HasId, OwnedStructureProperties};
use screeps_ai::object_manager::{ObjectBasicInfo, ScreepsObjectType};
use screeps_ai::offer_manager::offer_interface::get_offer_mut;
use screeps_ai::offer_manager::{
    ActionType, GroupEmployInfo, Manager, PointToPointWorkInfo, WorkType,
};
use screeps_ai::{get_object_manager, get_offer_manager, object_manager};
use std::rc::Rc;

impl Manager {
    //    fn add_range_target_offer(
    //        &mut self,
    //        level:i32,
    //        source_action:ActionType,
    //        target_action:ActionType,
    //        offer_type:WorkType,
    //        max_num:usize){
    //
    //        let temp_target = get_object_manager().get_empty_basic_info();
    //        let mut employ_info = GroupEmployInfo::new();
    //        let p2p = PointToPointWorkInfo {
    //            source: temp_target.clone(),
    //            source_action,
    //            target: temp_target,
    //            target_action,
    //        };
    //
    //        employ_info.offer_type = offer_type;
    //        employ_info.max_number = max_num;
    //
    //        let offers = self.offer_list.entry(level).or_default();
    //        offers.push(employ_info);
    //
    //    }

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

    fn add_offer_info(
        &mut self,
        level: i32,
        target_id: String,
        target_action: ActionType,
        number1: usize,
        number2: usize,
    ) {
        let offers = self.offer_list.entry(level).or_default();
        let obj_manager = get_object_manager();
        let mut first = true;

        for source in obj_manager.get_object_to_source(&target_id) {
            if number1 == 0 {
                if first {
                    first = false;
                    continue;
                }
            }
            if number2 == 0 {
                if !first {
                    continue;
                }
            }

            let mut employ_info = GroupEmployInfo::new();
            let p2p = PointToPointWorkInfo {
                source,
                source_action: ActionType::Harvest,
                target: obj_manager.get_object(&target_id),
                target_action: target_action.clone(),
            };

            employ_info.offer_type = WorkType::PointToPoint(p2p);

            employ_info.max_number = if first {
                first = false;
                number1
            } else {
                number2
            };
            offers.push(Rc::new(employ_info));
        }
    }

    fn init_spawn_offer(&mut self) {
        for spawn in get_object_manager().get_structures(ScreepsObjectType::Spawn) {
            self.add_offer_info(
                1,
                spawn.id.clone(),
                ActionType::Transfer(screeps::constants::ResourceType::Energy),
                1,
                0,
            );
        }
    }

    fn init_controller_offer(&mut self) {
        let level2 = 15;
        for controller in get_object_manager().get_structures(ScreepsObjectType::Controller) {
            self.add_offer_info(
                5,
                controller.id.clone(),
                ActionType::UpgradeController,
                1,
                0,
            );
            self.add_offer_info(
                level2,
                controller.id.clone(),
                ActionType::UpgradeController,
                3,
                7,
            );
        }
    }

    fn init_build_offer(&mut self) {
        let temp_target = get_object_manager().get_empty_basic_info();
        let mut employ_info = GroupEmployInfo::new();
        let p2p = PointToPointWorkInfo {
            source: temp_target.clone(),
            source_action: ActionType::Harvest,
            target: temp_target,
            target_action: ActionType::Build,
        };

        employ_info.offer_type = WorkType::BuildAll(p2p);
        employ_info.max_number = 3;

        let offers = self.offer_list.entry(13).or_default();
        offers.push(Rc::new(employ_info));
    }

    fn init_extensions_offer(&mut self) {
        let temp_target = get_object_manager().get_empty_basic_info();
        let mut employ_info = GroupEmployInfo::new();
        let p2p = PointToPointWorkInfo {
            source: temp_target.clone(),
            source_action: ActionType::Harvest,
            target: temp_target,
            target_action: ActionType::Transfer(screeps::constants::ResourceType::Energy),
        };

        employ_info.offer_type = WorkType::ExtensionTransfer(p2p);
        employ_info.max_number = 3;

        let offers = self.offer_list.entry(11).or_default();
        offers.push(Rc::new(employ_info));
    }

    fn init_workers_number(&mut self) {
        self.max_number = 0;
        for offers in self.offer_list.values_mut() {
            for offer in offers {
                self.max_number += offer.max_number;
            }
        }
    }

    fn init_pausing_do(&mut self) {
        //        let level2 = 15;
        //        let level1 = 1;
        //        let level_builder = 13;
        //        let level_extension = 11;
        //        let controller_offer = get_offer_manager().offer_list.get(&level2).expect("fix me");
        //
        //        let self_controller = self.offer_list.get_mut(&level2).expect("fix me3");
        //
        //        let spawn_offer = self.offer_list.get_mut(&level1).expect("fix me2");
        //        Rc::get_mut(&mut spawn_offer[0]).expect("spawn_offer 1").next_offer = Some(&controller_offer[0]);
        //
        //        self_controller[0].next_offer = Some(&controller_offer[1]);
        //
        //        let self_builder = self.offer_list.get_mut(&level_builder).expect("fix me4");
        //        self_builder[0].next_offer = Some(&controller_offer[1]);
        //
        //        let self_extension = self.offer_list.get_mut(&level_extension).expect("fix me5");
        //        self_extension[0].next_offer = Some(&controller_offer[0]);
    }

    pub fn init_default_offers(&mut self) {
        self.init_basic_offer();

        //        self.init_spawn_offer();
        //        self.init_extensions_offer();
        //
        //        self.init_controller_offer();
        //
        //        self.init_build_offer();
        //
        //        self.init_pausing_do();

        self.init_workers_number();
    }

    fn check_and_set_offer_pausing(
        p2p: &mut PointToPointWorkInfo,
        target: &Option<Rc<ObjectBasicInfo>>,
    ) -> bool {
        if Manager::is_invalid_action(&p2p.target.id, &p2p.target_action) {
            match target {
                None => {
                    true;
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

    pub fn set_offer_state(&mut self) {
        for offers in self.offer_list.values_mut() {
            for offer in offers {
                let offer = get_offer_mut(offer);
                offer.pausing = Manager::check_work_need_pausing(&mut offer.offer_type);
            }
        }
    }
}
