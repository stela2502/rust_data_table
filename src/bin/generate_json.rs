// generate_json.rs

use std::collections::HashSet;
use std::path::PathBuf;
use clap::{Arg, Command};
use rust_data_table::SurvivalData;

fn main() -> anyhow::Result<()> {
    let matches = Command::new("generate_json")
        .version("0.1.0")
        .author("Stefan Lang <you@example.com>")
        .about(
            "Convert a Seurat or AnnData 'meta.tsv' file into a JSON description \
             file that encodes numeric and categorical data for use in downstream tools (e.g., VR apps)."
        )
        .after_help(
r"EXAMPLES:
  # prepare the factors.json file for tsv: 
  generate_json pbmc3k/meta.tsv

  # Specify comma as delimiter (for CSV files)
  generate_json data/meta.csv --delimiter ,

  # Mark specific columns as categorical (numeric but treated as factors)
  generate_json data/meta.tsv --categorical cluster,sex,condition

NOTES:
  • The generated JSON file contains factor information 
    for the not numerical and categorical metadata,
    similar to R 'factor' information.
  • Afterwards you can modify the json file to 
    define exactly the factor setup you want/need."
        )
        .arg(
            Arg::new("input")
                .help("Input metadata file (TSV or CSV)")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("delimiter")
                .short('d')
                .long("delimiter")
                .help("Field delimiter (default: tab '\\t')")
                .default_value("\\t"),
        )
        .arg(
            Arg::new("categorical")
                .short('c')
                .long("categorical")
                .help("Comma-separated list of categorical column names")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("factors_file")
                .short('f')
                .long("factors-file")
                .help("Optional path for factor definitions output")
                .required(false)
                .num_args(1),
        )
        .get_matches();

    let input_path = PathBuf::from(matches.get_one::<String>("input").unwrap());
    let delimiter_str = matches.get_one::<String>("delimiter").unwrap();
    let delimiter = match delimiter_str.as_str() {
        "\\t" | "tab" => b'\t',
        "," | "comma" => b',',
        other if other.len() == 1 => other.as_bytes()[0],
        _ => anyhow::bail!("Invalid delimiter '{}'", delimiter_str),
    };

    // Categorical columns (optional)
    let categorical_cols: HashSet<String> = matches
        .get_one::<String>("categorical")
        .map(|s| s.split(',').map(|v| v.trim().to_string()).collect())
        .unwrap_or_default();

    // Factors file (optional)
    let factors_file = matches
        .get_one::<String>("factors_file")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("factors.tsv"));

    if factors_file.exists(){
        println!("factors file already exists - no need to run this.");
        return Ok(());
    }
    println!("📄 Input file: {:?}", input_path);
    println!("📘 Factors file: {:?}", factors_file);
    println!("Categorical cols: {:?}", categorical_cols);

    match SurvivalData::from_file(
        &input_path,
        delimiter,
        categorical_cols,
        &factors_file
    ) {
        Ok(_) => println!("This is trange - this should actually fail here!"),
        Err(_) => (),
    }

    println!("✅ JSON successfully written to {:?}", factors_file);
    Ok(())
}