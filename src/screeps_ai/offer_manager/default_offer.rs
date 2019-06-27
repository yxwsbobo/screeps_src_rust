use screeps_ai::offer_manager::{Manager, GroupEmployInfo, WorkType, PointToPointWorkInfo};
use std::collections::HashMap;
use screeps_ai::{get_creep_manager, object_manager};
use screeps::{HasId, OwnedStructureProperties};
use core::borrow::BorrowMut;

impl Manager {

    fn add_offer_info(&mut self, offers:&mut HashMap<String,GroupEmployInfo>, target_id:String,
                      number1:(usize,usize,usize), number2:(usize,usize,usize)){

        let mut employ_info = GroupEmployInfo::new();
        let source_id = get_creep_manager().get_closest_source(&target_id);

        employ_info.offer_type = WorkType::PointToPoint(PointToPointWorkInfo {
            source: source_id.clone(),
            target: target_id.clone(),
        });

        employ_info.at_least_number = number1.0;
        employ_info.normal_number = number1.1;
        employ_info.max_number = number1.2;

        offers.insert(String::from("")+ &source_id + "_" + &target_id, employ_info.clone());

        for (id, source) in get_creep_manager().get_sources() {
            if source_id == *id{
                continue;
            }

            employ_info.offer_type = WorkType::PointToPoint(PointToPointWorkInfo {
                source: id.clone(),
                target: target_id.clone(),
            });

            employ_info.at_least_number = number2.0;
            employ_info.normal_number = number2.1;
            employ_info.max_number = number2.2;

            offers.insert(String::from("")+ id + "_" + &target_id, employ_info.clone());
        }
    }

    fn init_spawn_offer(&mut self) {
        let mut offers:HashMap<String,GroupEmployInfo> = HashMap::new();

        for spawn in &object_manager::Manager::get_my_spawns() {
            self.add_offer_info(offers.borrow_mut(), spawn.id(), (1, 2, 3), (0, 6, 9));
        }

        self.offer_list.insert(1, offers);
    }

    fn init_controller_offer(&mut self) {
        let mut offers:HashMap<String,GroupEmployInfo> = HashMap::new();
        for room in &screeps::game::rooms::values() {
            let controller:&screeps::objects::StructureController = &room.controller().unwrap();
            if !controller.my(){
                continue;
            }
            self.add_offer_info(offers.borrow_mut(), controller.id(), (1, 4, 6), (0, 8, 10));
        }
        self.offer_list.insert(2, offers);
    }

    pub fn init_default_offers(&mut self){
        self.init_spawn_offer();

        self.init_controller_offer();

        info!("offer_list: {:#?}", self.offer_list);
    }
}