use super::Manager;

use screeps::{ prelude::*, ReturnCode};
use screeps_ai::offer_manager::{WorkerInfo, WorkerState, WorkType, PointToPointWorkInfo};
use screeps_ai::{get_object_manager, get_offer_manager};
use screeps_ai::object_manager::{ObjectBasicInfo, ScreepsObjectType};

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

        let r = creep.transfer_all(spawn,screeps::constants::ResourceType::Energy);

        match r {
            ReturnCode::Full =>{
                get_offer_manager().pause_group_offer(&target_info.id);
            }
            ReturnCode::NotInRange =>{
                creep.move_to(spawn);
            }
            ReturnCode::Ok =>{}
            _ =>{
                warn!("couldn't harvest: {:?}", r);
            }
        }
    }

    fn do_controller_target_work(creep:& screeps::objects::Creep, target_info: &ObjectBasicInfo){
        let controller:&screeps::objects::StructureController = &screeps::game::get_object_typed(
            &target_info.id).expect("get_object_typed with controller failed").unwrap();

        let r = creep.upgrade_controller(controller);
        if r == ReturnCode::NotInRange {
            creep.move_to(controller);
        } else if r != ReturnCode::Ok {
            warn!("couldn't upgrade: {:?}", r);
        }
    }
//
//    fn do_harvest_work(creep:& screeps::objects::Creep, target: &screeps::objects::RoomObject){
//        if creep.pos().is_near_to(target) {
//            let r = creep.harvest(target);
//            if r != ReturnCode::Ok {
//                warn!("couldn't harvest: {:?}", r);
//            }
//        } else {
//            creep.move_to(target);
//        }
//    }

    fn creep_do_source_work(creep:& screeps::objects::Creep, work_info: &mut WorkerInfo){
        if creep.carry_total() == creep.carry_capacity() {
            work_info.state = WorkerState::DoTargetWork;
            return Manager::creep_do_target_work(creep, work_info);
        }


        match &work_info.info {
            WorkType::PointToPoint(info) =>{
                let target_info = get_object_manager().get_object(&info.source);
                Manager::do_source_source_work(creep, target_info);
            }
            _=>{
                warn!("not support work type: {:#?}", work_info.info);
            }
        }
    }

    fn creep_do_p2p_target_work(creep:& screeps::objects::Creep, target_info: &ObjectBasicInfo){
        match target_info.obj_type {
            ScreepsObjectType::Spawn =>{Manager::do_spawn_target_work(creep, target_info);}
            ScreepsObjectType::Controller =>{Manager::do_controller_target_work(creep,target_info);}
            _=>{
                warn!("not support target type: {:#?}", target_info.obj_type);
            }
        }
    }

    fn creep_do_target_work(creep:& screeps::objects::Creep, work_info: &mut WorkerInfo){
        if creep.carry_total() == 0 {
            work_info.state = WorkerState::DoSourceWork;
            return Manager::creep_do_source_work(creep, work_info);
        }

        match &work_info.info {
            WorkType::PointToPoint(info) =>{
                let target_info = get_object_manager().get_object(&info.target);
                Manager::creep_do_p2p_target_work(creep,target_info);
            }
            _=>{
                warn!("not support work type: {:#?}", work_info.info);
            }
        }
    }

    pub fn creep_do_work(&mut self){
        for creep in screeps::game::creeps::values() {
            if creep.spawning() {
                continue;
            }
            let name = creep.name();
            let mut worker_info = self.workers_info.get_mut(&name).
                expect("can't find worker_info");
            match worker_info.state {
                WorkerState::StupidWorker=>{
                    info!("StupidWorker do nothing");
                }

                WorkerState::DoSourceWork=>{
                    Manager::creep_do_source_work(& creep, &mut worker_info);
                }

                WorkerState::DoTargetWork =>{
                    Manager::creep_do_target_work(& creep, &mut worker_info);
                }
            }
        }
    }
}