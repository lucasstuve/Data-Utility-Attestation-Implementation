use benchmarks::data_generator::{
    generate_data_by_frequency, generate_test_data, generate_test_data_large,
};
fn main() {
    /*
    generate_test_data(1, "test-data-1KB.json", 0.0, 0.0, 0.0);
    generate_test_data(10, "test-data-10KB.json", 0.0, 0.0, 0.0);
    generate_test_data(100, "test-data-100KB.json", 0.0, 0.0, 0.0);
    generate_test_data(1000, "test-data-1000KB.json", 0.0, 0.0, 0.0);*/

    /*

    generate_data_by_frequency(0.05, "test-data-5-percent.json", 100);
    generate_data_by_frequency(0.10, "test-data-10-percent.json", 100);
    generate_data_by_frequency(0.15, "test-data-15-percent.json", 100);
    generate_data_by_frequency(0.20, "test-data-20-percent.json", 100);
    generate_data_by_frequency(0.25, "test-data-25-percent.json", 100);
    generate_data_by_frequency(0.30, "test-data-30-percent.json", 100);
    generate_data_by_frequency(0.35, "test-data-35-percent.json", 100);
    generate_data_by_frequency(0.40, "test-data-40-percent.json", 100);
    generate_data_by_frequency(0.45, "test-data-45-percent.json", 100);
    generate_data_by_frequency(0.50, "test-data-50-percent.json", 100);
    generate_data_by_frequency(0.55, "test-data-55-percent.json", 100);
    generate_data_by_frequency(0.60, "test-data-60-percent.json", 100);
    generate_data_by_frequency(0.65, "test-data-65-percent.json", 100);
    generate_data_by_frequency(0.70, "test-data-70-percent.json", 100);
    generate_data_by_frequency(0.75, "test-data-75-percent.json", 100);
    generate_data_by_frequency(0.80, "test-data-80-percent.json", 100);
    generate_data_by_frequency(0.85, "test-data-85-percent.json", 100);
    generate_data_by_frequency(0.90, "test-data-90-percent.json", 100);
    generate_data_by_frequency(0.95, "test-data-95-percent.json", 100);
    generate_data_by_frequency(1.0, "test-data-100-percent.json", 100); */

    generate_test_data_large(100 * 1024, "test-data-100MB.json", 0.0, 0.0, 0.0);
}
