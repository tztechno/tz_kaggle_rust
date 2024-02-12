/*
polars = "0.14.7"
polars-core = {version = "0.14.7", features=["ndarray"]}
smartcore = { version = "0.2.0", default-features = false, features=["nalgebra-bindings", "ndarray-bindings", "datasets"]}
*/


use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use smartcore::linalg::BaseMatrix;
use smartcore::linear::logistic_regression::LogisticRegression;
use smartcore::model_selection::train_test_split;

// metrics
use smartcore::metrics::mean_squared_error;
use smartcore::metrics::accuracy;

use std::convert::TryFrom;
use std::fs::File;
use std::path::Path;

use polars::prelude::*;
use polars::frame::DataFrame;
use polars::prelude::Result as PolarResult;
use polars::prelude::SerReader;


//CSVファイルを読み込んでDataFrameを返す
fn read_csv_with_schema<P: AsRef<Path>>(path: P) -> PolarResult<DataFrame> {
    let schema = Schema::new(vec![
        Field::new("species", DataType::Utf8),
        Field::new("island", DataType::Utf8),
        Field::new("culmen_length_mm", DataType::Float64),
        Field::new("culmen_depth_mm", DataType::Float64),
        Field::new("flipper_length_mm", DataType::Float64),
        Field::new("body_mass_g", DataType::Float64),
        Field::new("sex", DataType::Utf8)
    ]);
    let file = File::open(path).expect("Cannot open file.");
    CsvReader::new(file)
        .with_schema(Arc::new(schema))
        .has_header(true)
        .with_ignore_parser_errors(true) //エラーが出ても処理継続
        .finish()
}

//featureとtargetに分割
fn get_feature_target(df: &DataFrame) -> (PolarResult<DataFrame>, PolarResult<DataFrame>) {
    let features = df.select(vec![
        "culmen_length_mm",
        "culmen_depth_mm",
        "flipper_length_mm",
        "body_mass_g",
    ]);
    let target = df.select("species");
    (features, target)
}

/* 機能データフレームを smartcore で読める DenseMatrix に変換 */
pub fn convert_features_to_matrix(df: &DataFrame) -> Result<DenseMatrix<f64>> {
    let nrows = df.height();
    let ncols = df.width();

    let features_res = df.to_ndarray::<Float64Type>().unwrap();
    let mut xmatrix: DenseMatrix<f64> = BaseMatrix::zeros(nrows, ncols);
    let mut col: u32 = 0;
    let mut row: u32 = 0;

    for val in features_res.iter() {
        let m_row = usize::try_from(row).unwrap();
        let m_col = usize::try_from(col).unwrap();
        xmatrix.set(m_row, m_col, *val);
        // check what we have to update
        if m_col == ncols - 1 {
            row += 1;
            col = 0;
        } else {
            col += 1;
        }
    }

    Ok(xmatrix)
}

//speciesのLabelエンコーディング用function
fn str_to_num(str_val: &Series) -> Series {
    str_val
        .utf8()
        .unwrap()
        .into_iter()
        .map(|opt_name: Option<&str>| {
            opt_name.map(|name: &str| {
                match name {
                    "Adelie" => 1,
                    "Chinstrap" => 2,
                    "Gentoo" => 3,
                    _ => panic!("Problem species str to num"),
                }
            })
        })
        .collect::<UInt32Chunked>()
        .into_series()
}

fn main() {

    //csv読み込み
    let csv = "./penguins.csv";
    let df: DataFrame = read_csv_with_schema(&csv).unwrap();

    //不正データdrop
    let df2 = df.drop_nulls(None).unwrap();

    //featrures/target分割
    let (feature, target) = get_feature_target(&df2);

    //features変換
    let xmatrix = convert_features_to_matrix(&feature.unwrap());

    //speciesのLabelエンコーディング
    let target_array = target
        .unwrap()
        .apply("species", str_to_num)
        .unwrap()
        .to_ndarray::<Float64Type>()
        .unwrap();

    // create a vec type and populate with y values
    let mut y: Vec<f64> = Vec::new();
    for val in target_array.iter() {
        y.push(*val);
    }

    //データ分割
    let (x_train, x_test, y_train, y_test) = train_test_split(&xmatrix.unwrap(), &y, 0.3, true);

    //学習
    let reg = LogisticRegression::fit(&x_train, &y_train, Default::default()).unwrap();

    //予測
    let preds = reg.predict(&x_test).unwrap();
    let mse = mean_squared_error(&y_test, &preds);
    println!("MSE: {}", mse);
    println!("accuracy : {}", accuracy(&y_test, &preds));
}
