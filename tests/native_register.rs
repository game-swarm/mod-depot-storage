use bevy::prelude::App;
use serde_json::json;
use swarm_engine_api::prelude::{RoomId, WorldMode};
use swarm_engine_plugin_sdk::prelude::{
    BodyPartRegistry, Drone, InstalledPluginDescriptors, NativeModConfig,
    NativeModInstallExpectation, NativeModRegisterContext, NativeModRegisterError, Position,
    Structure, StructureType,
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

#[test]
fn app_update_uses_later_funded_depot_when_first_eligible_depot_is_empty() {
    let mut app = registered_default_app();

    app.world_mut().spawn((
        ForwardDepot::default(),
        Position {
            x: 0,
            y: 0,
            room: RoomId(1),
        },
    ));
    let funded_depot = app
        .world_mut()
        .spawn((
            ForwardDepot {
                storage: [("Energy".to_string(), 3)].into(),
                ..Default::default()
            },
            Position {
                x: 1,
                y: 0,
                room: RoomId(1),
            },
        ))
        .id();
    let mut aging_drone = Drone::new(1, Vec::new(), &BodyPartRegistry::default());
    aging_drone.age = 1;
    let drone = app
        .world_mut()
        .spawn((
            aging_drone,
            Position {
                x: 0,
                y: 0,
                room: RoomId(1),
            },
        ))
        .id();

    app.update();

    assert_eq!(app.world().entity(drone).get::<Drone>().unwrap().age, 0);
    assert_eq!(
        app.world()
            .entity(funded_depot)
            .get::<ForwardDepot>()
            .unwrap()
            .storage
            .get("Energy"),
        Some(&2)
    );
}

#[test]
fn app_update_uses_later_depot_when_first_eligible_depot_repairs_zero_age() {
    let mut app = registered_default_app();
    let first_depot = app
        .world_mut()
        .spawn((
            ForwardDepot {
                storage: [("Energy".to_string(), 3)].into(),
                repair_age_per_energy: 0,
                ..Default::default()
            },
            Position {
                x: 0,
                y: 0,
                room: RoomId(1),
            },
        ))
        .id();
    let funded_depot = app
        .world_mut()
        .spawn((
            ForwardDepot {
                storage: [("Energy".to_string(), 3)].into(),
                ..Default::default()
            },
            Position {
                x: 1,
                y: 0,
                room: RoomId(1),
            },
        ))
        .id();
    let mut aging_drone = Drone::new(1, Vec::new(), &BodyPartRegistry::default());
    aging_drone.age = 1;
    let drone = app
        .world_mut()
        .spawn((
            aging_drone,
            Position {
                x: 0,
                y: 0,
                room: RoomId(1),
            },
        ))
        .id();

    app.update();

    assert_eq!(app.world().entity(drone).get::<Drone>().unwrap().age, 0);
    assert_eq!(
        app.world()
            .entity(first_depot)
            .get::<ForwardDepot>()
            .unwrap()
            .storage
            .get("Energy"),
        Some(&3)
    );
    assert_eq!(
        app.world()
            .entity(funded_depot)
            .get::<ForwardDepot>()
            .unwrap()
            .storage
            .get("Energy"),
        Some(&2)
    );
}

#[test]
fn native_register_rejects_unknown_fields_without_installing_the_plugin() {
    let mut app = App::new();
    let error = {
        let mut context = NativeModRegisterContext::new(
            &mut app,
            "depot-storage",
            WorldMode::Default,
            NativeModConfig::from_defaults(json!({
                "depot_capacity": 10_000,
                "depot_hits": 5_000,
                "repair_range": 1,
                "repair_capacity": 5,
                "unexpected": true
            })),
            NativeModInstallExpectation::enabled("0.1.0"),
        );

        register(&mut context).expect_err("unknown config field must fail registration")
    };

    assert!(matches!(
        error,
        NativeModRegisterError::InvalidConfig { .. }
    ));
    assert!(app.world().get_resource::<DepotStorageConfig>().is_none());
    assert!(
        app.world()
            .get_resource::<InstalledPluginDescriptors>()
            .is_none()
    );
}

fn registered_default_app() -> App {
    let mut app = App::new();
    let mut context = NativeModRegisterContext::new(
        &mut app,
        "depot-storage",
        WorldMode::Default,
        NativeModConfig::from_defaults(json!({
            "depot_capacity": 10_000,
            "depot_hits": 5_000,
            "repair_range": 1,
            "repair_capacity": 5
        })),
        NativeModInstallExpectation::enabled("0.1.0"),
    );
    register(&mut context).expect("register depot-storage");
    app
}
