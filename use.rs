
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



