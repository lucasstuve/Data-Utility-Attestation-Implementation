use chrono::{DateTime, Duration, TimeZone, Utc};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use uuid::Uuid;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn generate_test_data(
    file_size_kb: u64,
    file_name: &str,
    _frequency_filter: f32,
    _frequency_patterns: f32,
    _frequency_session: f32,
) {
    //  let base = "evaluator_program/benchmarks/test_data";

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_data");

    let file_path = path.join(file_name);

    fs::create_dir_all(&path).unwrap();

    println!("manifest dir = {}", env!("CARGO_MANIFEST_DIR"));
    println!("base         = {}", path.display());
    println!("file_path    = {}", file_path.display());

    // let path = Path::new(&base).join(file_name);

    let uuid = Uuid::new_v4().to_string();

    let mut doc = EventDocument {
        vin: "XXXUUDSE4".into(),
        user_id: uuid,
        data: Vec::new(),
    };

    let initial_tmstp: DateTime<Utc> = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
    println!("path = {:?}", &path);

    let mut file = File::create(&file_path).unwrap();

    loop {
        let event = generate_random_event(initial_tmstp);
        doc.data.push(event);

        let json_string = serde_json::to_string(&doc).unwrap();

        let new_size = json_string.len() as u64;

        if new_size > file_size_kb * 1024 {
            doc.data.pop();
            let final_json = serde_json::to_string(&doc).unwrap();
            file.write_all(final_json.as_bytes()).unwrap();
            file.flush().unwrap();
            break;
        }
    }
}

fn generate_random_event(initial_tmstp: DateTime<Utc>) -> Event {
    let mut rng = rand::rng();

    let dice = rng.random_range(0..3_u32);
    let time_stmp_current = initial_tmstp + Duration::seconds(0 + next_counter() as i64);

    let data_field_names = [
        "profiles.targetSOCPercentage",
        "mileage",
        "CHARGE_MODE_SELECTION",
    ];
    let charge_mode = ["SOC_KW_IN", "PHOTO_VOLTAIC", "HYBRID"];
    let mut data_field_name = "";
    let mut value = "".to_owned();

    if dice == 0 {
        data_field_name = data_field_names[0];

        value = format!("{:.2}", rng.random_range(1.0..100.0));
    } else if dice == 1 {
        data_field_name = data_field_names[1];
        let value_str = format!("{:.2}", rng.random_range(100.0..1000.0));
        value = value_str
    } else {
        data_field_name = data_field_names[2];
        value = charge_mode[rng.random_range(0..3)].to_owned();
    }

    let id = Uuid::new_v4();

    Event {
        key: id.to_string(),
        value: value,
        dataFieldName: data_field_name.into(),
        timestampUtc: time_stmp_current
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string(),
    }
}

#[derive(Serialize, Deserialize)]
struct EventDocument {
    vin: String,
    user_id: String,
    data: Vec<Event>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Event {
    key: String,
    dataFieldName: String,
    value: String,
    timestampUtc: String,
}

pub fn next_counter() -> usize {
    COUNTER.fetch_add(60, Ordering::Relaxed)
}

pub fn generate_data_by_frequency(frequency: f32, file_name: &str, file_size_kb: u64) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_data");
    let file_path = path.join(file_name);

    fs::create_dir_all(&path).unwrap();

    let uuid = Uuid::new_v4().to_string();

    let mut doc = EventDocument {
        vin: "XXXUUDSE4".into(),
        user_id: uuid,
        data: Vec::new(),
    };

    let initial_tmstp: DateTime<Utc> = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
    println!("path = {:?}", &path);

    let mut file = File::create(&file_path).unwrap();
    let mut rng = rand::rng();
    let mut counter_fitting: f64 = 0.0;
    let mut counter_not_fitting: f64 = 0.0;

