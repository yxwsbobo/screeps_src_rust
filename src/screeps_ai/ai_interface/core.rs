use screeps_ai::SuperAI;

impl SuperAI {
    fn check_run_init(&mut self)->bool{
        if self.init_flag{ return true; }

        info!("in init");
        self.init_flag = self.obj_manager.init() &&
            self.offer_mgr.init() &&
            self.cp_manager.init();

        false
    }

    pub(crate) fn ai_run_once(&mut self){
        if !self.check_run_init(){ return; }

        self.cp_manager.check_create_creep();
        self.offer_mgr.creep_do_work();

    }
}