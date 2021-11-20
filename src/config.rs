use anyhow::Result;
use classic_crypto::lang::Lang;
use directories::ProjectDirs;
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io::Read, path::PathBuf};

use crate::error::CipherError;

const QUALIFIER: &str = "";
const ORGANISATION: &str = "";
const APPLICATION: &str = "cipher";
const CONFIG_FILENAME: &str = "config.toml";
const LANG_DIR: &str = "lang";

/// Abstraction for config directory and language files
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CipherConfig {
    selected_lang: Option<String>,
    lang_map: HashMap<String, LangMeta>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LangMeta {
    // used as the filename
    pub id: usize,
    // primary alphabet length
    pub primary: usize,
    // lengths of the alphabets
    pub alphabets: Vec<usize>,
}

impl CipherConfig {
    /// Sets the primary alphabet for the provided lang,
    /// or if `name` is `None` then sets for the selected lang.
    pub fn set_primary_alph(&mut self, name: Option<String>, length: usize) -> Result<()> {
        let name = match name.or_else(|| self.selected_lang().map(|s| s.to_owned())) {
            Some(name) => name,
            None => return Err(CipherError::NoLangSelected.into()),
        };

        let mut lang = self.load_lang(&name)?;
        lang.set_primary(length)?;

        let mut meta = self.lang_map.get_mut(&name).unwrap();
        meta.primary = length;

        let path = Self::lang_file_path(self.lang_map[&*name].id)?;
        let bytes = bincode::serialize(&lang)?;
        std::fs::write(path, &bytes)?;

        Ok(())
    }
    /// Gets the currently selected lang
    pub fn selected_lang(&self) -> Option<&str> {
        self.selected_lang.as_deref()
    }
    /// Loads the lang given by `name`, or if that fails, the selected lang
    pub fn load_lang_or_selected(&self, name: Option<String>) -> Result<Lang> {
        match name {
            Some(name) => match self.load_lang(&name) {
                Ok(lang) => Ok(lang),
                Err(_) => self.load_selected(),
            },
            None => self.load_selected(),
        }
    }
    /// Metadata for the language
    pub fn lang_meta(&self, name: &str) -> Option<&LangMeta> {
        self.lang_map.get(name.trim())
    }
    /// Loads the preferred lang if it is set
    pub fn load_selected(&self) -> Result<Lang> {
        self.load_lang(
            self.selected_lang
                .as_ref()
                .ok_or(CipherError::NoLangSelected)?,
        )
    }
    /// Adds a language file by name
    pub fn add_lang(&mut self, name: String, lang: &Lang) -> Result<()> {
        // check whether name already exists
        let name = name.trim();
        if self.lang_map.contains_key(name) {
            return Err(CipherError::LangAlreadyExists.into());
        }

        // insert at new id
        let id = self.lang_map.values().map(|m| m.id).max().unwrap_or(0);
        self.lang_map.insert(
            name.to_string(),
            LangMeta {
                id,
                primary: lang.primary_len(),
                alphabets: lang.alphabets().map(|a| a.alphabet_len()).collect(),
            },
        );

        // create directory
        fs::create_dir_all(Self::lang_dir()?)?;
        fs::write(Self::lang_file_path(id)?, bincode::serialize(lang)?)?;

        Ok(())
    }
    /// Deletes a language file by name
    pub fn rm_lang(&mut self, name: &str) -> Result<()> {
        // check whether name exists
        let name = name.trim();
        let id = self
            .lang_map
            .get(name)
            .map(|m| m.id)
            .ok_or(CipherError::LangNotFound)?;

        // remove from cfg
        self.lang_map.remove(name);

        // delete file
        fs::remove_file(Self::lang_file_path(id)?)?;

        // remove from preferred
        if self.selected_lang.as_deref() == Some(name) {
            self.selected_lang = None;
        }

        Ok(())
    }
    /// Attempts to load a language file, given its name
    pub fn load_lang(&self, name: &str) -> Result<Lang> {
        // check whether name exists
        let name = name.trim();
        let id = self
            .lang_map
            .get(name)
            .map(|m| m.id)
            .ok_or(CipherError::LangNotFound)?;

        let bytes = fs::read(Self::lang_file_path(id)?)?;
        let lang = bincode::deserialize(&bytes)?;

        Ok(lang)
    }

    /// An iterator over language names
    pub fn lang_names(&self) -> impl Iterator<Item = &String> {
        self.lang_map.keys()
    }
    /// Sets the default selected language
    pub fn set_selected(&mut self, name: &str) -> Result<()> {
        let name = name.trim();

        if !self.lang_map.contains_key(name) {
            Err(CipherError::LangNotFound.into())
        } else {
            self.selected_lang = Some(name.to_string());
            Ok(())
        }
    }

    /// Loads the config file, returning a default version if it could
    /// not be found
    pub fn load() -> Self {
        Self::load_safe().unwrap_or_default()
    }
    /// Saves the config file
    pub fn save(&self) -> Result<()> {
        trace!("saving config");

        let cfg_path = Self::config_path()?;
        if !cfg_path.exists() {
            debug!("config path did not exist");
            fs::create_dir_all(&cfg_path)?;
        }

        fs::write(cfg_path.join(CONFIG_FILENAME), toml::to_vec(self)?)?;

        Ok(())
    }

    fn load_safe() -> Result<Self> {
        trace!("loading config");
        let cfg_path = Self::config_path()?;

        if !cfg_path.exists() {
            return Err(anyhow::anyhow!("config path did not exist"));
        }

        let mut file = fs::OpenOptions::new()
            .read(true)
            .open(cfg_path.join(CONFIG_FILENAME))?;
        let mut buf = Vec::new();

        file.read_to_end(&mut buf)?;

        Ok(toml::from_slice(&buf)?)
    }

    fn lang_dir() -> Result<PathBuf> {
        Self::config_path().map(|p| p.join(LANG_DIR))
    }
    fn lang_file_path(id: usize) -> Result<PathBuf> {
        Self::lang_dir().map(|cfg| cfg.join(format!("{}.bin", id.to_string())))
    }
    fn config_path() -> Result<PathBuf> {
        ProjectDirs::from(QUALIFIER, ORGANISATION, APPLICATION)
            .map(|dirs| dirs.config_dir().to_owned())
            .ok_or_else(|| anyhow::anyhow!("Failed to determine config path"))
    }
}
