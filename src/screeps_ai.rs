

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


//pub fn get_creep_manager() ->&'static mut creep_manager::Manager{
//    &mut SuperAI::get_ai().cp_manager
//}

pub fn get_object_manager() ->&'static mut object_manager::Manager{
    &mut SuperAI::get_ai().obj_manager
}

pub fn get_offer_manager() ->&'static mut offer_manager::Manager{
    &mut SuperAI::get_ai().offer_mgr
}

