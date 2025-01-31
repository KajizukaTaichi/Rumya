//! # Rumya
//! Rumya programming language's binding for Rust.
//! You can evaluate Rumya program embedded in Rust.
//! ## Example
//! ```
//! let rumya = Rumya::new().set_rumya(PATH);
//! let result = rumya.eval::<i32>("let x = 0. for i in 1 ~ 10 do x += i. x");
//! assert_eq!(result, Some(45));
//! ```
use std::io::Write;
use std::process::Command;
use std::str::FromStr;
use std::{
    fs::{remove_file, File},
    path::Path,
};

macro_rules! some {
    ($result_value: expr) => {
        if let Ok(ok) = $result_value {
            Some(ok)
        } else {
            None
        }
    };
}

// Environment that manages interpreter paths of Rumya and her base technology Lamuta.
// And parse and evaluate Rumya program from source code string using eval method.
#[derive(Clone)]
pub struct Rumya {
    rumya_path: String,
    lamuta_path: String,
}

impl Rumya {
    pub fn new() -> Self {
        Self {
            rumya_path: "rumya.lm".to_string(),
            lamuta_path: "lamuta".to_string(),
        }
    }

    pub fn set_rumya(&self, path: &str) -> Self {
        let path = Path::new(path);
        Self {
            rumya_path: path.display().to_string(),
            ..self.clone()
        }
    }

    pub fn set_lamuta(&self, path: &str) -> Self {
        let path = Path::new(path);
        Self {
            lamuta_path: path.display().to_string(),
            ..self.clone()
        }
    }

    pub fn eval<T: Sized + FromStr>(&self, code: &str) -> Option<T> {
        const TEMP_FILE_NAME: &str = "Rumya-binding.temp.lm";
        let mut temp_file = some!(File::create(TEMP_FILE_NAME))?;
        let code = format!("print\nbegin\n{code}\nend\n");
        some!(temp_file.write_all(code.as_bytes()))?;

        let output = some!(Command::new(&self.lamuta_path)
            .args([&self.rumya_path, TEMP_FILE_NAME])
            .output())?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            some!(remove_file(TEMP_FILE_NAME))?;
            some!(T::from_str(stdout.lines().last()?))
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // This path is only my environment, change your Rumya path when you test
        const PATH: &str = "/Users/kajizukataichi/Desktop/repositories/Rumya/rumya.lm";
        let rumya = Rumya::new().set_rumya(PATH);
        let result = rumya.eval::<i32>("let x = 0. for i in 1 ~ 10 do x += i. x");
        assert_eq!(result, Some(45));
    }
}
