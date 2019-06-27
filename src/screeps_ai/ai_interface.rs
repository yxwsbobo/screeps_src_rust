mod core;

use screeps_ai::{SuperAI, creep_manager, object_manager, offer_manager};

static mut GLOBAL_AI_CACHE:Option<SuperAI> = None;

impl SuperAI {
    pub fn init_global_ai(){
        unsafe {
            if let None = GLOBAL_AI_CACHE {
                GLOBAL_AI_CACHE = Some(SuperAI{
                    init_flag:false,
                    cp_manager: creep_manager::Manager::new(),
                    obj_manager:object_manager::Manager::new(),
                    offer_mgr: offer_manager::Manager::new(),
                })
            }
        }
    }

    pub fn run_once(){
        unsafe {
            GLOBAL_AI_CACHE.as_mut().unwrap().ai_run_once();
        }
    }
}