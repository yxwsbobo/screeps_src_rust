

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
    current_tick:u32,
    low_time:bool,
    middle_time:bool,
    long_time:bool,
}


//pub fn get_creep_manager() ->&'static mut creep_manager::Manager{
//    &mut SuperAI::get_ai().cp_manager
//}

pub fn low_time() ->bool{
    SuperAI::get_ai().low_time
}

pub fn middle_time() ->bool{
    SuperAI::get_ai().middle_time
}

pub fn long_time() ->bool{
    SuperAI::get_ai().long_time
}

pub fn get_tick() -> u32{
    SuperAI::get_ai().current_tick
}


pub fn get_object_manager() ->&'static mut object_manager::Manager{
    &mut SuperAI::get_ai().obj_manager
}

pub fn get_offer_manager() ->&'static mut offer_manager::Manager{
    &mut SuperAI::get_ai().offer_mgr
}

