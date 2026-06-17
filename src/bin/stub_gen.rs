use _pyroquad::pyo3_stub_gen::Result;
use std::path::PathBuf;
use _pyroquad::stub_info;

/// Running this generates the stub info for the engine. With stub info being 'projectname.pyi'.
fn main() -> Result<()> {
    match stub_info() {
        Ok(stub) => {
            println!("stub_info succeeded.");
            stub.generate()?;
            Ok(())
        }
        Err(e) => {
            eprintln!("stub_info() failed: {:?}", e);
            let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| "<unknown>".into());
            eprintln!("CARGO_MANIFEST_DIR = {:?}", manifest);
            let pyp = PathBuf::from(&manifest).join("pyproject.toml");
            eprintln!("Looking for pyproject.toml at {:?}", pyp);
            match std::fs::metadata(&pyp) {
                Ok(md) => eprintln!("Found pyproject.toml, metadata: {:?}", md),
                Err(err) => eprintln!("pyproject.toml lookup failed: {:?}", err),
            }
            match std::fs::read_to_string(&pyp) {
                Ok(s) => {
                    eprintln!("pyproject.toml contents begin:\n{}", &s[..s.len().min(200)]);
                }
                Err(err) => eprintln!("Could not read pyproject.toml: {:?}", err),
            }
            let guess = PathBuf::from(&manifest).join(format!("{}.pyi", "pyroquad"));
            eprintln!("Guess stub path: {:?}", guess);
            return Err(e);
        }
    }
}
