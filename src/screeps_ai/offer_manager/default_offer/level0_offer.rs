use screeps_ai::offer_manager::{Manager, ActionType};
use screeps_ai::get_object_manager;
use screeps_ai::object_manager::ScreepsObjectType;
use screeps_ai::offer_manager::offer_interface::get_offer_mut;
use screeps_ai::offer_manager::default_offer::basic_offers::{BASIC_UPGRADE_OFFER_LEVEL, BASIC_BUILD_OFFER_LEVEL, BASIC_SPAWN_OFFER_LEVEL, BASIC_EXTENSION_OFFER_LEVEL, BASIC_REPAIR_OFFER_LEVEL, BASIC_NORMAL_TRANSFER_OFFER_LEVEL};

impl Manager {
    pub fn init_offer_level0(&mut self){
        let count = self.get_upgrade_offer_number();

        for index in 1 .. count {
            let upgrade_offer = self.get_basic_employ_index(BASIC_UPGRADE_OFFER_LEVEL,index);
            let build_offer = self.get_basic_employ_index(BASIC_BUILD_OFFER_LEVEL,index);
            let spawn_offer = self.get_basic_employ_index(BASIC_SPAWN_OFFER_LEVEL,index);
            let extension_offer = self.get_basic_employ_index(BASIC_EXTENSION_OFFER_LEVEL,index);

            get_offer_mut(&extension_offer).max_number = upgrade_offer.max_number * 3;
            get_offer_mut(&upgrade_offer).max_number = 0;
            get_offer_mut(&spawn_offer).max_number = 0;

            Manager::connect_employ_on_pausing(&build_offer, &spawn_offer);

        }

        let spawn_offer = self.get_basic_employ(BASIC_SPAWN_OFFER_LEVEL);
        get_offer_mut(&spawn_offer).max_number = spawn_offer.max_number * 4;

        let upgrade_offer = self.get_basic_employ(BASIC_UPGRADE_OFFER_LEVEL);
        get_offer_mut(&upgrade_offer).max_number = 0;
    }
}