use screeps::{HasId, OwnedStructureProperties};
use screeps_ai::offer_manager::{
    ActionType, GroupEmployInfo, Manager, PointToPointWorkInfo, WorkType,
};
use screeps_ai::{get_object_manager, get_offer_manager, object_manager};

impl Manager {
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
            offers.push(employ_info);
        }
    }

    fn init_spawn_offer(&mut self) {
        for spawn in &object_manager::Manager::get_my_spawns() {
            self.add_offer_info(
                10,
                spawn.id(),
                ActionType::Transfer(screeps::constants::ResourceType::Energy),
                3,
                7,
            );
        }
    }

    fn init_controller_offer(&mut self) {
        let level2 = 15;
        for room in &screeps::game::rooms::values() {
            let controller: &screeps::objects::StructureController = &room.controller().unwrap();
            if !controller.my() {
                continue;
            }
            self.add_offer_info(5, controller.id(), ActionType::UpgradeController, 1, 0);
            self.add_offer_info(
                level2,
                controller.id(),
                ActionType::UpgradeController,
                6,
                13,
            );
        }
    }

    fn init_build_offer(&mut self) {

        let temp_target = get_object_manager().get_first_source();
        let mut employ_info = GroupEmployInfo::new();
        let p2p = PointToPointWorkInfo {
            source:temp_target.clone(),
            source_action: ActionType::Harvest,
            target: temp_target,
            target_action: ActionType::Build,
        };

        employ_info.offer_type = WorkType::BuildAll(p2p);
        employ_info.max_number = 3;

        let offers = self.offer_list.entry(13).or_default();
        offers.push(employ_info);
//        let level = 13;
//        let construction_sites = &screeps::game::construction_sites::values();
//        for construction_site in construction_sites {
//            if !construction_site.my() {
//                continue;
//            }
//
//            self.add_offer_info(level, construction_site.id(), ActionType::Build, 0, 0);
//        }
//
//        let build_offer = self.offer_list.get_mut(&level).expect("fix me3");
//        build_offer[0].max_number = 4;
//
//        let g_builder = get_offer_manager()
//            .offer_list
//            .get_mut(&level)
//            .expect("fix me 4");
//
//        for index in 1..g_builder.len() {
//            build_offer[index - 1].next_offer = Some(&g_builder[index]);
//        }
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
        let level2 = 15;
        let level1 = 10;
        let level_builder = 13;
        let controller_offer = get_offer_manager().offer_list.get(&level2).expect("fix me");

        let spawn_offer = self.offer_list.get_mut(&level1).expect("fix me2");
        spawn_offer[0].next_offer = Some(&controller_offer[0]);
        spawn_offer[1].next_offer = Some(&controller_offer[1]);

        let self_controller = self.offer_list.get_mut(&level2).expect("fix me");
        self_controller[0].next_offer = Some(&controller_offer[1]);

        let self_builder = self.offer_list.get_mut(&level_builder).expect("fix me");
        self_builder[0].next_offer = Some(&controller_offer[1]);
    }

    pub fn init_default_offers(&mut self) {
        self.init_spawn_offer();

        self.init_controller_offer();

        self.init_build_offer();

        self.init_pausing_do();

        self.init_workers_number();
    }

    fn check_work_need_pausing(work: &mut WorkType) -> bool {
        match work {
            WorkType::UnKnown => true,
            WorkType::PointToPoint(v) => {
                Manager::is_invalid_action(&v.source, &v.source_action)
                    || Manager::is_invalid_action(&v.target, &v.target_action)
            }
            WorkType::BuildAll(v) => match get_object_manager().get_building_object() {
                None => true,
                Some(target) => {
                    if v.target.id != target.id {
                        v.target = target;
                        v.source = get_object_manager().get_object_to_source(&v.target.id)[0].clone();
                    }

                    Manager::is_invalid_action(&v.source, &v.source_action)
                        || Manager::is_invalid_action(&v.target, &v.target_action)
                }
            },
            WorkType::CleanRoom => true,
        }
    }

    pub fn set_offer_state(&mut self) {
        for offers in self.offer_list.values_mut() {
            for offer in offers {
                offer.pausing = Manager::check_work_need_pausing(&mut offer.offer_type);
            }
        }
    }
}
