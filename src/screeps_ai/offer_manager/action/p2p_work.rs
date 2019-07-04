use screeps_ai::offer_manager::{Manager, PointToPointWorkInfo, WorkerState};

impl Manager {
    pub(crate) fn creep_do_p2p_work(
        creep: &screeps::objects::Creep,
        state: &mut WorkerState,
        info: &PointToPointWorkInfo,
    ) {
        let energy = creep.carry_total();
        if energy == 0 {
            if let WorkerState::DoTargetWork = state {
                *state = WorkerState::DoSourceWork;
            }
        } else if energy == creep.carry_capacity() {
            if let WorkerState::DoSourceWork = state {
                *state = WorkerState::DoTargetWork;
            }
        }
//
//        if screeps::game::time() % 40 == 0{
//            info!("creep: {}, work info: {:#?}, state: {:#?}", creep.name(), info, state);
//        }

        match state {
            WorkerState::StupidWorker => {
                info!("StupidWorker do nothing");
            }

            WorkerState::DoSourceWork => {
                Manager::creep_do_work(creep, &info.source, &info.source_action);
            }

            WorkerState::DoTargetWork => {
                Manager::creep_do_work(creep, &info.target, &info.target_action);
            }
        }
    }
}
