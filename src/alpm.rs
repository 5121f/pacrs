use std::ops::Deref;

use alpm::{Alpm, Group, Package};
use alpm_utils::DbListExt;
use anyhow::{bail, Context};

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

    // pub fn installed(&self, package: &str) -> bool {
    //     let localdb = self.0.localdb();
    //     localdb.pkg(package).is_ok() || localdb.group(package).is_ok()
    // }

    pub fn package_was_updated_in_db(
        &self,
        alpm_tmp: &TempAlpm,
        package: &str,
    ) -> anyhow::Result<bool> {
        let pkg = self.syncdb_pkg(package)?;
        let pkg_tmp = alpm_tmp.syncdb_pkg(package)?;
        Ok(pkg.version() < pkg_tmp.version())
    }

    pub fn pkgs_or_their_deps_was_updated_in_db(
        &self,
        alpm_tmp: &TempAlpm,
        packages: Vec<String>,
    ) -> anyhow::Result<bool> {
        let mut packages_for_check = packages;
        let mut packages_we_already_checked = Vec::with_capacity(packages_for_check.len());
        while let Some(pkg) = packages_for_check.pop() {
            let already_checked = packages_we_already_checked.contains(&pkg);
            if !already_checked {
                if self.package_was_updated_in_db(alpm_tmp, &pkg)? {
                    return Ok(true);
                }
                let deps = self
                    .dependencies(&pkg)?
                    .into_iter()
                    .map(|dep| dep.name().to_owned());
                packages_for_check.extend(deps);
            }
            packages_we_already_checked.push(pkg);
        }
        Ok(false)
    }

    pub fn dependencies<'a>(&'a self, package: &str) -> anyhow::Result<Vec<&'a Package>> {
        if let Ok(pkg) = self.0.syncdbs().pkg(package) {
            let dependencies = pkg
                .depends()
                .into_iter()
                .map(|dep| self.0.syncdbs().find_satisfier(dep.name()).unwrap())
                .collect();
            return Ok(dependencies);
        }
        if let Ok(grp) = self.group(package) {
            return Ok(grp.packages().into_iter().collect());
        }
        bail!("Falied to define package type")
    }

    fn group<'a>(&'a self, group: &str) -> alpm::Result<&'a Group> {
        for db in self.0.syncdbs() {
            if let Ok(grp) = db.group(group) {
                return Ok(grp);
            }
        }
        self.0.localdb().group("error")
    }

    fn syncdb_pkg<'a>(&'a self, package: &str) -> anyhow::Result<&'a Package> {
        self.0
            .syncdbs()
            .pkg(package)
            .with_context(|| format!("Package {package} not found"))
    }
}

impl Deref for PacrsAlpm {
    type Target = Alpm;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
