    
    # Extract target values (c_values)
    let target = c_values.into_iter().map(|val| *val).collect::<Vec<f64>>();

    # Combine features (a_values and b_values) into a single matrix
    let mut features = Vec::new();
    for (a, b) in a_values.iter().zip(b_values.iter()) {
      features.push(vec![*a, *b]);
    }
