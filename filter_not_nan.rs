fn filter_not_nan(df: &DataFrame) -> DataFrame {
    let filtered_df = df.iter_rows()
        .filter(|row| {
            // Iterate through columns and check for NaN using column methods
            // (replace with appropriate NaN checks based on your library)
            row.iter_cols().all(|col| !col.is_nan())
        })
        .collect();
    filtered_df
}
