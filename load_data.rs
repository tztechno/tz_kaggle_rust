


###############################################



###############################################

###############################################

###############################################


###############################################
###############################################
###############################################

###############################################



###############################################
fn main() -> PolarsResult<()> {
    let file = std::fs::File::open("/home/ritchie46/Downloads/tpch/tables_scale_100/lineitem.tbl")
        .unwrap();
    let file = Box::new(file) as Box<dyn MmapBytesReader>;
    let _df = CsvReader::new(file)
        .with_separator(b'|')
        .has_header(false)
        .with_chunk_size(10)
        .batched_mmap(None)
        .unwrap();
  
    Ok(())
}
###############################################
    let breast_cancer_data::<i32> = bc_load();

    let x: DenseMatrix<f32> = DenseMatrix::new(
        breast_cancer_data.num_samples,
        breast_cancer_data.num_features,
        breast_cancer_data.data,
        false
    );

    let y<i32> = breast_cancer_data.target;
###############################################
