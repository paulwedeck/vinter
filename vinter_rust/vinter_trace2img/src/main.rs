use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

use tokio::sync::{Semaphore, OwnedSemaphorePermit};
use futures_executor::{ThreadPool, ThreadPoolBuilder};
extern crate alloc;
use alloc::sync::Arc;

use std::sync::mpsc::channel;


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
        test_config: Vec<PathBuf>,
        /// Path to output directory. Default "."
        #[clap(long, parse(from_os_str))]
        output_dir: Option<PathBuf>,
        #[clap(short,long)]
        threads : Option<usize>,
        #[clap(short,long)]
        cov_dir : Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
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
            threads,
            cov_dir,
        } => {
            let realthreads = threads.unwrap_or_else(|| {num_cpus::get()});
            let real_out_dir = output_dir.unwrap_or(PathBuf::from("."));
            let pool = ThreadPoolBuilder::new().pool_size(realthreads).create().unwrap();
            let poolguard = Arc::new(Semaphore::new(realthreads));
            let (tx, rx) = channel();

            let shared_cov_dir = cov_dir;

            for single_test in &test_config {
                let txc = tx.clone();
                let vm_config_cpy = vm_config.clone();
                let single_test_cpy = single_test.clone();
                let real_out_dir_cpy = real_out_dir.clone();
                let shared_cov_dir_cpy = shared_cov_dir.clone();
                let localguard = poolguard.clone();
                let localpool = pool.clone();
                let permit = poolguard.clone().acquire_owned().await.unwrap();

                pool.spawn_ok(async move {
                  let result = run_test_combined(vm_config_cpy, single_test_cpy, real_out_dir_cpy, shared_cov_dir_cpy, localpool, localguard, permit).await;
                  txc.send(result).expect("send failed!");
               });
            }

            for real_gen in rx.iter().take(test_config.len()) {
                real_gen?.merge_cov_files()?;
            }
        }
    }
    Ok(())
}

async fn run_test_combined(vm_config : PathBuf,
                           test_config : PathBuf,
                           output_dir : PathBuf,
                           cov_dir : Option<PathBuf>,
                           pool : ThreadPool,
                           poolguard : Arc<Semaphore>,
                           permit : OwnedSemaphorePermit) -> Result<HeuristicCrashImageGenerator> {
  let result = run_test1(vm_config, test_config, output_dir, cov_dir);
  drop(permit);
  run_test2(result, &pool, &poolguard).await
}

fn run_test1(vm_config : PathBuf, test_config : PathBuf, output_dir : PathBuf, cov_dir : Option<PathBuf>) -> Result<(HeuristicCrashImageGenerator, PathBuf)> {
    println!("Running test {} on vm {}", test_config.to_str().unwrap(), vm_config.to_str().unwrap());

    let mut gen = HeuristicCrashImageGenerator::new(
        vm_config.clone(),
        test_config.clone(),
        output_dir,
        cov_dir
    )?;
    println!("Tracing command...");
    gen.trace_pre_failure()
        .context("pre-failure tracing failed")?;
    println!("Pre-failure trace finished. Replaying trace...");
    let fences_with_writes = gen.replay().context("replay failed")?;
    println!(
        "Replay finished. {} fences with writes, {} crash images",
        fences_with_writes,
        gen.crash_images.len()
    );
    Ok((gen, test_config))
}

async fn run_test2(gen : Result<(HeuristicCrashImageGenerator, PathBuf)>, pool : &ThreadPool, poolguard : &Arc<Semaphore>) -> Result<HeuristicCrashImageGenerator> {
    let (mut real_gen, cfg) = gen?;

    println!("{}: Extracting semantic states...", cfg.to_str().unwrap());
    real_gen.extract_semantic_states(pool, poolguard).await
        .context("semantic state extraction failed")?;
    println!(
        "State extraction finished. {} unique states",
        real_gen.semantic_states.len()
    );
    Ok(real_gen)
}
