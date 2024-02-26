


[dependencies]
plotters = "0.3.5"
csv = "1.1.6"
polars = "0.14.7"
polars-core = {version = "0.14.7", features=["ndarray"]}
smartcore = { version = "0.2.0", default-features = false, features=["nalgebra-bindings", "ndarray-bindings", "datasets"]}





use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use smartcore::linalg::BaseMatrix;
use smartcore::ensemble::random_forest_regressor::RandomForestRegressor;
use smartcore::model_selection::train_test_split;
use smartcore::metrics::accuracy;

use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use plotters::prelude::*;
                
use polars::prelude::*;
use polars::frame::DataFrame;
use polars::prelude::Result as PolarResult;
use polars::prelude::SerReader;                
