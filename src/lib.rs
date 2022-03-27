use std::{fs, io, path::PathBuf};

#[derive(Debug, Clone, Copy)]
pub enum Data {
    /// `&[&u8]`
    Content,
    /// `&[&str]`
    Paths,
    /// `&[(&str, &[u8])]`
    Both,
}

impl Data {
    fn generate_data_structure(&self, var_name: &str, data: &[PathBuf]) -> String {
        match self {
            Self::Content => {
                let mut out = format!("const {var_name}: &[&[u8]] = &[\n");
                for p in data {
                    out += &format!("    include_bytes!({:?}),\n", p);
                }
                out += "];\n";
                out
            }
            Self::Paths => {
                let mut out = format!("const {var_name}: &[&str] = &[\n");
                for p in data {
                    out += &format!("    {:?},\n", p);
                }
                out += "];\n";
                out
            }
            Self::Both => {
                let mut out = format!("const {var_name}: &[(&str, &[u8])] = &[\n");
                for p in data {
                    out += &format!("    ({:?}, include_bytes!({:?})),\n", p, p);
                }
                out += "];\n";
                out
            }
        }
    }
}

pub fn include_dir(
    path: impl Into<PathBuf>,
    out_file: impl Into<PathBuf>,
    var_kind: Data,
    var_name: &str,
    recursive: bool,
) -> io::Result<()> {
    let mut paths = Vec::new();
    read_dir(path, &mut paths, recursive)?;
    let data = var_kind.generate_data_structure(var_name, &paths);
    fs::write(out_file.into(), data)?;
    Ok(())
}

fn read_dir(dir: impl Into<PathBuf>, paths: &mut Vec<PathBuf>, recursive: bool) -> io::Result<()> {
    let dir: PathBuf = dir.into();
    for maybe_dir_entry in fs::read_dir(&dir)? {
        let dir_entry = maybe_dir_entry?;
        let metadata = dir_entry.metadata()?;
        if metadata.is_file() {
            paths.push(dir.join(dir_entry.file_name()));
        } else if metadata.is_dir() && recursive {
            read_dir(&dir_entry.path(), paths, recursive)?;
        }
    }
    Ok(())
}
