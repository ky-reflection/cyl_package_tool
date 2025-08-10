use clap::Parser;
use cyl_package_tool::cylheim_tools::chart_viewer::convert_chart_to_svg;
use cyl_package_tool::cylheim_tools::cyl_chart::load_chart_from_backup;
use cyl_package_tool::cylheim_tools::CylheimChart;
use resvg::render;
use serde_json;
use std::{fs, path::PathBuf, time::Instant};
use tiny_skia::Pixmap;
use usvg::Options;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: Option<PathBuf>,
    #[arg(short, long)]
    output: Option<PathBuf>,
    #[arg(short = 'n', long = "name", help = "指定输出文件的文件名，不包含后缀")]
    filename: Option<String>,
    #[arg(short, long, default_value_t = 4)]
    columns: usize,
    #[arg(short = 'g', long = "ghost", default_value_t = true)]
    show_ghost_note: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let input_path = match args.input {
        Some(path) => path,
        None => {
            eprintln!("Error: Input file is required.");
            return Err("Input file not provided".into());
        }
    };
    let show_ghost_note = args.show_ghost_note;
    let column = args.columns;
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

    if input_path.is_dir() {
        for entry in fs::read_dir(input_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Err(e) =
                    process_file(&path, &output_dir, column, show_ghost_note, &args.filename)
                {
                    eprintln!("Error processing file {:?}: {}", path, e);
                }
            }
        }
    } else if input_path.is_file() {
        if let Err(e) = process_file(
            &input_path,
            &output_dir,
            column,
            show_ghost_note,
            &args.filename,
        ) {
            eprintln!("Error processing file {:?}: {}", input_path, e);
        }
    }

    Ok(())
}

fn process_file(
    input_path: &PathBuf,
    output_dir: &PathBuf,
    column: usize,
    show_ghost_note: bool,
    filename: &Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let input_file_stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output")
        .to_string();
    let f = fs::read_to_string(input_path)?;
    let cylchart: CylheimChart = match serde_json::from_str(&f) {
        Ok(chart) => chart,
        Err(_) => load_chart_from_backup(&f)?,
    };
    let start = Instant::now();
    let svg = convert_chart_to_svg(cylchart, column, show_ghost_note, None)?;
    let build_duration = start.elapsed();
    println!("convert_chart_to_svg time elapsed: {:?}", build_duration);
    let mut opt = Options::default();
    opt.fontdb_mut().load_system_fonts();
    let tree = usvg::Tree::from_data(svg.to_string().as_bytes(), &opt)?;
    let size = tree.size().to_int_size();
    let mut pixmap = Pixmap::new(size.width(), size.height()).ok_or("无法创建 pixmap")?;
    render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
    let file_name = filename.clone().unwrap_or(input_file_stem);
    let png_path = output_dir.join(format!("{}.png", file_name));
    let svg_path = output_dir.join(format!("{}.svg", file_name));
    std::fs::write(&svg_path, svg.to_string())?;
    println!("save to {:?}", svg_path);
    pixmap.save_png(&png_path)?;
    println!("save to {:?}", png_path);
    Ok(())
}
