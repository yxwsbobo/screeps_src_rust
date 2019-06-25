use screeps_ai::{SuperAI, creep_manager};

static mut GLOBAL_AI_CACHE:Option<SuperAI> = None;

impl SuperAI {
    pub fn init_global_ai(){
        unsafe {
            if let None = GLOBAL_AI_CACHE {
                GLOBAL_AI_CACHE = Some(SuperAI{
                    init_flag:false,
                    cp_manager: creep_manager::Manager::new()
                })
            }
        }
    }

    pub fn run_once(){
        unsafe {
            GLOBAL_AI_CACHE.as_mut().unwrap().ai_run_once();
        }
    }

    fn check_run_init(&mut self)->bool{
        if self.init_flag{
            return true;
        }
        self.init_flag = self.cp_manager.init();

        false
    }

    fn ai_run_once(&mut self){
        if !self.check_run_init(){
            return;
        }

        self.cp_manager.check_create_creep();
        self.cp_manager.creep_do_work();
    }

}