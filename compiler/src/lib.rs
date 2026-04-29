use std::{ffi::{OsStr, OsString}, process::Command};

pub struct Compiler {
    binary: OsString
}

impl Compiler{
    
    pub fn new<P: AsRef<OsStr>>(binary_path: P) -> Self {
        Self { 
            binary: binary_path.as_ref().to_os_string(),
        }
    }
    
    pub fn to_sql(&self, prop: &str) -> Result<String, String> {
        let output = Command::new(&self.binary)
            .arg(prop)
            .output()
            .map_err(|e| 
                format!("Failed to run the Haskell compiler backend: {}", e)
            )?;
        if output.status.success() {
            let result_string = 
                String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(result_string)
        } else {
            let error_string = 
                String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(format!(
                    "Compiler exited with {}.\nError output:\n{}",
                    output.status, error_string
            ))
        }
    }
}

