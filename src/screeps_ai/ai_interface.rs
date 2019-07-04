mod core;
mod depends;

use screeps_ai::{creep_manager, object_manager, offer_manager, SuperAI};

static mut GLOBAL_AI_CACHE: Option<SuperAI> = None;

impl SuperAI {
    pub fn init_global_ai() {
        info!("in init global ai");
        unsafe {
            if let None = GLOBAL_AI_CACHE {
                info!("start init global ai");
                GLOBAL_AI_CACHE = Some(SuperAI {
                    init_flag: false,
                    cp_manager: creep_manager::Manager::new(),
                    obj_manager: object_manager::Manager::new(),
                    offer_mgr: offer_manager::Manager::new(),
                    current_tick: 0
                })
            }
        }
    }

    pub fn run_once() {
        SuperAI::get_ai().ai_run_once();
    }

    pub fn get_ai() -> &'static mut SuperAI {
        unsafe { GLOBAL_AI_CACHE.as_mut().unwrap() }
    }
}
