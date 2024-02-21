fn filter_not_nan(df: &DataFrame) -> DataFrame {
  // NaNではない行のみを抽出する
  let filtered_df = df
    .filter(|row| {
      // すべての列がNaNではないことを確認する
      row.iter_cols().all(|col| !col.is_nan())
    })
    .collect();

  // エラー処理
  if filtered_df.is_empty() {
    panic!("すべての行がNaNです");
  }

  filtered_df
}
