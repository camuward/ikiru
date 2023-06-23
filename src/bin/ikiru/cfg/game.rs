use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Seek};
use std::path::{Path, PathBuf};
use std::time::Instant;

use rayon::prelude::*;
use walkdir::WalkDir;

use ikiru::game::{AppXml, MetaXml, TitleId};

#[derive(Debug, Clone)]
pub struct GameLibrary {
    /// The directories to search for games.
    search_paths: HashSet<PathBuf>,
    /// The games found in the search paths.
    entries: BTreeMap<TitleId, GameEntry>,
}

#[derive(Debug, Clone)]
pub struct GameEntry {
    path: GamePath,
    pub title: TitleId,
    pub meta: MetaXml,
    pub app: AppXml,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GamePath {
    Folder(PathBuf),
    Wux(PathBuf),
    Wud(PathBuf),
}

impl GameLibrary {
    pub fn new(search_paths: impl IntoIterator<Item = impl AsRef<Path>>) -> eyre::Result<Self> {
        let search_paths = search_paths
            .into_iter()
            .map(|path| path.as_ref().canonicalize())
            .collect::<io::Result<HashSet<_>>>()?;

        let game_paths: HashSet<GamePath> = search_paths
            .iter()
            .flat_map(|dir| {
                WalkDir::new(dir)
                    .follow_links(true)
                    .max_depth(3)
                    .into_iter()
            })
            .filter_map(|subdir| {
                let subdir = subdir.ok()?;
                let path = subdir.path();

                let file_name = path.file_name().and_then(|s| s.to_str());
                let extension = path.extension().and_then(|s| s.to_str());

                match (file_name, extension) {
                    (Some("meta" | "code" | "content"), None) if path.is_dir() => {
                        Some(GamePath::Folder(path.parent()?.canonicalize().ok()?))
                    }
                    (Some(_name), Some("wud")) if path.is_file() => {
                        Some(GamePath::Wud(path.canonicalize().ok()?))
                    }
                    (Some(_name), Some("wux")) if path.is_file() => {
                        Some(GamePath::Wux(path.canonicalize().ok()?))
                    }
                    _ => None,
                }
            })
            .collect();

        let entries: BTreeMap<TitleId, GameEntry> = game_paths
            .into_par_iter()
            .try_fold(
                || BTreeMap::new(),
                |mut map, path| -> eyre::Result<_> {
                    let entry = GameEntry::new(path)?;

                    if map.contains_key(&entry.title) {
                        map.insert(entry.title, entry);
                        Ok(map)
                    } else {
                        Ok(map)
                    }
                },
            )
            .try_reduce(
                || BTreeMap::new(),
                |mut map1, map2| {
                    map1.extend(map2);
                    Ok(map1)
                },
            )?;

        Ok(Self {
            search_paths,
            entries,
        })
    }

    pub fn entries(&self) -> &BTreeMap<TitleId, GameEntry> {
        &self.entries
    }
}

impl GameEntry {
    pub fn new(path: GamePath) -> eyre::Result<Self> {
        let meta: MetaXml = quick_xml::de::from_reader(path.get_asset("meta/meta.xml".as_ref())?)?;
        let app: AppXml = quick_xml::de::from_reader(path.get_asset("code/app.xml".as_ref())?)?;

        let title = meta.title_id.parse().or_else(|_| app.title_id.parse())?;

        Ok(Self {
            path,
            title,
            meta,
            app,
        })
    }

    pub fn get_asset(&self, path: impl AsRef<Path>) -> eyre::Result<impl Read + BufRead + Seek> {
        self.path.get_asset(path.as_ref())
    }
}

impl GamePath {
    fn get_asset(&self, path: &Path) -> eyre::Result<impl Read + BufRead + Seek> {
        match self {
            GamePath::Folder(base) => {
                let path = base.join(path);
                let file = File::open(path)?;

                Ok(BufReader::new(file))
            }
            _ => todo!(),
        }
    }
}
