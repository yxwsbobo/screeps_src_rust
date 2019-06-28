mod p2p_work;

use super::Manager;

use screeps::{ prelude::*, ReturnCode};
use screeps_ai::offer_manager::{WorkerInfo, WorkerState, WorkType, PointToPointWorkInfo, GroupEmployInfo};
use screeps_ai::{get_object_manager, get_offer_manager};
use screeps_ai::object_manager::{ObjectBasicInfo, ScreepsObjectType};
use std::collections::HashMap;
use core::borrow::BorrowMut;

impl Manager {
    fn init_workers(&mut self){
        for creep in screeps::game::creeps::values() {
            self.offer_creep(&creep.name())
        }
    }

    pub fn init_worker_action(&mut self) ->bool{
        self.init_workers();
        true
    }

    fn do_source_source_work(creep:& screeps::objects::Creep, target_info: &ObjectBasicInfo){
        let source:&screeps::objects::Source = &screeps::game::get_object_typed(
            &target_info.id).expect("get_object_typed with source failed").unwrap();

        if creep.pos().is_near_to(source) {
            let r = creep.harvest(source);
            match r {
                ReturnCode::Ok =>{}
                _ =>{
                    warn!("couldn't harvest: {:?}", r);
                }
            }
        } else {
            creep.move_to(source);
        }
    }

    fn do_spawn_target_work(creep:& screeps::objects::Creep, target_info: &ObjectBasicInfo){
        let spawn:&screeps::objects::StructureSpawn = &screeps::game::get_object_typed(
            &target_info.id).expect("get_object_typed with controller failed").unwrap();

        if creep.pos().is_near_to(spawn) {
            let r = creep.transfer_all(spawn,screeps::constants::ResourceType::Energy);
            match r {
                ReturnCode::Full =>{
                    get_offer_manager().pause_group_offer(&target_info.id);
                }
                ReturnCode::Ok =>{}
                _ =>{
                    warn!("couldn't harvest: {:?}", r);
                }
            }
        } else {
            creep.move_to(spawn);
        }
    }

    fn do_controller_target_work(creep:& screeps::objects::Creep, target_info: &ObjectBasicInfo){
        let controller:&screeps::objects::StructureController = &screeps::game::get_object_typed(
            &target_info.id).expect("get_object_typed with controller failed").unwrap();

        if creep.pos().in_range_to(controller,3) {
            let r = creep.upgrade_controller(controller);
            if r == ReturnCode::NotInRange {
                creep.move_to(controller);
            } else if r != ReturnCode::Ok {
                warn!("couldn't upgrade: {:?}", r);
            }
        } else {
            creep.move_to(controller);
        }
    }

    pub fn creeps_do_work(&mut self){
        let creeps:HashMap<String,screeps::objects::Creep> =
            screeps::game::creeps::keys().into_iter().zip(screeps::game::creeps::values().into_iter()).collect();

        for (_, offers) in self.offer_list.borrow_mut() {
            for (_, offer_info) in offers{
                if !offer_info.pausing{

                }else{
                    match &offer_info.offer_type {
                        WorkType::PointToPoint(v)=>{
                            for (name,state) in &mut offer_info.workers {
                                let creep = creeps.get(name).expect("worker not in creeps");
                                if creep.spawning(){ continue }
                                Manager::creep_do_p2p_work(creep,state,v);
                            }
                        }
                        WorkType::CleanRoom => info!("offer CleanRoom not implement"),
                        WorkType::UnKnown => info!("offer Unknown Type")
                    }
                }
            }
        }
    }
}