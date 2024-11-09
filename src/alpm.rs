use std::ops::Deref;

use alpm::{Alpm, Group, Package};
use alpm_utils::DbListExt;
use anyhow::{bail, Context};

use crate::temp_db::{initialize_temp_db, TEMP_DB_PATH};

fn pacmanconf() -> anyhow::Result<pacmanconf::Config> {
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

    pub fn installed(&self, package: &str) -> bool {
        let localdb = self.0.localdb();
        localdb.pkg(package).is_ok() || localdb.group(package).is_ok()
    }

    pub fn package_was_updated_in_db(
        &self,
        alpm_tmp: &TempAlpm,
        package: &str,
    ) -> anyhow::Result<bool> {
        let pkg = self.syncdb_pkg(package)?;
        let pkg_tmp = alpm_tmp.syncdb_pkg(package)?;
        Ok(pkg.version() < pkg_tmp.version())
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
        syncdb_pkg(&self.0, package)
    }
}

fn syncdb_pkg<'a>(alpm: &'a Alpm, package: &str) -> anyhow::Result<&'a Package> {
    alpm.syncdbs()
        .pkg(package)
        .context("Package {package} not found")
}

pub struct TempAlpm(PacrsAlpm);

impl TempAlpm {
    pub fn new() -> anyhow::Result<Self> {
        let conf = pacmanconf()?;
        let mut alpm = Alpm::new(&*conf.root_dir, TEMP_DB_PATH)
            .context("Failed to initialize alpm connection")?;
        alpm_utils::configure_alpm(&mut alpm, &conf).context("Failed to configure alpm")?;
        initialize_temp_db()?;
        Ok(Self(PacrsAlpm(alpm)))
    }
}

impl Deref for TempAlpm {
    type Target = PacrsAlpm;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
