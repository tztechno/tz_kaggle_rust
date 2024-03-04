
[dependencies]
pyo3 = {version = "*", features = ["extension-module"]}
pyo3-polars = {version = "0.11", features = ["derive"]}
polars = {version = "0.37", features = ["performant", "lazy", "dtype-array", "ndarray","log", "cutqcut","nightly"]}
num = "0.4.1"
faer = {version = "0.16", features = ["ndarray", "nightly"]}
serde = {version = "*", features=["derive"]}
ndarray = {version="0.15.6", features=["rayon"]}
hashbrown = {version = "0.14.2", features=["nightly"]}
itertools = "0.12.0"
rand = {version = "0.8.5"} # Simd support feature seems to be broken atm
rand_distr = "0.4.3"
realfft = "3.3.0"
rapidfuzz = "0.5.0"
inflections = "1.1.1"
kdtree = {git = "https://github.com/mrhooray/kdtree-rs.git"}
petgraph = "0.6.4"
ordered-float = "4.2.0"

[dependencies]
plotters = "0.3.5"
csv = "1.1.6"
polars = "0.14.7"
polars-core = {version = "0.14.7", features=["ndarray"]}
smartcore = { version = "0.2.0", default-features = false, features=["nalgebra-bindings", "ndarray-bindings", "datasets"]}



////////////////////////////////////////////////////////////////////////////////
  
mod graph;
mod num;
mod stats;
mod stats_utils;
mod str2;
mod utils;

use petgraph::{
    stable_graph::NodeIndex,
    Direction::{Incoming, Outgoing},
};

use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use pyo3::{pymodule, types::PyModule, PyResult, Python};

use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use smartcore::linalg::BaseMatrix;
use smartcore::ensemble::random_forest_regressor::RandomForestRegressor;
use smartcore::model_selection::train_test_split;
use smartcore::metrics::accuracy;
use std::convert::TryFrom;
use std::fs::File;
use std::path::Path; 

use polars::prelude::*;
use polars::frame::DataFrame;
use polars::prelude::Result as PolarResult;
use polars::prelude::SerReader;        
use polars::prelude::*;
use polars::frame::DataFrame;
use polars::prelude::Result as PolarResult;
use polars::prelude::SerReader; 

use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use smartcore::linalg::BaseMatrix;
use smartcore::ensemble::random_forest_regressor::RandomForestRegressor;
use smartcore::model_selection::train_test_split;
use smartcore::metrics::accuracy;

use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

use plotters::prelude::*;
use plotters::evcxr::SVGWrapper;
use plotters::prelude::RED; 
use plotters::prelude::BLUE;
use plotters::prelude::GREEN;
use plotters::prelude::Circle;                
use plotters::prelude::Rectangle;
use plotters::prelude::ChartBuilder;
use plotters::prelude::evcxr_figure;                
                
               
