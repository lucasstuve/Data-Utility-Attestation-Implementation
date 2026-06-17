use benchmarks::data_generator::{
    generate_data_by_frequency, generate_test_data, generate_test_data_large,
};
fn main() {
    /*
    generate_test_data(1, "test-data-1KB.json", 0.0, 0.0, 0.0);
    generate_test_data(10, "test-data-10KB.json", 0.0, 0.0, 0.0);
    generate_test_data(100, "test-data-100KB.json", 0.0, 0.0, 0.0);
    generate_test_data(1000, "test-data-1000KB.json", 0.0, 0.0, 0.0);*/

    

    generate_data_by_frequency(0.050, "test-data-5-percent.json", 100);
    generate_data_by_frequency(0.100, "test-data-10-percent.json", 100);
    generate_data_by_frequency(0.150, "test-data-15-percent.json", 100);
    generate_data_by_frequency(0.200, "test-data-20-percent.json", 100);
    generate_data_by_frequency(0.250, "test-data-25-percent.json", 100);
    generate_data_by_frequency(0.300, "test-data-30-percent.json", 100);
    generate_data_by_frequency(0.350, "test-data-35-percent.json", 100);
    generate_data_by_frequency(0.400, "test-data-40-percent.json", 100);
    generate_data_by_frequency(0.450, "test-data-45-percent.json", 100);
    generate_data_by_frequency(0.500, "test-data-50-percent.json", 100);
    generate_data_by_frequency(0.550, "test-data-55-percent.json", 100);
    generate_data_by_frequency(0.600, "test-data-60-percent.json", 100);
    generate_data_by_frequency(0.650, "test-data-65-percent.json", 100);
    generate_data_by_frequency(0.700, "test-data-70-percent.json", 100);
    generate_data_by_frequency(0.750, "test-data-75-percent.json", 100);
    generate_data_by_frequency(0.800, "test-data-80-percent.json", 100);
    generate_data_by_frequency(0.850, "test-data-85-percent.json", 100);
    generate_data_by_frequency(0.900, "test-data-90-percent.json", 100);
    generate_data_by_frequency(0.950, "test-data-95-percent.json", 100);
    generate_data_by_frequency(1.00, "test-data-100-percent.json", 100); 

   // generate_test_data_large(10 * 1024, "test-data-10MB.json", 0.0, 0.0, 0.0);
    //  generate_test_data_large(100 * 1024, "test-data-100MB.json", 0.0, 0.0, 0.0);
   // generate_test_data_large(1000 * 1024, "test-data-1000MB.json", 0.0, 0.0, 0.0);

  //  generate_data_by_frequency(0.200, "new-test-data-20-percent.json", 100);
}
