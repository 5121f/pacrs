use std::{fmt, path::Path};

use anyhow::Context;
use fs_err as fs;
use regex::Regex;

const PACMAN_CACHE_PATH: &str = "/var/cache/pacman/pkg";

// Packages that have not been updated for a long time may not have a subversion
const CACHE_ENTRY_REGEX: &str = r"(?<name>[\w\-\d\.+]+)-(?<version>[\w\d\.\-:+]*)-(?<subversion>[\w\d\._]+)-(?<arch>[\d\w_]+)\.(?<ext>[\w\.]+)";

#[derive(PartialEq, Eq, Clone, Debug)]
struct CacheEntry {
    pkg_name: String,
    version: String,
    subversion: Option<String>,
    arch: String,
    ext: String,
}

#[allow(clippy::needless_range_loop)]
pub fn clean(keep: u8) -> anyhow::Result<()> {
    let regex = Regex::new(CACHE_ENTRY_REGEX)
        .context("failed to compile regular expression for cache file names")?;
    let mut cache = Vec::new();
    for entry in fs::read_dir(PACMAN_CACHE_PATH)? {
        let entry = entry?;
        match entry.path().extension() {
            Some(ext) if ext == "sig" => continue,
            _ => {}
        }
        let file_name = entry.file_name().to_string_lossy().to_string();
        let cache_entry = parse_file_name(&file_name, &regex)
            .context(format!("failedt to parse file name '{file_name}'"))?;
        cache.push(cache_entry);
    }
    cache.sort();
    cache.reverse();
    let mut remove_candidates = Vec::new();
    let mut i = 0;
    while i < cache.len() {
        let keeped = i + keep as usize;
        if keeped > cache.len() {
            break;
        }
        let base = i;
        for y in keeped..cache.len() {
            if cache[base].pkg_name == cache[y].pkg_name {
                remove_candidates.push(cache[y].clone());
                i += 1;
            } else {
                break;
            }
        }
        i += 1;
    }
    for entry in remove_candidates {
        let path = Path::new(PACMAN_CACHE_PATH).join(entry.to_string());
        let path_str = path.to_string_lossy();
        log::info!("Removing file: {path_str}");
        fs::remove_file(&path)?;
        let sig_path = format!("{path_str}.sig");
        log::info!("Removing file: {sig_path}");
        fs::remove_file(sig_path)?;
    }
    Ok(())
}

fn parse_file_name(file_name: &str, regex: &Regex) -> Option<CacheEntry> {
    let captures = regex.captures(file_name)?;
    Some(CacheEntry {
        pkg_name: captures.name("name")?.as_str().to_string(),
        version: captures.name("version")?.as_str().to_string(),
        subversion: captures.name("subversion").map(|v| v.as_str().to_string()),
        arch: captures.name("arch")?.as_str().to_string(),
        ext: captures.name("ext")?.as_str().to_string(),
    })
}

impl fmt::Display for CacheEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.pkg_name, self.version)?;
        if let Some(subversion) = &self.subversion {
            write!(f, "-{subversion}")?;
        }
        write!(f, "-{arch}.{ext}", arch = self.arch, ext = self.ext)
    }
}

impl PartialOrd for CacheEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CacheEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.pkg_name
            .cmp(&other.pkg_name)
            .then(self.version.cmp(&other.version))
            .then(self.subversion.cmp(&other.subversion))
            .then(self.arch.cmp(&other.arch))
            .then(self.ext.cmp(&other.ext))
    }
}
