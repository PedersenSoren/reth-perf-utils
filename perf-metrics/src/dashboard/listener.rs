//! [DashboardListener] is used to display various metrics.
use std::{
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};

#[cfg(any(
    feature = "enable_opcode_metrics",
    feature = "enable_cache_record",
    feature = "enable_execution_duration_record"
))]
use super::commons::*;

use crate::metrics::metric::MetricEvent;
use tokio::sync::mpsc::UnboundedReceiver;

#[cfg(feature = "enable_tps_gas_record")]
use super::tps_gas::TpsAndGasDisplayer;

/// Struct responsible for listening to and displaying various metric events.
#[derive(Debug)]
pub struct DashboardListener {
    events_rx: UnboundedReceiver<MetricEvent>,

    #[cfg(feature = "enable_tps_gas_record")]
    tps_gas_displayer: TpsAndGasDisplayer,
}

impl DashboardListener {
    /// Creates a new [DashboardListener] with the provided receiver of [MetricEvent].
    ///
    /// # Arguments
    ///
    /// * `events_rx` - A receiver to get metric events.
    pub fn new(events_rx: UnboundedReceiver<MetricEvent>) -> Self {
        Self {
            events_rx,

            #[cfg(feature = "enable_tps_gas_record")]
            tps_gas_displayer: TpsAndGasDisplayer::default(),
        }
    }

    /// Handles a metric event based on its type.
    ///
    /// # Arguments
    ///
    /// * `event` - The metric event to handle.
    fn handle_event(&mut self, event: MetricEvent) {
        match event {
            #[cfg(feature = "enable_execution_duration_record")]
            MetricEvent::ExecutionStageTime { block_number, record } => {
                record.print(block_number);
            }
            #[cfg(feature = "enable_tps_gas_record")]
            MetricEvent::BlockTpsAndGas { block_number, record } => {
                self.tps_gas_displayer.print(block_number, record);
            }
            #[cfg(feature = "enable_opcode_metrics")]
            MetricEvent::OpcodeInfo { block_number, record } => {
                record.print(block_number);
            }
            #[cfg(feature = "enable_cache_record")]
            MetricEvent::CacheDbInfo { block_number, size, record } => {
                super::cache::print_state_size(block_number, size);
                record.print(block_number);
            }
        }
    }
}

impl Future for DashboardListener {
    type Output = ();

    /// Polls the `DashboardListener` for new metric events and processes them.
    ///
    /// # Arguments
    ///
    /// * `cx` - The context in which the function is called.
    ///
    /// # Returns
    ///
    /// * `Poll<Self::Output>` - Indicates whether the future is ready or not.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        // Continuously poll for events and handle them
        loop {
            // Poll the receiver for a new event
            match ready!(this.events_rx.poll_recv(cx)) {
                Some(event) => this.handle_event(event),
                None => {
                    // Receiver has been closed, future is done
                    return Poll::Ready(());
                }
            }
        }
    }
}
