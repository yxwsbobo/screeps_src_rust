use screeps::{objects::*, Transferable};

//unsafe impl Transferable for RoomObject {}
//unsafe impl Withdrawable for RoomObject {}
//unsafe impl Attackable for RoomObject {}
//unsafe impl HasStore for RoomObject {}
//unsafe impl HasId for RoomObject {}
//unsafe impl StructureProperties for RoomObject {}
//unsafe impl CanStoreEnergy for RoomObject {}
//unsafe impl HasCooldown for RoomObject {}
//unsafe impl CanDecay for RoomObject {}
//unsafe impl OwnedStructureProperties for RoomObject {}

//creep_simple_concrete_action! {
//    (_attack_controller(RoomObject) -> attackController),
//    (_build(RoomObject) -> build),
//    (_claim_controller(RoomObject) -> claimController),
//    (_generate_safe_mode(RoomObject) -> generateSafeMode),
//    (_harvest(RoomObject) -> harvest),
//    (_heal(RoomObject) -> heal),
//    (_pickup(RoomObject) -> pickup),
//    (_ranged_heal(RoomObject) -> rangedHeal),
//    (_reserve_controller(RoomObject) -> reserveController),
//    (_upgrade_controller(RoomObject) -> upgradeController),
//}

//
//impl RoomObject {
//    pub fn energy_full(&self) -> bool {
//        js_unwrap! { @{self.as_ref()}.energy === @{self.as_ref()}.energyCapacity }
//    }
//
//    pub fn energy_empty(&self) -> bool {
//        js_unwrap! { @{self.as_ref()}.energy === 0 }
//    }
//
//    pub fn build_over(&self) -> bool {
//        js_unwrap! { @{self.as_ref()}.progress === @{self.as_ref()}.progressTotal }
//    }
//}
