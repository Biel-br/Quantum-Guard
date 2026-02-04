use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::path::Path;

use memmap2::Mmap;
use blake3;

use std::os::windows::io::AsRawHandle;
use windows::Win32::Foundation::HANDLE;

use windows::Win32::Storage::FileSystem::{
    GetFileInformationByHandle,
    BY_HANDLE_FILE_INFORMATION,
};

pub fn get_file_id(file: &File) -> io::Result<u64> {
    let mut info = BY_HANDLE_FILE_INFORMATION::default();
    let handle = HANDLE(file.as_raw_handle() as isize);

    unsafe {
        GetFileInformationByHandle(handle, &mut info)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }

    Ok(((info.nFileIndexHigh as u64) << 32) | info.nFileIndexLow as u64)
}

pub fn process_file(path: &Path, seen: &mut HashSet<u64>) -> io::Result<()> {
    let file = File::open(path)?;

    let file_id = get_file_id(&file)?;

    if !seen.insert(file_id) {
        println!("Ignorando duplicado: {:?}", path);
        return Ok(());
    }

    let mmap = unsafe { Mmap::map(&file)? };

    let hash = blake3::hash(&mmap);

    println!(
        "Arquivo: {:?}\nHash: {}\n",
        path,
        hash.to_hex()
    );

    Ok(())
}



