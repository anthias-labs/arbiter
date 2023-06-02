#![warn(missing_docs)]
#![warn(unsafe_code)]
//! ## module for the environment
//!
//! An abstraction on the EVM, to be used in simulations.
pub mod contract;

use crossbeam_channel::{unbounded, Receiver, Sender};
use ethers::abi::Token;
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{ExecutionResult, Log, TxEnv, U256},
    EVM,
};
use std::{thread, pin::Pin};
use tokio::sync::broadcast;
use futures::{task::{Context, Poll}, Stream};
use futures::stream::StreamExt;

use crate::agent::{SimulationEventFilter, AgentError, filter_events};

/// The simulation environment that houses the execution environment and event logs.
/// # Fields
/// * `evm` - The EVM that is used for the simulation.
/// * `event_senders` - The senders on the event channel that is used to send events to the agents and simulation manager.
pub struct SimulationEnvironment {
    /// The EVM that is used for the simulation.
    pub(crate) evm: EVM<CacheDB<EmptyDB>>,
    /// The sender on the event channel that is used to send events to the agents and simulation manager.
    pub(crate) event_broadcaster: broadcast::Sender<Vec<Log>>,
    /// The receiver of txs from agents.
    /// Bundles with a sender to send the execution result back to the agent.
    pub(crate) transaction_channel: (
        Sender<(TxEnv, Sender<ExecutionResult>)>,
        Receiver<(TxEnv, Sender<ExecutionResult>)>,
    ),
}

impl SimulationEnvironment {
    pub(crate) fn new() -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.env.cfg.limit_contract_code_size = Some(0x100000); // This is a large contract size limit, beware!
        evm.env.block.gas_limit = U256::MAX;
        evm.database(db);
        let transaction_channel = unbounded::<(TxEnv, Sender<ExecutionResult>)>();
        Self {
            evm,
            event_broadcaster: broadcast::channel(16).0,
            transaction_channel,
        }
    }

    pub(crate) fn run(&self) {
        let tx_receiver = self.transaction_channel.1.clone();
        let mut evm = self.evm.clone();
        let event_broadcaster = self.event_broadcaster.clone();
        thread::spawn(move || {
            while let Ok((tx, sender)) = tx_receiver.recv() {
                // Execute the transaction, echo the logs to all agents, and report the execution result to the agent who made the call.
                let execution_result = execute(&mut evm, tx);
                event_broadcaster.send(execution_result.logs()).unwrap(); // TODO: We can avoid an unwrap here and gracefully handle this error.
                sender.send(execution_result).unwrap();
            }
        });
    }
}

/// Execute a transaction in the execution environment.
/// # Arguments
/// * `tx` - The transaction environment that is used to execute the transaction.
/// # Returns
/// * `ExecutionResult` - The execution result of the transaction.
fn execute(evm: &mut EVM<CacheDB<EmptyDB>>, tx: TxEnv) -> ExecutionResult {
    evm.env.tx = tx;

    let execution_result = match evm.transact_commit() {
        Ok(val) => val,
        // URGENT: change this to a custom error
        Err(_) => panic!("failed"),
    };

    execution_result
}


struct EventStream {
    receiver: broadcast::Receiver<Vec<Log>>,
    filters: Vec<SimulationEventFilter>,
    decoder: fn(Vec<u8>, usize) -> Result<Vec<Token>, AgentError>,
}

impl EventStream {
    async fn next(&mut self) -> Option<Result<Vec<Token>, AgentError>> {
        let event_filters = self.filters.clone();
        let decoder = self.decoder;

        self.receiver.recv().await.ok().map(|logs| {
            let filtered_logs = filter_events(event_filters.clone(), logs);
            if filtered_logs.is_empty() {
                return Ok(vec![]);
            }
            let data = filtered_logs[0].data.clone().into_iter().collect();
            decoder(data, 0)
        })
    }

    fn into_stream(self) -> impl Stream<Item = Result<Vec<Token>, AgentError>> + '_ {
        futures::stream::unfold(self, |mut state| async {
            let item = state.next().await;
            Some((item, state))
        })
    }
}