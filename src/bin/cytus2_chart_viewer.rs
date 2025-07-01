use clap::Parser;
use cyl_package_tool::cylheim_tools::chart_viewer::convert_chart_to_svg;
use cyl_package_tool::cylheim_tools::CylheimChart;
use dialoguer::Confirm;
use dialoguer::Input;
use rfd::FileDialog;
use serde_json;
use std::{fs, path::PathBuf, time::Instant};

/// 谱面预览命令行工具
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: Option<PathBuf>,
    #[arg(short, long)]
    output: Option<PathBuf>,
    #[arg(short, long, default_value_t = 4)]
    columns: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let input_path = match args.input {
        Some(path) => path,
        None => {
            println!("File input:");
            let selected_file = FileDialog::new()
                .add_filter("JSON Files", &["json"])
                .pick_file();
            match selected_file {
                Some(file) => file,
                None => {
                    return Ok(());
                }
            }
        }
    };
    let input_file_stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output")
        .to_string();
    let show_ghost_note = Confirm::new()
        .with_prompt("Show ghost note?")
        .default(true)
        .interact()?;
    let column = Input::new().with_prompt("Columns").default(4).interact()?;
    let output_dir = match args.output {
        Some(path) => path,
        None => {
            let input_parent: PathBuf = input_path
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| std::env::current_dir().expect("cannot get current directory"));
            let mut input_parent_dir = input_parent;
            input_parent_dir.push("preview_output");
            fs::create_dir_all(&input_parent_dir)?;
            println!("Default path: {:?}", input_parent_dir);
            input_parent_dir
        }
    };
    fs::create_dir_all(&output_dir)?;
    let f = fs::read_to_string(&input_path)?;
    let cylchart: CylheimChart = serde_json::from_str(&f)?;
    let start = Instant::now();
    let svg = convert_chart_to_svg(cylchart, column, show_ghost_note, None)?;
    let build_duration = start.elapsed();
    println!("convert_chart_to_svg time elapsed: {:?}", build_duration);
    let svg_path = output_dir.join(format!("{}.svg", input_file_stem));
    fs::write(&svg_path, svg.to_string())?;
    println!("save to {:?}", output_dir);
    Ok(())
}
