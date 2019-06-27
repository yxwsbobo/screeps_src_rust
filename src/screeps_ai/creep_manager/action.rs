use super::Manager;

use screeps::{ prelude::*, ReturnCode};

impl Manager {
    fn init_workers(&mut self){
        for creep in screeps::game::creeps::values() {
            self.born_init_creep(&creep.name())
        }
    }

    pub fn action_init(&mut self) ->bool{
        self.init_workers();

        true
    }

    fn do_source_source_work(creep:& screeps::objects::Creep, work_info: &mut WorkerInfo){
        let source:&screeps::objects::Source = &screeps::game::get_object_typed(
            &work_info.sr_info.source.id.to_string()).expect("get_object_typed with source failed").unwrap();
        if creep.pos().is_near_to(source) {
            let r = creep.harvest(source);
            if r != ReturnCode::Ok {
                warn!("couldn't harvest: {:?}", r);
            }
        } else {
            creep.move_to(source);
        }
    }

    fn do_spawn_target_work(creep:& screeps::objects::Creep, work_info: &mut WorkerInfo){
        let spawn:&screeps::objects::StructureSpawn = &screeps::game::get_object_typed(
            &work_info.sr_info.target.id.to_string()).expect("get_object_typed with controller failed").unwrap();

        let r = creep.transfer_all(spawn,screeps::constants::ResourceType::Energy);
        if r == ReturnCode::NotInRange {
            creep.move_to(spawn);
        } else if r != ReturnCode::Ok {
            warn!("couldn't transfer: {:?}", r);
        }

    }

    fn do_controller_target_work(creep:& screeps::objects::Creep, work_info: &mut WorkerInfo){
        let controller:&screeps::objects::StructureController = &screeps::game::get_object_typed(
            &work_info.sr_info.target.id.to_string()).expect("get_object_typed with controller failed").unwrap();

        let r = creep.upgrade_controller(controller);
        if r == ReturnCode::NotInRange {
            creep.move_to(controller);
        } else if r != ReturnCode::Ok {
            warn!("couldn't upgrade: {:?}", r);
        }
    }

    fn creep_do_source_work(creep:& screeps::objects::Creep, work_info: &mut WorkerInfo){
        if creep.carry_total() == creep.carry_capacity() {
            work_info.state = WorkerState::DoTargetWork;
            return Manager::creep_do_target_work(creep, work_info);
        }

        match work_info.sr_info.source.obj_type {
            ScreepsObjectType::Source =>{ Manager::do_source_source_work(creep, work_info); }
            _ =>{
                warn!("not support do source work, source type: ");
            }
        }
    }

    fn creep_do_target_work(creep:& screeps::objects::Creep, work_info: &mut WorkerInfo){
        if creep.carry_total() == 0 {
            work_info.state = WorkerState::DoSourceWork;
            return Manager::creep_do_source_work(creep, work_info);
        }

        match work_info.sr_info.target.obj_type {
            ScreepsObjectType::Spawn =>{ Manager::do_spawn_target_work(creep, work_info); }
            ScreepsObjectType::Controller =>{ Manager::do_controller_target_work(creep,work_info); }

            _ =>{
                warn!("not support do target work, source type: ", );
            }
        }
    }

    pub fn creep_do_work(&mut self){
        for creep in screeps::game::creeps::values() {
            let name = creep.name();
            if creep.spawning() {
                continue;
            }
            let mut worker_info = self.workers_info.get_mut(&name).
                expect("can't find worker_info");
            match worker_info.state {
                WorkerState::StupidWorker=>{

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