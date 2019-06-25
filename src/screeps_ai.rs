

mod creep_manager;
mod ai_interface;

pub struct SuperAI{
    init_flag:bool,
    cp_manager:creep_manager::Manager,
}