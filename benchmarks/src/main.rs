use benchmarks::data_generator::generate_test_data;
fn main() {
    generate_test_data(1, "test-data-1KB.json", 0.0, 0.0, 0.0);
    generate_test_data(10, "test-data-10KB.json", 0.0, 0.0, 0.0);
    generate_test_data(100, "test-data-100KB.json", 0.0, 0.0, 0.0);
    generate_test_data(1000, "test-data-1000KB.json", 0.0, 0.0, 0.0);
}
