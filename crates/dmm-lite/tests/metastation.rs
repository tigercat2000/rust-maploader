use dmm_lite::{
    block::{get_block_locations, parse_block},
    parse_map_multithreaded,
    prefabs::{detect_tgm, get_prefab_locations, parse_prefab_line},
};
use winnow::Parser as _;

#[test]
fn test_tgm_detection() {
    let metastation = std::fs::read_to_string("./tests/maps/MetaStation.dmm").unwrap();
    let metastation_tgm = std::fs::read_to_string("./tests/maps/MetaStation-tgm.dmm").unwrap();
    // tgm files sometimes have a header
    // //MAP CONVERTED BY dmm2tgm.py THIS HEADER COMMENT PREVENTS RECONVERSION, DO NOT REMOVE
    let metastation_tgm: String = metastation_tgm
        .lines()
        .map(|l| format!("{}\n", l))
        .skip(1)
        .collect();

    assert!(!detect_tgm(&mut metastation.as_str()));
    assert!(detect_tgm(&mut metastation_tgm.as_str()));
}

#[test]
fn test_prefab_detection() {
    let metastation = std::fs::read_to_string("./tests/maps/MetaStation.dmm").unwrap();
    let metastation_tgm = std::fs::read_to_string("./tests/maps/MetaStation-tgm.dmm").unwrap();
    // tgm files sometimes have a header
    // //MAP CONVERTED BY dmm2tgm.py THIS HEADER COMMENT PREVENTS RECONVERSION, DO NOT REMOVE
    let metastation_tgm: String = metastation_tgm
        .lines()
        .map(|l| format!("{}\n", l))
        .skip(1)
        .collect();

    let metastation_location_count = get_prefab_locations(&metastation).len();
    let metastation_tgm_location_count = get_prefab_locations(&metastation_tgm).len();

    assert_eq!(metastation_location_count, metastation_tgm_location_count);
    assert_eq!(metastation_location_count, 8564);
}

#[test]
fn test_prefab_line() {
    let metastation = std::fs::read_to_string("./tests/maps/MetaStation.dmm").unwrap();
    #[allow(clippy::format_collect)] // I'm not figuring out fold for a test case
    let metastation: String = metastation
        .lines()
        .skip(11)
        .map(|l| format!("{}\n", l))
        .collect();
    let metastation_tgm = std::fs::read_to_string("./tests/maps/MetaStation-tgm.dmm").unwrap();
    // tgm files sometimes have a header
    // //MAP CONVERTED BY dmm2tgm.py THIS HEADER COMMENT PREVENTS RECONVERSION, DO NOT REMOVE
    let metastation_tgm: String = metastation_tgm
        .lines()
        .map(|l| format!("{}\n", l))
        .skip(15)
        .take(10)
        .collect();

    assert_eq!(
        parse_prefab_line.parse_next(&mut metastation.as_str()),
        Ok((
            "aal",
            vec![
                ("/obj/structure/cable", None),
                (
                    "/obj/machinery/atmospherics/pipe/smart/manifold4w/supply/hidden/layer4",
                    None
                ),
                (
                    "/obj/machinery/atmospherics/pipe/smart/manifold4w/scrubbers/hidden/layer2",
                    None
                ),
                ("/obj/structure/disposalpipe/segment", None),
                ("/obj/effect/turf_decal/tile/neutral", Some("{dir = 4}")),
                ("/turf/open/floor/iron", None),
                ("/area/station/hallway/primary/port", None)
            ]
        ))
    );
    assert_eq!(
        parse_prefab_line.parse_next(&mut metastation_tgm.as_str()),
        Ok((
            "aal",
            vec![
                ("/obj/structure/cable", None),
                (
                    "/obj/machinery/atmospherics/pipe/smart/manifold4w/supply/hidden/layer4",
                    None
                ),
                (
                    "/obj/machinery/atmospherics/pipe/smart/manifold4w/scrubbers/hidden/layer2",
                    None
                ),
                ("/obj/structure/disposalpipe/segment", None),
                (
                    "/obj/effect/turf_decal/tile/neutral",
                    Some("{\n\tdir = 4\n\t}")
                ),
                ("/turf/open/floor/iron", None),
                ("/area/station/hallway/primary/port", None)
            ]
        ))
    );
}

#[test]
fn full_prefab_parse() {
    let metastation = std::fs::read_to_string("./tests/maps/MetaStation.dmm").unwrap();
    let metastation_tgm = std::fs::read_to_string("./tests/maps/MetaStation-tgm.dmm").unwrap();

    let metastation_locations = get_prefab_locations(&metastation);
    for loc in metastation_locations {
        let mut parse = &metastation[loc..];
        assert!(parse_prefab_line.parse_next(&mut parse).is_ok())
    }

    let metastation_tgm_locations = get_prefab_locations(&metastation_tgm);
    for loc in metastation_tgm_locations {
        let mut parse = &metastation_tgm[loc..];
        assert!(parse_prefab_line.parse_next(&mut parse).is_ok())
    }
}

#[test]
fn test_block_detection() {
    let metastation = std::fs::read_to_string("./tests/maps/MetaStation.dmm").unwrap();
    let metastation_tgm = std::fs::read_to_string("./tests/maps/MetaStation-tgm.dmm").unwrap();
    // tgm files sometimes have a header
    // //MAP CONVERTED BY dmm2tgm.py THIS HEADER COMMENT PREVENTS RECONVERSION, DO NOT REMOVE
    let metastation_tgm: String = metastation_tgm
        .lines()
        .map(|l| format!("{}\n", l))
        .skip(1)
        .collect();

    let metastation_location_count = get_block_locations(&metastation).len();
    assert_eq!(metastation_location_count, 1);
    let metastation_tgm_location_count = get_block_locations(&metastation_tgm).len();
    assert_eq!(metastation_tgm_location_count, 255);
}

#[test]
fn full_block_parse() {
    let metastation = std::fs::read_to_string("./tests/maps/MetaStation.dmm").unwrap();
    let metastation_tgm = std::fs::read_to_string("./tests/maps/MetaStation-tgm.dmm").unwrap();

    let metastation_locations = get_block_locations(&metastation);
    for loc in metastation_locations {
        let mut parse = &metastation[loc..];
        let value = parse_block.parse_next(&mut parse);
        match value {
            Ok(_) => {}
            Err(e) => panic!("Test Failed at {parse:#?}: {:#?}", e),
        }
    }

    let metastation_tgm_locations = get_block_locations(&metastation_tgm);
    for loc in metastation_tgm_locations {
        let mut parse = &metastation_tgm[loc..];
        let value = parse_block.parse_next(&mut parse);
        match value {
            Ok(_) => {}
            Err(e) => panic!("Test Failed at {parse:#?}: {:#?}", e),
        }
    }
}

#[test]
fn full_parse() {
    let map = std::fs::read_to_string("./tests/maps/MetaStation.dmm").unwrap();
    let map_tgm = std::fs::read_to_string("./tests/maps/MetaStation-tgm.dmm").unwrap();

    let (meta, (prefabs, blocks)) = parse_map_multithreaded(&map).unwrap();
    assert!(!meta.is_tgm);
    assert_eq!(prefabs.len(), 8564);
    assert_eq!(blocks.len(), 1);

    let (meta, (tgm_prefabs, tgm_blocks)) = parse_map_multithreaded(&map_tgm).unwrap();
    assert!(meta.is_tgm);
    assert_eq!(tgm_prefabs.len(), 8564);
    assert_eq!(tgm_blocks.len(), 255);
}