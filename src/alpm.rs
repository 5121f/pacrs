// SPDX-License-Identifier: GPL-3.0-only

use std::ops::Deref;

use alpm::{Alpm, Group, Package};
use alpm_utils::DbListExt;
use anyhow::{anyhow, Context};

use crate::temp_db::TempAlpm;

pub fn pacmanconf() -> anyhow::Result<pacmanconf::Config> {
    pacmanconf::Config::new().context("Failed to read pacmanconf")
}

pub struct PacrsAlpm(Alpm);

impl PacrsAlpm {
    pub fn new() -> anyhow::Result<Self> {
        let conf = pacmanconf()?;
        let alpm =
            alpm_utils::alpm_with_conf(&conf).context("Failed to initialize alpm connection")?;
        Ok(Self(alpm))
    }

    pub fn with_alpm(alpm: Alpm) -> Self {
        Self(alpm)
    }

    pub fn package_was_updated(&self, alpm_tmp: &TempAlpm, package: &str) -> anyhow::Result<bool> {
        let pkg = self.syncdb_pkg(package)?;
        let pkg_tmp = alpm_tmp.syncdb_pkg(package)?;
        Ok(pkg.version() < pkg_tmp.version())
    }

    pub fn pkgs_or_their_deps_was_updated_in_db(
        &self,
        alpm_tmp: &TempAlpm,
        packages: Vec<&str>,
    ) -> anyhow::Result<Vec<String>> {
        let mut for_check = packages;
        let mut already_checked = Vec::with_capacity(for_check.len());
        let mut update_pkgs = Vec::new();
        while let Some(pkg) = for_check.pop() {
            if already_checked.contains(&pkg) {
                continue;
            }
            let was_updated = self
                // We assume that if package not found in syncdb, then the package from AUR and we ignore it
                .package_was_updated(alpm_tmp, pkg)
                .unwrap_or(false);
            if was_updated {
                update_pkgs.push(pkg.to_owned());
            }
            let deps = self
                .dependencies(pkg)
                // We assume that if you could not find dependencies, then the package from AUR and we ignore it
                .unwrap_or_default()
                .into_iter()
                .map(|dep| dep.name());
            for_check.extend(deps);
            already_checked.push(pkg);
        }
        Ok(update_pkgs)
    }

    pub fn dependencies<'a>(&'a self, package: &str) -> anyhow::Result<Vec<&'a Package>> {
        self.0
            .syncdbs()
            .pkg(package)
            .ok()
            .map(|pkg| {
                pkg.depends()
                    .into_iter()
                    .map(|dep| self.0.syncdbs().find_satisfier(dep.name()).unwrap())
                    .collect::<Vec<_>>()
            })
            .or_else(|| {
                self.group(package)
                    .ok()
                    .map(|grp| grp.packages().into_iter().collect::<Vec<_>>())
            })
            .context("Failed to define package type")
    }

    fn group<'a>(&'a self, group: &str) -> anyhow::Result<&'a Group> {
        self.0
            .syncdbs()
            .into_iter()
            .map(|db| db.group(group).ok())
            .find(|grp| grp.is_some())
            .flatten()
            .with_context(|| anyhow!("Failed to find group \"{group}\""))
    }

    // pub fn localdb_pkg<'a>(&'a self, name: &str) -> anyhow::Result<&'a Package> {
    //     self.0
    //         .localdb()
    //         .pkg(name)
    //         .with_context(|| format!("{name}: failed to find package. Maybe it didn't install?"))
    // }

    fn syncdb_pkg<'a>(&'a self, package: &str) -> anyhow::Result<&'a Package> {
        self.0
            .syncdbs()
            .pkg(package)
            .with_context(|| format!("{package}: Package not found"))
    }
}

impl Deref for PacrsAlpm {
    type Target = Alpm;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
