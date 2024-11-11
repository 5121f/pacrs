use anyhow::anyhow;
use raur::{Package, Raur};

use crate::alpm::PacrsAlpm;

pub struct PacrsAur {
    raur: raur::Handle,
}

impl PacrsAur {
    pub fn new() -> Self {
        let raur = raur::Handle::new();
        Self { raur }
    }

    pub async fn pkg(&self, name: &str) -> anyhow::Result<Package> {
        let search_result = self.raur.search_by(name, raur::SearchBy::Name).await?;
        search_result
            .into_iter()
            .find(|pkg| pkg.name == name)
            .ok_or_else(|| anyhow!("{}: failed to find package in AUR", name))
    }

    pub async fn pkg_was_updated(
        &self,
        name: &str,
        alpm: &PacrsAlpm,
    ) -> anyhow::Result<Option<(String, String)>> {
        let pkg = alpm.localdb_pkg(name)?;
        let aur_pkg = self.pkg(name).await?;
        let pkg_ver = pkg.version().as_str();
        if pkg_ver < &*aur_pkg.version {
            return Ok(Some((pkg_ver.to_owned(), aur_pkg.version)));
        }
        Ok(None)
    }
}
