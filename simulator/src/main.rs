#![feature(duration_millis_float)]

use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use color_eyre::eyre::{ErrReport, Result};
use metrics::{describe_histogram, gauge, Unit};
use metrics_exporter_tcp::TcpBuilder;
use mugraph_simulator::{Config, Simulation};
use tracing::{error, info};

fn main() -> Result<()> {
    color_eyre::install()?;
    let metric_address = "0.0.0.0:9999";
    TcpBuilder::new()
        .listen_address(metric_address.parse::<SocketAddr>()?)
        .install()?;

    let cores = core_affinity::get_core_ids().unwrap();
    let should_continue = Arc::new(AtomicBool::new(true));
    let config = Config::default();

    describe_histogram!(
        "mugraph.database.len.time_taken",
        Unit::Milliseconds,
        "database time call #len"
    );
    describe_histogram!(
        "mugraph.database.read.time_taken",
        Unit::Milliseconds,
        "database time call #read"
    );
    describe_histogram!(
        "mugraph.database.set_len.time_taken",
        Unit::Milliseconds,
        "database time call #set_len"
    );
    describe_histogram!(
        "mugraph.database.sync_data.time_taken",
        Unit::Milliseconds,
        "database time call #sync_data"
    );
    describe_histogram!(
        "mugraph.database.write.time_taken",
        Unit::Milliseconds,
        "database time call #write"
    );
    describe_histogram!(
        "mugraph.simulator.tick.time_taken",
        Unit::Milliseconds,
        "how long it took to run a simulation tick"
    );
    describe_histogram!(
        "mugraph.simulator.state.next.time_taken",
        Unit::Milliseconds,
        "how long it took to generate the next action in the simulation"
    );
    describe_histogram!(
        "mugraph.simulator.state.next.split.time_taken",
        Unit::Milliseconds,
        "how long it took to generate the next split action in the simulation"
    );
    describe_histogram!(
        "mugraph.simulator.state.next.join.time_taken",
        Unit::Milliseconds,
        "how long it took to generate the next join action in the simulation"
    );
    describe_histogram!(
        "mugraph.simulator.delegate.transaction_v0",
        Unit::Milliseconds,
        "How long it took to get a server response"
    );

    // Force interface to run on the first possible core
    core_affinity::set_for_current(cores[0]);

    for (i, core) in cores.into_iter().enumerate().skip(1).take(config.threads) {
        let sc = should_continue.clone();
        let mut sim = Simulation::new(core.id as u32)?;

        thread::spawn(move || {
            core_affinity::set_for_current(core);

            info!("Starting simulation on core {i}.");

            let mut round = 0;

            while sc.load(Ordering::Relaxed) {
                gauge!(
                    "mugraph.simulator.current_round",
                    "core_id" => core.id.to_string()
                )
                .set(round as f64);

                sim.tick(round)?;
                round += 1;
            }

            Ok::<_, ErrReport>(())
        });
    }

    let sc = should_continue.clone();
    ctrlc::set_handler(move || {
        sc.swap(false, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    match metrics_observer::main(metric_address, should_continue.clone()) {
        Ok(_) => {
            info!("Observer finished.");
        }
        Err(e) => {
            error!(msg = %e, "Observer failed because of error");
        }
    }

    should_continue.swap(false, Ordering::Relaxed);
    metrics_observer::restore_terminal()?;

    Ok(())
}
