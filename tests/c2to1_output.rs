use cyl_package_tool::cylheim_tools::{cytus1_chart::Cytus1Chart, CylheimChart};

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
