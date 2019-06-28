use screeps_ai::offer_manager::{Manager, WorkerState, PointToPointWorkInfo};
use screeps_ai::object_manager::{ObjectBasicInfo, ScreepsObjectType};

impl Manager {

    fn creep_do_p2p_source_work(creep:& screeps::objects::Creep, source_info: &ObjectBasicInfo){

        match &source_info.obj_type{
            ScreepsObjectType::Source=>{
                Manager::do_source_source_work(creep, source_info);
            }
            _=>{
                warn!("not support target type: {:#?} in p2p work, source", source_info.obj_type);
            }
        }
    }

    fn creep_do_p2p_target_work(creep:& screeps::objects::Creep, target_info: &ObjectBasicInfo){
        match target_info.obj_type {
            ScreepsObjectType::Spawn =>{Manager::do_spawn_target_work(creep, target_info);}
            ScreepsObjectType::Controller =>{Manager::do_controller_target_work(creep,target_info);}
            _=>{
                warn!("not support target type: {:#?} in p2p work, target", target_info.obj_type);
            }
        }
    }

    pub(crate) fn creep_do_p2p_work(creep:&screeps::objects::Creep, state:&mut WorkerState, info:&PointToPointWorkInfo){
        match state {
            WorkerState::StupidWorker=>{
                info!("StupidWorker do nothing");
            }

            WorkerState::DoSourceWork=>{
                if creep.carry_total() == creep.carry_capacity() {
                    *state = WorkerState::DoTargetWork;
                    Manager::creep_do_p2p_target_work(creep, info.target)
                }else{
                    Manager::creep_do_p2p_source_work(creep, info.source)
                }
            }

            WorkerState::DoTargetWork =>{
                if creep.carry_total() == 0 {
                    *state = WorkerState::DoSourceWork;
                    Manager::creep_do_p2p_source_work(creep, info.source)
                }else{
                    Manager::creep_do_p2p_target_work(creep,info.target)
                }
            }
        }

    }
}