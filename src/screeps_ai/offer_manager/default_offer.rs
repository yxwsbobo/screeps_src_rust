use screeps_ai::offer_manager::{Manager, GroupEmployInfo, WorkType};
use std::collections::HashMap;

impl Manager {

    fn init_spawn_office(&mut self) {
//        let office_level = 1;
//        self.office_list.insert(office_level, HashMap::new());
//        let offices = self.office_list.get_mut(&office_level).unwrap();
//        let mut employ_info = GroupEmployInfo::new();
//
//        for spawn in &Manager::get_my_spawns() {
//
//            let mut range = std::u32::MAX;
//            let mut find_id = &String::new();
//
//            for (id, source) in &self.sources_info {
//                let diff = self_info.pool_diff_range(&source.basic_info);
//                if range > diff {
//                    range = diff;
//                    find_id = id;
//                }
//            }
//
//            employ_info.offer_type = WorkType::PointToPoint(PointToPointWorkInfo {
//                source: self.sources_info[find_id].basic_info.clone(),
//                target: self_info.clone(),
//            });
//
//
//            employ_info.at_least_number = 0;
//            employ_info.max_number = 0;
//            employ_info.normal_number = 0;
//
//            offices.insert(String::from("spawn")+ find_id, employ_info.clone());
//
//            for (id, source) in &self.sources_info {
//                if find_id == id{
//                    continue;
//                }
//
//                employ_info.employ_type = super::ObjectEmployType::PointToPoint(PointToPointWorkInfo {
//                    source: source.basic_info.clone(),
//                    target: self_info.clone(),
//                });
//
//                employ_info.at_least_number = 2;
//                employ_info.max_number = 10;
//                employ_info.normal_number = 8;
//
//                offices.insert(String::from("spawn")+ id, employ_info.clone());
//            }
//        }
    }

    fn init_controller_office(&mut self) {
//        let office_level = 2;
//        self.office_list.insert(office_level, HashMap::new());
//        let offices = self.office_list.get_mut(&office_level).unwrap();
//        let mut employ_info = super::ObjectEmployInfo::new();
//        let rooms: &Vec<screeps::objects::Room> = &screeps::game::rooms::values();
//        for room in rooms {
//            let controller:&screeps::objects::StructureController = &room.controller().unwrap();
//            if controller.my(){
//                let self_info = super::ObjectBasicInfo {
//                    obj_type: super::ScreepsObjectType::Controller,
//                    name: controller.id(),
//                    id: controller.id(),
//                    pos: super::Position {
//                        x: controller.pos().x(),
//                        y: controller.pos().y(),
//                    },
//                };
//
//                let mut range = std::u32::MAX;
//                let mut find_id = &String::new();
//
//                for (id, source) in &self.sources_info {
//                    let diff = self_info.pool_diff_range(&source.basic_info);
//                    if range > diff {
//                        range = diff;
//                        find_id = id;
//                    }
//                }
//
//                employ_info.employ_type = super::ObjectEmployType::PointToPoint(PointToPointWorkInfo {
//                    source: self.sources_info[find_id].basic_info.clone(),
//                    target: self_info.clone(),
//                });
//
//                employ_info.at_least_number = 4;
//                employ_info.max_number = 8;
//                employ_info.normal_number = 6;
//
//                offices.insert(String::from("controller")+ find_id, employ_info.clone());
//
//                for (id, source) in &self.sources_info {
//                    if find_id == id{
//                        continue;
//                    }
//
//                    employ_info.employ_type = super::ObjectEmployType::PointToPoint(PointToPointWorkInfo {
//                        source: source.basic_info.clone(),
//                        target: self_info.clone(),
//                    });
//
//                    employ_info.at_least_number = 0;
//                    employ_info.max_number = 10;
//                    employ_info.normal_number = 5;
//
//                    offices.insert(String::from("controller")+ id, employ_info.clone());
//                }
//            }
//        }
    }

    pub fn init_default_offers(&mut self){
        self.init_spawn_office();

        self.init_controller_office();
    }
}