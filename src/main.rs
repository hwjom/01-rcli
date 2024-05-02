// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
// use anyhow::Ok;
use rcli::{process_csv, process_genpass, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        }
        SubCommand::Genpass(opts) => {
           process_genpass(opts.length, opts.upper, opts.lower, opts.number, opts.symbol)?;
        }
    }
    Ok(())
}
