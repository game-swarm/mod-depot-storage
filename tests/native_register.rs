use bevy::prelude::App;
use serde_json::json;
use swarm_engine_api::prelude::WorldMode;
use swarm_engine_plugin_sdk::prelude::{
    InstalledPluginDescriptors, NativeModConfig, NativeModInstallExpectation,
    NativeModRegisterContext, Structure, StructureType,
};
use swarm_mod_depot_storage::{DepotStorageConfig, ForwardDepot, register};

#[test]
fn native_register_preserves_depot_config_and_update_behavior() {
    let mut app = App::new();
    let mut context = NativeModRegisterContext::new(
        &mut app,
        "depot-storage",
        WorldMode::Default,
        NativeModConfig::from_defaults(json!({
            "depot_capacity": 20_000,
            "depot_hits": 8_000,
            "repair_range": 3,
            "repair_capacity": 9
        })),
        NativeModInstallExpectation::enabled("0.1.0"),
    );

    register(&mut context).expect("register depot-storage");

    let descriptor = app
        .world()
        .resource::<InstalledPluginDescriptors>()
        .get("depot-storage")
        .expect("installed descriptor");
    assert_eq!(descriptor.version, "0.1.0");

    let config = app.world().resource::<DepotStorageConfig>();
    assert_eq!(config.depot_capacity, 20_000);
    assert_eq!(config.depot_hits, 8_000);
    assert_eq!(config.repair_range, 3);
    assert_eq!(config.repair_capacity, 9);

    let depot = app
        .world_mut()
        .spawn((
            ForwardDepot::default(),
            Structure {
                structure_type: StructureType("ForwardDepot"),
                owner: None,
                hits: 100,
                hits_max: 100,
                energy: None,
                energy_capacity: None,
                cooldown: 0,
            },
        ))
        .id();
    app.update();

    let entity = app.world().entity(depot);
    let depot = entity.get::<ForwardDepot>().expect("forward depot");
    assert_eq!(depot.capacity, 20_000);
    assert_eq!(depot.repair_range, 3);
    assert_eq!(depot.repair_capacity, 9);
    let structure = entity.get::<Structure>().expect("structure");
    assert_eq!(structure.hits, 8_000);
    assert_eq!(structure.hits_max, 8_000);
}
