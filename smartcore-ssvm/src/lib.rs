use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
    println!("The Rust function say() received {}", s);
    let r = String::from("hello ");
    return r + s;
}

const FILE_NAME: &str = "iris_knn.model";
use std::fs::File;
use ssvm_wasi_helper::ssvm_wasi_helper::_initialize;
use std::io::{Write, Read};

#[wasm_bindgen]
pub fn create_file(path: &str, content: &str) -> String {
  _initialize();
  let mut output = File::create(path).unwrap();
  output.write_all(content.as_bytes()).unwrap();
  path.to_string()
}

#[wasm_bindgen]
pub fn read_file(path: &str) -> String {
  _initialize();
  let mut f = File::open(path).unwrap();
  let mut s = String::new();
  match f.read_to_string(&mut s) {
    Ok(_) => s,
    Err(e) => e.to_string(),
  }
}

#[wasm_bindgen]
pub fn load_model() -> String {
    _initialize();
        let model: LinearRegression<f64, DenseMatrix<f64>> = {
            let mut buf: Vec<u8> = Vec::new();
            File::open(&FILE_NAME)
                .and_then(|mut f| f.read_to_end(&mut buf))
                .expect("Can not load model");
            bincode::deserialize(&buf).expect("Can not deserialize the model")
        };
        let data = DenseMatrix::from_array(1, 6, &[234.289, 235.6, 159.0, 107.608, 1947., 60.323]);

        let prediction = model.predict(&data).unwrap();
        return prediction[0].to_string();
}

// DenseMatrix wrapper around Vec
use smartcore::linalg::naive::dense_matrix::DenseMatrix;
// Linear Regression
use smartcore::linear::linear_regression::LinearRegression;
// Model performance
use smartcore::metrics::mean_squared_error;
use smartcore::model_selection::train_test_split;

// Load dataset
#[wasm_bindgen]
pub fn basic_prediction2() -> String {
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
    let (x_train, x_test, y_train, y_test) = train_test_split(&x, &y, 0.2, true);
    let y_hat_lr = LinearRegression::fit(&x_train, &y_train, Default::default())
        .and_then(|lr| lr.predict(&x_test))
        .unwrap();
    let mse = mean_squared_error(&y_test, &y_hat_lr);

    return mse.to_string();
}