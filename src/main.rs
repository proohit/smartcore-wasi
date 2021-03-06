use chrono::Utc;
use smartcore_wasi_lib::{init, load_model};
use std::env;
use std::io::Write;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        run_perf();
    } else {
        if args[1] == "--save-model" {
            save_model(build_model());
        }
        return;
    }
}

fn run_perf() {
    let mut path = String::from("iris_knn.model\0");
    let ptr = path.as_mut_ptr();
    std::mem::forget(ptr);
    #[cfg(target_arch = "x86_64")]
    init(ptr.cast::<i8>());
    #[cfg(not(target_arch = "x86_64"))]
    init(ptr);
    let mut performances: Vec<u128> = Vec::new();
    let start_time = Utc::now();
    let num_executions: i32 = env::var("noe").unwrap_or(1000.to_string()).parse().unwrap();
    println!("Executing {} times", num_executions);
    for _ in 0..num_executions {
        let now = Instant::now();
        load_model();
        let diff = now.elapsed().as_nanos();
        performances.push(diff);
    }
    let end_time = Utc::now();

    let output = format!(
        "startTime:{}\nendTime:{}\ndata: {:?}",
        start_time, end_time, &performances
    );
    let mut file = std::fs::File::create("data-native.csv").unwrap();
    file.write_all(output.as_bytes()).unwrap();
}

use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use smartcore::linear::linear_regression::LinearRegression;

fn build_model() -> LinearRegression<f64, DenseMatrix<f64>> {
    let x = DenseMatrix::from_2d_array(&[
        &[234.289, 235.6, 159.0, 107.608, 1947., 60.323],
        &[259.426, 232.5, 145.6, 108.632, 1948., 61.122],
        &[258.054, 368.2, 161.6, 109.773, 1949., 60.171],
        &[284.599, 335.1, 165.0, 110.929, 1950., 61.187],
        &[328.975, 209.9, 309.9, 112.075, 1951., 63.221],
        &[346.999, 193.2, 359.4, 113.270, 1952., 63.639],
        &[365.385, 187.0, 354.7, 115.094, 1953., 64.989],
        &[363.112, 357.8, 335.0, 116.219, 1954., 63.761],
        &[397.469, 290.4, 304.8, 117.388, 1955., 66.019],
        &[419.180, 282.2, 285.7, 118.734, 1956., 67.857],
        &[442.769, 293.6, 279.8, 120.445, 1957., 68.169],
        &[444.546, 468.1, 263.7, 121.950, 1958., 66.513],
        &[482.704, 381.3, 255.2, 123.366, 1959., 68.655],
        &[502.601, 393.1, 251.4, 125.368, 1960., 69.564],
        &[518.173, 480.6, 257.2, 127.852, 1961., 69.331],
        &[554.894, 400.7, 282.7, 130.081, 1962., 70.551],
    ]);

    let y: Vec<f64> = vec![
        83.0, 88.5, 88.2, 89.5, 96.2, 98.1, 99.0, 100.0, 101.2, 104.6, 108.4, 110.8, 112.6, 114.2,
        115.7, 116.9,
    ];
    let lr = LinearRegression::fit(&x, &y, Default::default()).unwrap();
    return lr;
}

use std::fs::File;
fn save_model(model: LinearRegression<f64, DenseMatrix<f64>>) {
    {
        let model_binary = bincode::serialize(&model).expect("Can not serialize the model");
        File::create("iris_knn.model")
            .and_then(|mut f| f.write_all(&model_binary))
            .expect("Can not persist model");
    }
}
