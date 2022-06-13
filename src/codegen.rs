use clap::Parser;

use aqlgen_renderer::{generate_from_path, Config};

/// Schema generator for async-graphql 4.x
#[derive(clap_derive::Parser)]
#[clap(author, version, about, long_about = None)]
struct Opts {
    _dummy: Option<String>,
    /// Path to the schema file
    #[clap(short, long, required = true)]
    schema: String,
    /// Path to the output folder
    #[clap(short, long, required = true)]
    output: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let path = opts.schema;
    let config = Config {
        output_bnase_path: opts.output,
    };
    generate_from_path(&path, &config);
}
