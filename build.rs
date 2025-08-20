use std::env;
use std::path::Path;
use std::{fs, io, path::PathBuf};

fn main() {
    let profile = std::env::var("PROFILE").unwrap();

    if profile == "release" {
        let lib_installer = LibInstaller;
        match lib_installer.install_stdlib() {
            Ok(_) => println!("Std lib copied!"),
            Err(_) => println!("Fail to copy std lib"),
        }
    } else {
        println!("Running debug-specific build step...");
    }
}

struct LibInstaller;

impl LibInstaller {
    fn copy_dir_all(&self, src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
        fs::create_dir_all(&dst)?;
        for entry in fs::read_dir(&src)? {
            let entry = entry?;
            let ty = entry.file_type()?;

            let src_path = entry.path();
            let dst_path = dst.as_ref().join(entry.file_name());

            if ty.is_dir() {
                self.copy_dir_all(src_path, dst_path)?;
            } else {
                fs::copy(src_path, dst_path)?;
            }
        }
        Ok(())
    }

    pub fn install_stdlib(&self) -> io::Result<()> {
        let src = Path::new("src/std");
        let dst = self.get_rei_std_path();
        self.copy_dir_all(src, dst)
    }

    fn get_rei_std_path(&self) -> PathBuf {
        if let Ok(custom_path) = env::var("REI_HOME") {
            return PathBuf::from(custom_path).join("std");
        }

        #[cfg(target_os = "linux")]
        return PathBuf::from("/usr/share/rei/std");

        #[cfg(target_os = "macos")]
        return PathBuf::from("/usr/local/share/rei/std");

        #[cfg(target_os = "windows")]
        return PathBuf::from("C:\\ProgramData\\rei\\std");
    }
}
