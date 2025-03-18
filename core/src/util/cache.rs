use std::{
    fs::{create_dir_all, read_to_string, write},
    path::PathBuf,
};

use bevy_ecs::system::Resource;
use tower_lsp::Client;

#[derive(Debug, Clone, Resource)]
pub enum Cache {
    File(FileCache),
    None(NoCache),
}
impl Cache {
    pub async fn from_client(client: &Client) -> Self {
        Self::File(FileCache::from_client(client).await)
    }
    pub fn get_dir(&self, dir: &str) -> Self {
        match self {
            Self::File(file_cache) => Self::File(file_cache.get_dir(dir)),
            Self::None(no_cache) => Self::None(no_cache.get_dir(dir)),
        }
    }

    pub fn get_file(&self, file: &str) -> Option<String> {
        match self {
            Self::File(file_cache) => file_cache.get_file(file),
            Self::None(no_cache) => no_cache.get_file(file),
        }
    }

    pub fn write_file(&self, file: &str, content: &str) -> Option<()> {
        match self {
            Self::File(file_cache) => file_cache.write_file(file, content),
            Self::None(no_cache) => no_cache.write_file(file, content),
        }
    }
    pub fn path(&self) -> Option<&PathBuf> {
        match self {
            Cache::File(file_cache) => Some(&file_cache.path),
            Cache::None(_) => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NoCache;

impl NoCache {
    fn get_dir(&self, _dir: &str) -> Self {
        NoCache
    }

    fn get_file(&self, _file: &str) -> Option<String> {
        None
    }

    fn write_file(&self, _file: &str, _content: &str) -> Option<()> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct FileCache {
    path: PathBuf,
}

impl FileCache {
    pub fn get_dir(&self, dir: &str) -> Self {
        let new_path = self.path.join(dir);

        // TODO: make secure
        if !new_path.exists() {
            let _ = create_dir_all(&new_path);
        }

        Self { path: new_path }
    }

    pub fn get_file(&self, file: &str) -> Option<String> {
        let p = self.path.join(file);
        tracing::debug!("Reading from {:?}", p);
        read_to_string(p).ok()
    }

    pub fn write_file(&self, file: &str, content: &str) -> Option<()> {
        let p = self.path.join(file);
        tracing::debug!("Writing to {:?}", p);
        write(p, content.as_bytes()).ok()
    }
}

impl FileCache {
    pub fn new(dir: &PathBuf) -> Self {
        tracing::debug!("File Cache at {:?}", dir);

        if !dir.exists() {
            let _ = create_dir_all(&dir);
        }
        Self { path: dir.clone() }
    }
    pub async fn from_client(client: &Client) -> Self {
        Self::new(&get_cache_directory(client).await)
    }
}

// TODO: add user preferences folder
async fn get_cache_directory(client: &Client) -> PathBuf {
    // Use workspaceFolders if available (optional)
    if let Some(workspaces) = client.workspace_folders().await.unwrap_or(None) {
        if let Some(folder) = workspaces.first() {
            let uri = &folder.uri;
            if let Ok(path) = uri.to_file_path() {
                return path.join(".swls-cache");
            }
        }
    }

    std::env::temp_dir().join("swls")
}
