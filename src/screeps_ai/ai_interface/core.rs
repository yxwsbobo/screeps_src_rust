use screeps_ai::SuperAI;

#[derive(Debug)]
struct MyHeapStatistics {
    total_heap_size: u32,
    total_heap_size_executable: u32,
    total_physical_size: u32,
    used_heap_size: u32,
    heap_size_limit: u32,
    malloced_memory: u32,
    peak_malloced_memory: u32,
    does_zap_garbage: u32,
    externally_allocated_size: u32,
}

impl SuperAI {
    fn check_run_init(&mut self) -> bool {
        if self.init_flag {
            return true;
        }

        if screeps::game::cpu::get_used() > 30. {
            return false;
        }

        info!("in init used cpu :{}", screeps::game::cpu::get_used());
        let mut init_temp = self.obj_manager.init();
        info!(
            "obj_manager init used cpu :{}",
            screeps::game::cpu::get_used()
        );

        init_temp = self.cp_manager.init();
        info!(
            "cp_manager init used cpu :{}",
            screeps::game::cpu::get_used()
        );

        init_temp = self.offer_mgr.init();
        info!(
            "offer_mgr init used cpu :{}",
            screeps::game::cpu::get_used()
        );

        self.init_flag = true;

        let heap_info = screeps::game::cpu::get_heap_statistics();
        let heap_info = MyHeapStatistics {
            total_heap_size: heap_info.total_heap_size,
            total_heap_size_executable: heap_info.total_heap_size_executable,
            total_physical_size: heap_info.total_physical_size,
            used_heap_size: heap_info.used_heap_size,
            heap_size_limit: heap_info.heap_size_limit,
            malloced_memory: heap_info.malloced_memory,
            peak_malloced_memory: heap_info.peak_malloced_memory,
            does_zap_garbage: heap_info.does_zap_garbage,
            externally_allocated_size: heap_info.externally_allocated_size,
        };
        //        logging::setup_logging(logging::Debug);

        info!(
            "in init, cost cpu: {}\n heap info:{:#?}",
            screeps::game::cpu::get_used(),
            heap_info
        );
        false
    }

    pub(crate) fn ai_run_once(&mut self) {
        if !self.check_run_init() {
            return;
        }

        debug!("start cost cpu: {}", screeps::game::cpu::get_used());
        self.obj_manager.clean_invalid_object();
        debug!("obj_manager cost cpu: {}", screeps::game::cpu::get_used());

        self.cp_manager.check_create_creep();
        debug!("cp_manager cost cpu: {}", screeps::game::cpu::get_used());

        self.offer_mgr.set_offer_state();
        debug!(
            "set_offer_state cost cpu: {}",
            screeps::game::cpu::get_used()
        );

        self.offer_mgr.creeps_do_work();
        debug!(
            "creeps_do_work cost cpu: {}",
            screeps::game::cpu::get_used()
        );
    }
}
