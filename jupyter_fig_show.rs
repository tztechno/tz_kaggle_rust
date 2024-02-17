

:dep evcxr_jupyter = "0.4"

use evcxr_jupyter::{execute, JupyterOutput};

    execute!(
    &root,
    JupyterOutput::Png(root.into_bitmap())
)?;
    

