use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;
use std::time::Instant;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use vinter_trace2img::{
    HeuristicCrashImageGenerator, LineGranularity, MemoryImage, MemoryImageMmap, MemoryReplayer,
    X86PersistentMemory,
};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Process a trace and write the resulting PMEM contents to a file.
    ProcessTrace {
        /// size of PMEM area
        #[clap(long)]
        pmem_len: usize,
        /// include unpersisted lines in output
        #[clap(long)]
        include_unpersisted: bool,
        /// trace file to process (from vinter_trace)
        #[clap(parse(from_os_str))]
        trace: PathBuf,
        /// output file for PMEM contents
        #[clap(parse(from_os_str))]
        output: PathBuf,
    },

    /// Analyze a program based on a VM definition YAML file.
    Analyze {
        /// Path to VM definition YAML
        #[clap(parse(from_os_str))]
        vm_config: PathBuf,
        /// Path to test definition YAML
        #[clap(parse(from_os_str))]
        test_config: PathBuf,
        /// Path to output directory. Default "."
        #[clap(long, parse(from_os_str))]
        output_dir: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ProcessTrace {
            pmem_len,
            include_unpersisted,
            trace,
            output,
        } => {
            let image = MemoryImageMmap::new(pmem_len)?;
            let mem = X86PersistentMemory::new(image, LineGranularity::Word)?;
            let mut replayer = MemoryReplayer::new(mem);
            let f = File::open(trace).context("could not open trace file")?;
            let mut reader = BufReader::new(f);
            for entry in replayer.process_trace(&mut reader) {
                entry?;
            }
            let mut mem = replayer.mem.borrow_mut();
            mem.print_unpersisted();
            if include_unpersisted {
                mem.persist_unpersisted();
            }
            let mut out = File::create(output).context("could not create output file")?;
            out.write(mem.memory_content())
                .context("could not write output file")?;
        }
        Commands::Analyze {
            vm_config,
            test_config,
            output_dir,
        } => {
            let mut gen = HeuristicCrashImageGenerator::new(
                vm_config,
                test_config,
                output_dir.unwrap_or(PathBuf::from(".")),
            )?;
            println!("Tracing command...");
            let t1 = Instant::now();
            gen.trace_pre_failure()
                .context("pre-failure tracing failed")?;
            println!("Pre-failure trace finished. Replaying trace...");
            let t2 = Instant::now();
            let fences_with_writes = gen.replay().context("replay failed")?;
            println!(
                "Replay finished. {} fences with writes, {} crash images",
                fences_with_writes,
                gen.crash_images.len()
            );
            let t3 = Instant::now();
            println!("Extracing semantic states...");
            gen.extract_semantic_states()
                .context("semantic state extraction failed")?;
            println!(
                "State extraction finished. {} unique states",
                gen.semantic_states.len()
            );
            let t4 = Instant::now();
            let d1 = t2.duration_since(t1).as_millis();
            let d2 = t3.duration_since(t2).as_millis();
            let d3 = t4.duration_since(t3).as_millis();

            println!("benchout: {} {} {} {}", d1, d2, d3, gen.crash_images.len());
        }
    }
    Ok(())
}
