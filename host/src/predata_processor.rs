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
