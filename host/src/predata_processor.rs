//! Builds the `index_list` the host later passes into the zkVM guest: a
//! list of `(start, end)` byte ranges, one per event object, computed by
//! scanning for the batch's `"data":[`/`"Data":[` array and brace-matching
//! each `{...}` inside it. This is a lightweight substitute for a full
//! JSON parse, and the guest re-validates the result independently (see
//! `dnf_core::index_list_validator::check_list`) since the host is
//! untrusted from the guest's perspective.

/// Finds the byte offsets of every top-level event object inside the
/// batch's data array. Accepts either `"data":[` or `"Data":[` as the
/// array's key casing; falls back to scanning from the start of `file` if
/// neither is found.
pub fn create_events_indexes(file: &str) -> Vec<(u32, u32)> {
    let data_list_indicator = [r#""Data":["#, r#""data":["#]; // \"ata\":[";

    let meta_base = data_list_indicator
        .iter()
        .find_map(|indicator| file.find(indicator).map(|p| p + indicator.len()))
        .unwrap_or(0);

    let data_bytes = &file[meta_base..];

    let mut starts = data_bytes
        .as_bytes()
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| (b == b'{').then_some((meta_base + i) as u32));

    let mut ends = data_bytes
        .as_bytes()
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| (b == b'}').then_some((meta_base + i + 1) as u32));

    starts.zip(&mut ends).collect()
}
