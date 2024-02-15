fn str_to_num(str_val: &Series) -> Series {
    str_val
        .utf8()
        .unwrap()
        .into_iter()
        .map(|opt_name: Option<&str>| {
            opt_name.map(|name: &str| {
                match name {
                    "Underweight" => 0,
                    "Normal Weight" => 1,
                    "Overweight" => 2,
                    "Obese Class 1" => 3,
                    "Obese Class 2" => 4,
                    "Obese Class 3" => 5,
                    _ => panic!("Problem species str to num"),
                }
            })
        })
        .collect::<UInt32Chunked>()
        .into_series()
}
