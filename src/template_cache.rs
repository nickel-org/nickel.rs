use mustache::{Error, Template, compile_path};
use serde::Serialize;
use std::collections::HashMap;
use std::fs::metadata;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use std::time::{Duration, SystemTime};


struct TemplateEntry {
    template: Template,       // Compiled template
    mtime: SystemTime,        // mtime of parsed template file
    last_checked: SystemTime, // last time the template file mtime was checked
}

impl TemplateEntry {
    // Loads a template from the given filename
    fn from_template_file<P: AsRef<Path>>(filename: P) -> Result<TemplateEntry, Error> {
        let path = filename.as_ref();
        let template = compile_path(path)?; // TODO: migration cleanup - needs async file reads
        let attr = metadata(path)?;
        Ok(TemplateEntry{template: template, mtime: attr.modified()?, last_checked: SystemTime::now()})
    }

    // render the tempate with the given data
    fn render<W, D>(&self, writer: &mut W, data: &D) -> Result<(), Error>
        where W: Write, D: Serialize {
        self.template.render(writer, data)
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
    pub fn clear(&self) {
        let mut c = self.cache.write().expect("TemplateCache::clear - cache poisoned");
        c.clear();
    }

    /// Force a reload of a template into the cache
    pub fn reload_template<P>(&self, path: P) -> Result<(), Error>
        where P: AsRef<Path> {

        let mut c = self.cache.write().expect("TemplateCache::reload_template - cache poisoned");
        let template = TemplateEntry::from_template_file(&path)?;
        c.insert(path.as_ref().to_path_buf(), template);
        Ok(())
    }

    // Tries to render the template with the given data. This method
    // only needs a read lock. Returns:
    //
    //   * Ok(true)  - successfully rendered
    //
    //   * Ok(false) - template needs loading, either it was never
    //                 loaded, or it is outdated
    //
    //   * Err(e) - mustache error
    fn try_render_template<P, W, D>(&self, path: P, writer: &mut W, data: &D) -> Result<bool, Error>
        where P: AsRef<Path>, W: Write, D: Serialize {

        let c = self.cache.read().expect("TemplateCache::try_render_template - cache poisoned");
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
                let mtime = metadata(path)?.modified()?;
                if mtime > template.mtime {
                    return Ok(false);
                }
            }
            template.render(writer, data)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // Load the template from disk, compile it, store the compiled
    // template in cache, and render. This needs a write lock.
    fn load_render_template<P, W, D>(&self, path: P, writer: &mut W, data: &D) -> Result<(), Error>
        where P: AsRef<Path>, W: Write, D: Serialize {

        let mut c = self.cache.write().expect("TemplateCache::load_render_template - cache poisoned");
        let template = TemplateEntry::from_template_file(&path)?;
        template.render(writer, data)?;
        c.insert(path.as_ref().to_path_buf(), template);
        Ok(())
    }

    /// Render the template at `path` to `writer` with
    /// `data`. Templates will be reloaded if necessary according to
    /// the reload policy.
    pub fn render<P, W, D>(&self, path: P, writer: &mut W, data: &D) -> Result<(), Error>
        where P: AsRef<Path>, W: Write, D: Serialize {
        let rendered = match self.try_render_template(&path, writer, data) {
            Ok(r) => r,
            Err(e) => {
                // Previously compiled template failed to render. Log
                // an error and force a reload.
                error!("Template render error: {:?}", e);
                false
            },
        };
        if !rendered {
            self.load_render_template(&path, writer, data)
        } else {
            Ok(())
        }
    }
}
