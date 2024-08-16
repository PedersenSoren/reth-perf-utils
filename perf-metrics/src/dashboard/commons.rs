#[cfg(any(feature = "enable_opcode_metrics", feature = "enable_cache_record"))]
use revm_utils::metrics::types::TimeDistributionStats;

/// This trait is used to support the display of metric records.
pub trait Print {
    fn print_title(&self) {}
    fn print_content(&self) {}
    
    /// Default implementation of print which calls print_title and print_content.
    fn print(&self, _block_number: u64) {
        println!();
        self.print_title();
        self.print_content();
        println!();
    }
}

#[cfg(any(feature = "enable_opcode_metrics", feature = "enable_cache_record"))]
const COL_WIDTH: usize = 15;

#[cfg(any(feature = "enable_opcode_metrics", feature = "enable_cache_record"))]
impl Print for TimeDistributionStats {
    fn print_content(&self) {
        let total_cnt: u64 = self.us_percentile.iter().map(|&v| v).sum();
        let mut cumulative_pct = 0.0;

        println!(
            "{:<COL_WIDTH$} {:>COL_WIDTH$} {:>COL_WIDTH$}",
            "Time (ns)", "Count (%)", "Cuml. (%)"
        );

        // Print time distribution in nanoseconds
        for index in 0..self.span_in_ns {
            let pct = self.ns_percentile[index] as f64 / total_cnt as f64;
            cumulative_pct += pct;
            println!(
                "{:<COL_WIDTH$} {:>COL_WIDTH$.3} {:>COL_WIDTH$.3}",
                (index + 1) * 100,
                pct * 100.0,
                cumulative_pct * 100.0
            );
        }

        // Print time distribution in microseconds
        let ns_span_in_us = (self.span_in_ns * 100) / 1000;
        for index in ns_span_in_us..self.span_in_us {
            let pct = self.us_percentile[index] as f64 / total_cnt as f64;
            cumulative_pct += pct;
            println!(
                "{:<COL_WIDTH$} {:>COL_WIDTH$.3} {:>COL_WIDTH$.3}",
                (index + 1) * 1000,
                pct * 100.0,
                cumulative_pct * 100.0
            );
        }

        println!();
        println!("========>:");
        println!("Total count: {:?}", total_cnt);
        println!("Span in ns: {:?}", self.span_in_ns);
        println!("Span in us: {:?}", self.span_in_us);
        println!("Percentiles in ns: {:?}", self.ns_percentile);
        println!("Percentiles in us: {:?}", self.us_percentile);
        println!();
    }
}

#[cfg(any(
    feature = "enable_opcode_metrics",
    feature = "enable_cache_record",
    feature = "enable_execution_duration_record"
))]
pub(super) fn cycles_as_secs(cycles: u64) -> f64 {
    revm_utils::time_utils::convert_cycles_to_duration(cycles).as_secs_f64()
}

#[cfg(feature = "enable_execution_duration_record")]
pub(super) fn convert_bytes_to_mega(size: usize) -> f64 {
    size as f64 / (1024.0 * 1024.0)
}
