// Capture only runs on Windows: on other systems the walking code is
// compiled but not used.
#![cfg_attr(not(windows), allow(dead_code, unused_imports))]

use std::collections::BTreeMap;
use std::fs::{self, File, Metadata};
use std::io::Read;
use std::path::Path;
use std::time::UNIX_EPOCH;

use sha2::{Digest, Sha256};

use crate::snapshot::error::Result;
use crate::{config, hex};
use crate::snapshot::types::{FileEntry, FileKind};


// Files bigger than this are captured without their hash, so a huge file
// does not slow the capture down.
const MAX_HASH_SIZE: u64 = 64 * 1024 * 1024;


#[cfg(not(windows))]
pub(super) fn capture() -> Result<BTreeMap<String, FileEntry>> {
    Ok(BTreeMap::new())
}


#[cfg(windows)]
pub(super) fn capture() -> Result<BTreeMap<String, FileEntry>> {
    let mut files = BTreeMap::new();

    for target in config::file_targets()? {
        // Targets with an unknown variable are skipped.
        if let Some(path) = expand(&target.path) {
            capture_root(
                Path::new(&path), 
                target.recursive, 
                target.hash, 
                &mut files
            );
        }
    }

    Ok(files)
}


fn capture_root(
    path: &Path,
    recursive: bool,
    hash: bool,
    out: &mut BTreeMap<String, FileEntry>,
) {
    // Paths that don't exist on this system are skipped.
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return;
    };

    out.insert(key(path), file_entry(path, &metadata, hash));

    if metadata.is_dir() {
        walk(path, recursive, hash, out);
    }
}


fn walk(
    dir: &Path,
    recursive: bool,
    hash: bool,
    out: &mut BTreeMap<String, FileEntry>,
) {
    // Folders without read permission are skipped.
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(metadata) = fs::symlink_metadata(&path) else {
            continue;
        };

        out.insert(key(&path), file_entry(&path, &metadata, hash));

        // Links are not followed: is_dir() is false for a link to a folder.
        if recursive && metadata.is_dir() {
            walk(&path, recursive, hash, out);
        }
    }
}


fn file_entry(path: &Path, metadata: &Metadata, hash: bool) -> FileEntry {
    let kind = if metadata.is_symlink() {
        let target = fs::read_link(path)
            .map(|t| t.to_string_lossy().into_owned())
            .unwrap_or_default();
        FileKind::Symlink { target }

    } else if metadata.is_dir() {
        FileKind::Directory

    } else {
        let size = metadata.len();
        let sha256 = if hash && size <= MAX_HASH_SIZE { sha256(path) } else { None };
        FileKind::File { size, sha256 }
    };

    let modified = metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map_or(0, |d| d.as_secs());

    FileEntry { kind, modified }
}


// Reads the file in blocks, so big files are not loaded in memory.
// Files that cannot be read (in use, no permission) get no hash.
fn sha256(path: &Path) -> Option<String> {
    let mut file = File::open(path).ok()?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];

    loop {
        let read = file.read(&mut buffer).ok()?;
        if read == 0 { break; }
        
        hasher.update(&buffer[..read]);
    }

    Some(hex::encode(&hasher.finalize()))
}


fn key(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}


// Expands %NAME% variables. Returns None if one does not exist.
fn expand(path: &str) -> Option<String> {
    let mut out = String::new();
    let mut rest = path;

    while let Some(start) = rest.find('%') {
        out.push_str(&rest[..start]);

        let after = &rest[start + 1..];
        let end = after.find('%')?;

        out.push_str(&std::env::var(&after[..end]).ok()?);
        rest = &after[end + 1..];
    }

    out.push_str(rest);
    Some(out)
}
