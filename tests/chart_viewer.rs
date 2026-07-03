use cyl_package_tool::cylheim_tools::{convert_chart_to_svg, svg_to_png, CylheimChart};
use std::fs;

const OUTPUT_DIR: &str = "./tests/output/viewer_cases/";

fn ensure_output_dir() {
    fs::create_dir_all(OUTPUT_DIR).unwrap();
}

fn load_chart(name: &str) -> CylheimChart {
    let path = format!("./tests/resources/{name}");
    let content = fs::read_to_string(&path).unwrap();
    serde_json::from_str(&content).unwrap()
}

fn save_case(name: &str, svg: &svg::Document) {
    let svg_path = format!("{OUTPUT_DIR}{name}.svg");
    let png_path = format!("{OUTPUT_DIR}{name}.png");
    let svg_text = svg.to_string();
    fs::write(&svg_path, &svg_text).unwrap();
    let png = svg_to_png(svg_text.as_bytes(), None).unwrap();
    fs::write(&png_path, png).unwrap();
    println!("saved: {svg_path}");
    println!("saved: {png_path}");
}

#[test]
fn viewer_single_click_note() {
    ensure_output_dir();
    let chart = load_chart("viewer_single_click.json");
    let svg = convert_chart_to_svg(chart, 1, false, None).unwrap();
    let text = svg.to_string();
    assert!(text.contains("<circle"), "click note should render circles");
    assert!(text.contains("\n0\n"), "page index label should be present");
    save_case("single_click", &svg);
}

#[test]
fn viewer_two_page_with_ghost() {
    ensure_output_dir();
    let chart = load_chart("viewer_two_page.json");
    let svg = convert_chart_to_svg(chart, 2, true, None).unwrap();
    let text = svg.to_string();
    assert!(text.contains("opacity=\"0.3\""), "ghost layer should use opacity 0.3");
    assert!(text.contains("\n0\n"));
    assert!(text.contains("\n1\n"));
    save_case("two_page_ghost", &svg);
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
    save_case("drag_chain", &svg);
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
