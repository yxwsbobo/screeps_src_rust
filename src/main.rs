//#![recursion_limit = "128"]
extern crate fern;
#[macro_use]
extern crate log;
extern crate screeps;
#[macro_use]
extern crate stdweb;
extern crate core;

mod logging;
mod screeps_ai;

fn my_test_call() {}

fn main() {
    stdweb::initialize();
    logging::setup_logging(logging::Debug);
    info!("in main");
    //    if screeps::game::shard::name() == "shard2" {
    //        return;
    //    }
    if screeps::game::shard::name() == "shard3" {
        screeps_ai::SuperAI::init_global_ai();
    }

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
    if screeps::game::shard::name() == "shard2" {
        return;
    }
    //    info!("in loop");
    let start_cpu = screeps::game::cpu::get_used();

    if start_cpu > 30. {
        warn!("in start loop used cup: {}", start_cpu);
        return;
    }

    screeps_ai::SuperAI::run_once();

    //    info!(
    //        "start cpu: {}, end cpu: {}",
    //        start_cpu,
    //        screeps::game::cpu::get_used()
    //    )
}