    loop {
        let event: Event = if rng.random_bool(frequency as f64) {
            counter_fitting = counter_fitting + 1.0;
            generate_schema_event()
        } else {
            counter_not_fitting = counter_not_fitting + 1.0;
            generate_not_schema_event()
        };

        doc.data.push(event);

        println!("Number of Elements created ... : {}", doc.data.len());

        let json_string = serde_json::to_string(&doc).unwrap();
        let new_size = json_string.len() as u64;

        if new_size > file_size_kb * 1024 {
            doc.data.pop();
            let final_json = serde_json::to_string(&doc).unwrap();
            file.write_all(final_json.as_bytes()).unwrap();
            file.flush().unwrap();
            println!(
                "Data frequency is: {}",
                (counter_fitting / (counter_fitting + counter_not_fitting))
            );
            break;
        }
    }
}

pub fn generate_schema_event() -> Event {
    // HARD CODED: QUERY='CREATE SCHEMA VehicleData (dataFieldName string, value float); assert ALL VehicleData (dataFieldName == "profiles.targetSOCPercentage" AND value < 50.0 );'
    // Event that fits the SCHEMA defintion and is positively reconised by the filter => Event that is actually processed.
    let id = Uuid::new_v4();
    let initial_tmstp: DateTime<Utc> = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
    let time_stmp_current = initial_tmstp + Duration::seconds(0 + next_counter() as i64);

    let mut rng = rand::rng();
    let value: f32 = rng.random_range(0.0..49.0);
    return Event {
        key: id.to_string(),
        dataFieldName: "profiles.targetSOCPercentage".into(),
        value: value.to_string(),
        timestampUtc: time_stmp_current.to_string(),
    };
}

pub fn generate_not_schema_event() -> Event {
    let mut rng = rand::rng();
    // HARD CODED: QUERY='CREATE SCHEMA VehicleData (dataFieldName string, value float); assert ALL VehicleData (dataFieldName == "profiles.targetSOCPercentage" AND value < 50.0 );'
    // Event that fits the SCHEMA defintion and is positively reconised by the filter => Event that is actually processed.
    let id = Uuid::new_v4();
    let initial_tmstp: DateTime<Utc> = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
    let time_stmp_current = initial_tmstp + Duration::seconds(0 + next_counter() as i64);

    let random_string = rng.random_range(0..3);

    let string_values = ["on", "ok", "off"];

    return Event {
        key: id.to_string(),
        dataFieldName: "engine.status.current".into(),
        value: string_values[random_string].to_string(),
        timestampUtc: time_stmp_current.to_string(),
    };
}

pub fn generate_test_data_large(
    file_size_kb: u64,
    file_name: &str,
    _frequency_filter: f32,
    _frequency_patterns: f32,
    _frequency_session: f32,
) {
    //  let base = "evaluator_program/benchmarks/test_data";

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_data");

    let file_path = path.join(file_name);

    fs::create_dir_all(&path).unwrap();

    println!("manifest dir = {}", env!("CARGO_MANIFEST_DIR"));
    println!("base         = {}", path.display());
    println!("file_path    = {}", file_path.display());

    // let path = Path::new(&base).join(file_name);

    let uuid = Uuid::new_v4().to_string();

    let mut doc = EventDocument {
        vin: "XXXUUDSE4".into(),
        user_id: uuid,
        data: Vec::new(),
    };

    let initial_tmstp: DateTime<Utc> = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
    println!("path = {:?}", &path);

    let mut file = File::create(&file_path).unwrap();

    let mut event_counter: u64 = 0;

    loop {
        let event = generate_random_event(initial_tmstp);
        doc.data.push(event);
        event_counter = event_counter + 1;
        //   let json_string = serde_json::to_string(&doc).unwrap();

        //   let new_size = json_string.len() as u64;

        if event_counter % 1000 == 0 {
            let json_string = serde_json::to_string(&doc).unwrap();
            let new_size = json_string.len() as u64;

            if new_size > file_size_kb * 1024 {
                doc.data.pop();
                let final_json = serde_json::to_string(&doc).unwrap();
                file.write_all(final_json.as_bytes()).unwrap();
                file.flush().unwrap();
                break;
            }
        }
    }
}

#[cfg(test)]

mod gen_test {

    use super::*;

    #[test]
    fn gen_data() {
        generate_test_data(1, "first_data.json", 0.0, 0.0, 0.0);

        assert!(Path::new("benchmarks/test_data/first_data.json").exists());
    }
}
