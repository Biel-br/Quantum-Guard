mod engine; //aponta para a pasta engine/

use std::collections::HashSet;
use walkdir::WalkDir;
use std::io;

fn main() -> io::Result<()> {
    let mut seen_files: HashSet<u64> = HashSet::new();

    for entry in WalkDir::new("C:\\")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            // chama via engine::ingest
            if let Err(e) = engine::ingest::process_file(path, &mut seen_files) {
                eprintln!("Erro em {:?}: {}", path, e);
            }
        }
    }

    Ok(())
}
