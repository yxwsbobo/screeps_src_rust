#![recursion_limit = "128"]
extern crate fern;
#[macro_use]
extern crate log;
extern crate screeps;
#[macro_use]
extern crate stdweb;
extern crate core;

mod logging;
mod creep_manager;

static mut GLOBAL_INIT_FLAG: bool = false;
static mut GLOBAL_INIT_NUMBERS: i32 = 0;


fn main() {
    stdweb::initialize();
    logging::setup_logging(logging::Info);

    js! {
        var game_loop = @{game_loop};

        module.exports.loop = function() {
            // Provide actual error traces.
            try {
                game_loop();
            } catch (error) {
                // console_error function provided by 'screeps-game-api'
                console_error("caught exception:", error);
                if (error.stack) {
                    console_error("stack trace:", error.stack);
                }
                console_error("resetting VM next tick.");
                // reset the VM since we don't know if everything was cleaned up and don't
                // want an inconsistent state.
                module.exports.loop = wasm_initialize;
            }
        }
    }
}

fn game_loop() {
    let start_cpu = screeps::game::cpu::get_used();

    let cp_manager = creep_manager::get_manager();

    unsafe {
        if ! GLOBAL_INIT_FLAG {
            GLOBAL_INIT_NUMBERS +=1;
            info!("init cache data! times: {}", GLOBAL_INIT_NUMBERS);
            if cp_manager.init() == 0{
                GLOBAL_INIT_FLAG = true;
            }
            return;
        }
    }

    cp_manager.check_create_creep();
    cp_manager.creep_do_work();

    info!("start cpu: {}, end cpu: {}",start_cpu, screeps::game::cpu::get_used())
}


