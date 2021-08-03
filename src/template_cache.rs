use mustache::{Error, Template, compile_path};
use serde::Serialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use tokio::fs::metadata;
use tokio::sync::RwLock;
use tokio::task;

struct TemplateEntry {
    template: Template,       // Compiled template
    mtime: SystemTime,        // mtime of parsed template file
    last_checked: SystemTime, // last time the template file mtime was checked
}

impl TemplateEntry {
    // Loads a template from the given filename
    async fn from_template_file<P: AsRef<Path>>(filename: P) -> Result<TemplateEntry, Error> {
        let path: PathBuf = filename.as_ref().to_path_buf();
        let path2 = path.clone();
        let template = task::spawn_blocking(|| compile_path(path2)).await
            .map_err(|_| Error::Io(std::io::Error::new(std::io::ErrorKind::Other, "thread join error")))??;
        let attr = metadata(path).await?;
        Ok(TemplateEntry{template: template, mtime: attr.modified()?, last_checked: SystemTime::now()})
    }

    // render the tempate with the given data
    fn render<D>(&self, data: &D) -> Result<String, Error>
        where D: Serialize {
        self.template.render_to_string(data)
    }
}

/// ReloadPolicy controls how often the modification time of template
/// file is checked. Checks take place only when rendering the
/// template.
pub enum ReloadPolicy {
    /// Never check for changes. Once loaded, templates will not
    /// change unless one of TemplateCache::clear or
    /// TemplateCache::reload_template are called, or the server is
    /// restarted.
    Never,
    /// Check after a period of time. For example, with a duration of
    /// 30 min, the template will be checked no more than once every
    /// 30 min.
    Periodic(Duration),
    /// Check every time the template is rendered.
    Always,
}

/// Cache of compiled mustache templates
pub struct TemplateCache {
    cache: RwLock<HashMap<PathBuf, TemplateEntry>>,
    reload_policy: ReloadPolicy,
}

impl TemplateCache {
    /// Create a TemplateCache with the specified reload policy
    pub fn with_policy(policy: ReloadPolicy) -> TemplateCache {
        TemplateCache{cache: RwLock::new(HashMap::new()), reload_policy: policy}
    }

    /// Remove all cache entries
    pub async fn clear(&self) {
        let mut c = self.cache.write().await;
        c.clear();
    }

    /// Force a reload of a template into the cache
    pub async fn reload_template<P>(&self, path: P) -> Result<(), Error>
        where P: AsRef<Path> {

        let mut c = self.cache.write().await;
        let template = TemplateEntry::from_template_file(&path).await?;
        c.insert(path.as_ref().to_path_buf(), template);
        Ok(())
    }

    // Tries to render the template with the given data. This method
    // only needs a read lock. Returns:
    //
    //   * Ok(Some(String))  - successfully rendered
    //
    //   * Ok(None) - template needs loading, either it was never
    //                loaded, or it is outdated
    //
    //   * Err(e) - mustache error
    async fn try_render_template<P, D>(&self, path: P, data: &D) -> Result<Option<String>, Error>
        where P: AsRef<Path>, D: Serialize {

        let c = self.cache.read().await;
        if let Some(template) = c.get(&path.as_ref().to_path_buf()) {
            let check_mtime = match self.reload_policy {
                ReloadPolicy::Never => false,
                ReloadPolicy::Always => true,
                ReloadPolicy::Periodic(period) => {
                    let now = SystemTime::now();
                    if let Ok(duration) = now.duration_since(template.last_checked) {
                        duration > period
                    } else {
                        // wierdness, we went back in time, force reload
                        true
                    }
                }
            };
            if check_mtime {
                let mtime = metadata(path).await?.modified()?;
                if mtime > template.mtime {
                    return Ok(None);
                }
            }
            let rendered = template.render(data)?;
            Ok(Some(rendered))
        } else {
            Ok(None)
        }
    }

    // Load the template from disk, compile it, store the compiled
    // template in cache, and render. This needs a write lock.
    async fn load_render_template<P, D>(&self, path: P, data: &D) -> Result<String, Error>
        where P: AsRef<Path>, D: Serialize {

        let mut c = self.cache.write().await;
        let template = TemplateEntry::from_template_file(&path).await?;
        let rendered = template.render(data)?;
        c.insert(path.as_ref().to_path_buf(), template);
        Ok(rendered)
    }

    /// Render the template at `path` to `writer` with
    /// `data`. Templates will be reloaded if necessary according to
    /// the reload policy.
    pub async fn render<P, D>(&self, path: P, data: &D) -> Result<String, Error>
        where P: AsRef<Path>, D: Serialize {
        let rendered = match self.try_render_template(&path, data).await {
            Ok(r) => r,
            Err(e) => {
                // Previously compiled template failed to render. Log
                // an error and force a reload.
                error!("Template render error: {:?}", e);
                None
            },
        };
        if let Some(r) = rendered {
            Ok(r)
        } else {
            self.load_render_template(&path, data).await
        }
    }
}
