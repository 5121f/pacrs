// SPDX-License-Identifier: GPL-3.0-only

use std::fmt;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use anyhow::Context;
use bytesize::ByteSize;
use fs_err as fs;
use owo_colors::OwoColorize;
use regex::Regex;

use crate::cli::{Answer, Cli};

const PACMAN_CACHE_PATH: &str = "/var/cache/pacman/pkg";

const CACHE_ENTRY_REGEX: &str = r"(?<name>[\w\-\d\.+]+)-(?<version>[\w\d\.\-:+]*)-(?<subversion>[\w\d\._]+)-(?<arch>[\d\w_]+)\.(?<ext>[\w\.]+)";

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CacheEntry {
    pkg_name: String,
    version: String,
    subversion: String,
    arch: String,
    ext: String,
}

pub fn clean(keep: u8) -> anyhow::Result<()> {
    let remove_candidates = remove_candidates(keep)?;
    let candidates_count = remove_candidates.len();
    if candidates_count == 0 {
        println!("No candidates to remove");
        return Ok(());
    }
    show_cache(&remove_candidates, true)?;
    let mut cli = Cli::new();
    let answer = cli.confirm("Remove?", Answer::Yes)?;
    if answer.is_no() {
        return Ok(());
    }
    let mut total_size = 0;
    for entry in remove_candidates {
        let path = entry.path();
        let metadata = entry.path().metadata()?;
        total_size += metadata.size();
        let path_str = path.to_string_lossy();
        log::info!("Removing file: {path_str}");
        fs::remove_file(&path)?;
        let sig_path = format!("{path_str}.sig");
        let metadata = Path::new(&sig_path).metadata()?;
        total_size += metadata.size();
        log::info!("Removing file: {sig_path}");
        fs::remove_file(sig_path)?;
    }
    let files_count = candidates_count * 2;
    let total_size = ByteSize::b(total_size).to_string();
    let total_size = total_size.bright_blue();
    let prompt = "==>".green();
    println!(
        "{}",
        format!(
            "{prompt} {candidates_count} candidates. \
            {files_count} files removed. \
            {total_size} disk space saved"
        )
        .bold()
    );
    Ok(())
}

pub fn show_cache(cache: &[CacheEntry], only_stats: bool) -> anyhow::Result<()> {
    let mut total_size = 0;
    for entry in cache {
        let metadata = entry.path().metadata()?;
        total_size += metadata.size();
        if !only_stats {
            println!("{entry}");
        }
    }
    let candidates_count = cache.len();
    let total_size = ByteSize::b(total_size).to_string();
    let total_size = total_size.bright_blue();
    let prompt = "==>".green();
    println!(
        "{}",
        format!("{prompt} {candidates_count} candidates using {total_size} of disk").bold()
    );
    Ok(())
}

pub fn remove_candidates(keep: u8) -> anyhow::Result<Vec<CacheEntry>> {
    let mut cache = read_cache()?;
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
            if cache[base].pkg_name != cache[y].pkg_name {
                break;
            }
            remove_candidates.push(cache[y].clone());
            i += 1;
        }
        i += 1;
    }
    Ok(remove_candidates)
}

fn read_cache() -> anyhow::Result<Vec<CacheEntry>> {
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
    Ok(cache)
}

fn parse_file_name(file_name: &str, regex: &Regex) -> Option<CacheEntry> {
    let captures = regex.captures(file_name)?;
    Some(CacheEntry {
        pkg_name: captures.name("name")?.as_str().to_string(),
        version: captures.name("version")?.as_str().to_string(),
        subversion: captures.name("subversion")?.as_str().to_string(),
        arch: captures.name("arch")?.as_str().to_string(),
        ext: captures.name("ext")?.as_str().to_string(),
    })
}

impl CacheEntry {
    fn path(&self) -> PathBuf {
        Path::new(PACMAN_CACHE_PATH).join(self.to_string())
    }
}

impl fmt::Display for CacheEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{name}-{version}-{subversion}-{arch}.{ext}",
            name = self.pkg_name,
            version = self.version,
            subversion = self.subversion,
            arch = self.arch,
            ext = self.ext
        )
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
