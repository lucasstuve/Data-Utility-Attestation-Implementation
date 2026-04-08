use csv::Writer;
use csv::WriterBuilder;
use std::error::Error;
use std::fs::OpenOptions;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(serde::Serialize)]
pub struct Row<'a> {
    pub n_events: u32,
    pub dsl_case: &'a str,
    pub input_bytes: u64,
    pub total_cycles: u32,
    pub user_cycles: u32,
    pub segments: u32,
    pub time_env_s: u32,
    pub time_exec_s: u32,
    pub time_prov_s: u32,
    pub time_veri_ms: u64,
}

pub fn next_counter() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

impl<'a> Row<'a> {
    pub fn new() -> Self {
        Row {
            n_events: (0 as u32),
            dsl_case: "",
            input_bytes: (0 as u64),
            total_cycles: (0 as u32),
            user_cycles: (0 as u32),
            segments: (0 as u32),
            time_env_s: (0 as u32),
            time_exec_s: (0 as u32),
            time_prov_s: (0 as u32),
            time_veri_ms: (0 as u64),
        }
    }

    pub fn set_dsl_query(&mut self, query: &'a str) {
        self.dsl_case = query;
    }
    pub fn set_input_bytes(&mut self, n_bytes: u64) {
        self.input_bytes = n_bytes;
    }
    pub fn set_total_cycles(&mut self, n_cycles: u32) {
        self.total_cycles = n_cycles;
    }
    pub fn set_user_cycles(&mut self, n_cycles: u32) {
        self.user_cycles = n_cycles;
    }
    pub fn set_env_time(&mut self, time_s: u32) {
        self.time_env_s = time_s;
    }
    pub fn set_exec_time(&mut self, time_s: u32) {
        self.time_exec_s = time_s;
    }
    pub fn set_prove_time(&mut self, time_s: u32) {
        self.time_prov_s = time_s;
    }
    pub fn set_veri_time(&mut self, time_ms: u64) {
        self.time_veri_ms = time_ms;
    }

    pub fn set_segments(&mut self, n_segments: u32) {
        self.segments = n_segments;
    }
    pub fn set_number_events(&mut self, n_events: u32) {
        self.n_events = n_events;
    }
}

pub fn write_results_csv(rows: Vec<Row>, file_name: &str) -> Result<(), Box<dyn Error>> {
    let base = "benchmarks/benchmark_results";
    let full_path = Path::new(base).join(file_name);

    let file_created = full_path.exists() && std::fs::metadata(&full_path)?.len() > 0;

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&full_path)?;

    let mut wtr = WriterBuilder::new()
        .has_headers(!file_created)
        .from_writer(file);

    for row in rows {
        wtr.serialize(row)?;
    }
    wtr.flush()?;
    Ok(())
}
