
extern crate libc;

use std::io;
use std::env;
use std::cmp;
use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;


#[cfg(target_family = "unix")]
fn mtime<P: AsRef<Path>>(path: P) -> io::Result<i64> {
    use std::os::unix::fs::MetadataExt;
    fs::metadata(path).map(|m| m.mtime())
}

#[cfg(target_family = "unix")]
const PYTHON_NAME: &str = "python3";

#[cfg(target_family = "windows")]
fn mtime<P: AsRef<Path>>(path: P) -> io::Result<i64> {
    use std::os::windows::fs::MetadataExt;
    fs::metadata(path).map(|m| m.last_write_time() as i64)
}

#[cfg(target_family = "windows")]
const PYTHON_NAME: &str = "python";


fn visit_xml<F>(xml_dir: &Path, cb: F) -> io::Result<()>
        where F: Fn(&Path) -> io::Result<()> {
    if fs::metadata(xml_dir)?.is_dir() {
        for entry in fs::read_dir(xml_dir)? {
            let path = entry?.path();
            if fs::metadata(&path)?.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "xml" { cb(&path)?; }
                }
            }
        }
    }
    Ok(())
}


fn xml_to_rs (rs_dir: &Path, xml_file: &Path) -> PathBuf {
    let mut path = PathBuf::from(&rs_dir);
    path.push(xml_file.file_stem().unwrap());
    path.set_extension("rs");
    path
}

fn optional_mtime (path: &Path, default: i64) -> i64 {
    mtime(path).unwrap_or(default)
}

fn main() {
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let r_client = Path::new(&root).join("rs_client.py");
    let build_rs = Path::new(&root).join("build.rs");
    let xml_dir = Path::new(&root).join("xml");
    let src_dir = Path::new(&root).join("src");
    let src_ffi_dir = Path::new(&src_dir).join("ffi");
    let out_dir = env::var("OUT_DIR").unwrap();

    let r_client_mtime = mtime(&r_client).unwrap();
    let build_rs_mtime = mtime(&build_rs).unwrap();
    let ref_mtime = cmp::max(r_client_mtime, build_rs_mtime);

    visit_xml(&xml_dir, |xml_file: &Path| -> io::Result<()> {
        let src_file = xml_to_rs(&src_dir, &xml_file);
        let ffi_file = xml_to_rs(&src_ffi_dir, &xml_file);
        let xml_file_mtime = mtime(&xml_file)?;
        let src_file_mtime = optional_mtime(&src_file, 0);
        let ffi_file_mtime = optional_mtime(&ffi_file, 0);

        let ref_mtime = cmp::max(ref_mtime, xml_file_mtime);

        if ref_mtime > src_file_mtime || ref_mtime > ffi_file_mtime {

            let status = Command::new(PYTHON_NAME)
                    .arg("-B")      // disable __pycache__ dir that messes with cargo
                    .arg(&r_client)
                    .arg("-o").arg(&out_dir)
                    .arg(&xml_file)
                    .env("PYTHONHASHSEED", "0")
                    .status()
                    .expect("Unable to find build dependency python3");
            if !status.success() {
                panic!("processing of {} returned non-zero ({})",
                    xml_file.display(), status.code().unwrap());
            }
        }
        Ok(())
    }).unwrap();


    let xcbgen_dir = Path::new(&root).join("xcbgen");

    println!("cargo:rerun-if-changed={}", &build_rs.display());
    println!("cargo:rerun-if-changed={}", &r_client.display());
    println!("cargo:rerun-if-changed={}", &xml_dir.display());
    println!("cargo:rerun-if-changed={}", &xcbgen_dir.display());

    #[cfg(target_os = "freebsd")]
    println!("cargo:rustc-link-search=/usr/local/lib");
}
