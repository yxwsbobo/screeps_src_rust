

mod creep_manager;
mod object_manager;
mod ai_interface;
mod offer_manager;
mod common;

pub struct SuperAI{
    init_flag:bool,
    cp_manager:creep_manager::Manager,
    obj_manager:object_manager::Manager,
    offer_mgr: offer_manager::Manager,
}
