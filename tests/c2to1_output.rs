use cyl_package_tool::cylheim_tools::{cytus1_chart::Cytus1Chart, CylheimChart};

#[test]
fn tempo_based_bpm_preserves_all_other_output_fields() {
    for (time_base, page_ticks, tempo) in [
        (480, 960, 500_000),
        (480, 1440, 500_000),
        (960, 1440, 600_000),
    ] {
        let source: CylheimChart = serde_json::from_value(serde_json::json!({
            "format_version": 1, "time_base": time_base, "start_offset_time": 0,
            "page_list": [{"start_tick": 0, "end_tick": page_ticks, "scan_line_direction": 1}],
            "tempo_list": [{"tick": 0, "value": 0}, {"tick": 120, "value": tempo}],
            "event_order_list": [],
            "note_list": [
                {"page_index": 0, "type": 0, "id": 0, "tick": 240, "x": 0.2, "has_sibling": false, "hold_tick": 0, "next_id": -1},
                {"page_index": 0, "type": 1, "id": 1, "tick": 360, "x": 0.4, "has_sibling": false, "hold_tick": 120, "next_id": -1},
                {"page_index": 0, "type": 3, "id": 2, "tick": 600, "x": 0.6, "has_sibling": false, "hold_tick": 0, "next_id": 3},
                {"page_index": 0, "type": 4, "id": 3, "tick": 720, "x": 0.8, "has_sibling": false, "hold_tick": 0, "next_id": -1}
            ]
        })).unwrap();
        let original = source.to_cytus1_chart_with_pageshift(false).unwrap();
        let mut tempo_based = source.to_cytus1_chart_with_music_bpm(false).unwrap();
        assert!((*tempo_based.bpm() - 120_000_000.0 / tempo as f64).abs() < 1e-9);
        if page_ticks != 2 * time_base {
            assert!((*original.bpm() - *tempo_based.bpm()).abs() > 1.0);
        }
        // Normalizing BPM must make the entire exported chart identical, including holds and drag links.
        tempo_based.set_bpm(*original.bpm());
        assert_eq!(tempo_based.to_string(), original.to_string());
        assert!(source.to_cytus1_chart_with_music_bpm(true).is_err());
    }
}

#[test]
fn converted_notes_use_uppercase_and_survive_round_trip() {
    let source: CylheimChart =
        serde_json::from_str(include_str!("resources/test_valid_c2to1_chart.json")).unwrap();
    // Use the same conversion and formatting path as the GUI converter.
    let chart = source.to_cytus1_chart_with_pageshift(false).unwrap();
    assert!(!chart.notes().is_empty());

    let output = chart.to_string();
    let note_lines: Vec<_> = output
        .lines()
        .filter(|line| {
            line.split_whitespace()
                .next()
                .is_some_and(|token| token.eq_ignore_ascii_case("NOTE"))
        })
        .collect();
    assert_eq!(note_lines.len(), chart.notes().len());
    for line in note_lines {
        assert!(line.starts_with("NOTE\t"), "invalid note keyword: {line}");
    }

    let parsed: Cytus1Chart = output.parse().unwrap();
    assert_eq!(parsed.notes().len(), chart.notes().len());
    assert_eq!(parsed.links().len(), chart.links().len());
    assert_eq!(parsed.to_string(), output);
}
