use cyl_package_tool::{convert_chart_to_svg, svg_to_png, CylheimChart};
use std::fs;

const OUTPUT_DIR: &str = "./tests/output/viewer_cases/";
const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";

fn ensure_output_dir() {
    fs::create_dir_all(OUTPUT_DIR).unwrap();
}

fn load_chart(name: &str) -> CylheimChart {
    let path = format!("./tests/resources/{name}");
    let content = fs::read_to_string(&path).unwrap();
    serde_json::from_str(&content).unwrap()
}

fn parse_png_dimensions(png: &[u8]) -> (u32, u32) {
    assert!(png.len() >= 24, "PNG too short for IHDR");
    assert_eq!(&png[0..8], PNG_SIGNATURE, "invalid PNG signature");
    let width = u32::from_be_bytes(png[16..20].try_into().unwrap());
    let height = u32::from_be_bytes(png[20..24].try_into().unwrap());
    (width, height)
}

fn parse_svg_viewbox(svg: &str) -> (u32, u32) {
    let marker = "viewBox=\"0 0 ";
    let start = svg
        .find(marker)
        .unwrap_or_else(|| panic!("viewBox not found in SVG"));
    let rest = &svg[start + marker.len()..];
    let end = rest.find('"').expect("malformed viewBox");
    let parts: Vec<&str> = rest[..end].split_whitespace().collect();
    assert_eq!(parts.len(), 2, "viewBox should have width and height");
    let width: u32 = parts[0].parse().expect("viewBox width");
    let height: u32 = parts[1].parse().expect("viewBox height");
    (width, height)
}

fn save_case(name: &str, svg_text: &str) {
    let svg_path = format!("{OUTPUT_DIR}{name}.svg");
    let png_path = format!("{OUTPUT_DIR}{name}.png");
    fs::write(&svg_path, svg_text).unwrap();
    let png = svg_to_png(svg_text.as_bytes(), None).unwrap();
    fs::write(&png_path, &png).unwrap();
    println!("saved: {svg_path}");
    println!("saved: {png_path}");
}

#[test]
fn svg_to_png_matches_svg_viewbox() {
    let chart = load_chart("viewer_single_click.json");
    let svg = convert_chart_to_svg(chart, 1, false, None).unwrap();
    let svg_text = svg.to_string();
    let (svg_w, svg_h) = parse_svg_viewbox(&svg_text);
    let png = svg_to_png(svg_text.as_bytes(), None).unwrap();
    let (png_w, png_h) = parse_png_dimensions(&png);
    assert_eq!(
        (png_w, png_h),
        (svg_w, svg_h),
        "PNG dimensions should match SVG viewBox"
    );
    println!("svg_to_png dimensions verified: {png_w}x{png_h}");
}

#[test]
fn viewer_single_click_note() {
    ensure_output_dir();
    let chart = load_chart("viewer_single_click.json");
    let svg = convert_chart_to_svg(chart, 1, false, None).unwrap();
    let text = svg.to_string();
    assert!(text.contains("<circle"), "click note should render circles");
    assert!(text.contains("\n0\n"), "page index label should be present");
    save_case("single_click", &text);
}

#[test]
fn viewer_two_page_with_ghost() {
    ensure_output_dir();
    let chart = load_chart("viewer_two_page.json");
    let svg = convert_chart_to_svg(chart, 2, true, None).unwrap();
    let text = svg.to_string();
    assert!(
        text.contains("opacity=\"0.3\""),
        "ghost layer should use opacity 0.3"
    );
    assert!(text.contains("\n0\n"));
    assert!(text.contains("\n1\n"));
    save_case("two_page_ghost", &text);
}

#[test]
fn viewer_drag_chain_links() {
    ensure_output_dir();
    let chart = load_chart("viewer_drag_chain.json");
    let svg = convert_chart_to_svg(chart, 1, false, None).unwrap();
    let text = svg.to_string();
    assert!(
        text.contains("stroke-dasharray=\"3,3\""),
        "drag links should render dashed lines"
    );
    // Each chain segment should connect two distinct note positions (not zero-length).
    let attrs: Vec<&str> = text
        .split("<line")
        .skip(1)
        .filter(|chunk| chunk.contains("stroke-dasharray=\"3,3\""))
        .collect();
    assert_eq!(attrs.len(), 2, "expected two dashed drag chain segments");
    for chunk in attrs {
        let x1 = extract_attr(chunk, "x1");
        let y1 = extract_attr(chunk, "y1");
        let x2 = extract_attr(chunk, "x2");
        let y2 = extract_attr(chunk, "y2");
        assert!(
            (x1 - x2).abs() > 1.0 || (y1 - y2).abs() > 1.0,
            "drag link must connect distinct positions, got ({x1},{y1})->({x2},{y2})"
        );
    }
    save_case("drag_chain", &text);
}

fn extract_attr(chunk: &str, name: &str) -> f64 {
    let marker = format!("{name}=\"");
    let start = chunk
        .find(&marker)
        .unwrap_or_else(|| panic!("{name} not found in line element"));
    let rest = &chunk[start + marker.len()..];
    let end = rest.find('"').expect("malformed attribute");
    rest[..end].parse().unwrap_or_else(|_| panic!("bad {name}"))
}

#[test]
fn viewer_rejects_columns_zero() {
    let chart = load_chart("viewer_single_click.json");
    let err = convert_chart_to_svg(chart, 0, false, None).unwrap_err();
    assert!(
        err.to_string().contains("columns must be greater than 0"),
        "unexpected error: {err}"
    );
    println!("columns=0 rejected: {err}");
}

#[test]
fn viewer_rejects_empty_tempo_list() {
    let chart_json = r#"{
      "format_version": 1,
      "time_base": 480,
      "start_offset_time": 0.0,
      "page_list": [{"start_tick": 0, "end_tick": 960, "scan_line_direction": 1}],
      "tempo_list": [],
      "event_order_list": [],
      "note_list": []
    }"#;
    let chart: CylheimChart = serde_json::from_str(chart_json).unwrap();
    let err = convert_chart_to_svg(chart, 1, false, None).unwrap_err();
    assert!(
        err.to_string().contains("chart has no tempos"),
        "unexpected error: {err}"
    );
    println!("empty tempo rejected: {err}");
}

#[test]
fn viewer_rejects_invalid_page_ticks() {
    let chart_json = r#"{
      "format_version": 1,
      "time_base": 480,
      "start_offset_time": 0.0,
      "page_list": [{"start_tick": 960, "end_tick": 960, "scan_line_direction": 1}],
      "tempo_list": [{"tick": 0, "value": 1200000}],
      "event_order_list": [],
      "note_list": []
    }"#;
    let chart: CylheimChart = serde_json::from_str(chart_json).unwrap();
    let err = convert_chart_to_svg(chart, 1, false, None).unwrap_err();
    assert!(
        err.to_string().contains("page end_tick must be greater than start_tick"),
        "unexpected error: {err}"
    );
    println!("invalid page rejected: {err}");
}
