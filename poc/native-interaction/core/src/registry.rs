//! Registro de entradas autorizadas: a UI só recebe IDs opacos e nomes de exibição;
//! caminhos completos ficam no núcleo (ver docs/architecture.md, `inspectInputs`).

use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};
use std::path::{Path, PathBuf};

pub const MAX_INPUTS: usize = 1000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    File,
    Directory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputView {
    pub id: String,
    pub display_name: String,
    pub kind: Kind,
    pub was_symlink: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rejection {
    NotAbsolute,
    NotFound,
    Unsupported,
    LimitReached,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rejected {
    pub display_name: String,
    pub reason: Rejection,
}

#[derive(Debug, Default)]
pub struct Outcome {
    pub accepted: Vec<InputView>,
    pub rejected: Vec<Rejected>,
    pub duplicates: usize,
}

pub struct Registry {
    by_id: HashMap<String, PathBuf>,
    by_path: HashMap<PathBuf, String>,
    state: RandomState,
    counter: u64,
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
            by_path: HashMap::new(),
            state: RandomState::new(),
            counter: 0,
        }
    }

    fn next_id(&mut self) -> String {
        self.counter += 1;
        let mut h = self.state.build_hasher();
        h.write_u64(self.counter);
        format!("in_{:016x}", h.finish())
    }

    /// Recebe caminhos vindos do SO (não confiáveis). Resolve links e duplicatas.
    /// A política de links da PoC: aceita o alvo canônico e sinaliza `was_symlink`;
    /// a política definitiva é decisão de HF-02.
    pub fn add<I: IntoIterator<Item = PathBuf>>(&mut self, paths: I) -> Outcome {
        let mut out = Outcome::default();
        for p in paths {
            let name = display_name(&p);
            if !p.is_absolute() {
                out.rejected.push(Rejected {
                    display_name: name,
                    reason: Rejection::NotAbsolute,
                });
                continue;
            }
            let was_symlink = std::fs::symlink_metadata(&p)
                .map(|m| m.file_type().is_symlink())
                .unwrap_or(false);
            let canon = match std::fs::canonicalize(&p) {
                Ok(c) => c,
                Err(_) => {
                    out.rejected.push(Rejected {
                        display_name: name,
                        reason: Rejection::NotFound,
                    });
                    continue;
                }
            };
            let kind = match std::fs::metadata(&canon) {
                Ok(m) if m.is_file() => Kind::File,
                Ok(m) if m.is_dir() => Kind::Directory,
                _ => {
                    out.rejected.push(Rejected {
                        display_name: name,
                        reason: Rejection::Unsupported,
                    });
                    continue;
                }
            };
            if self.by_path.contains_key(&canon) {
                out.duplicates += 1;
                continue;
            }
            if self.by_id.len() >= MAX_INPUTS {
                out.rejected.push(Rejected {
                    display_name: name,
                    reason: Rejection::LimitReached,
                });
                continue;
            }
            let id = self.next_id();
            self.by_id.insert(id.clone(), canon.clone());
            self.by_path.insert(canon, id.clone());
            out.accepted.push(InputView {
                id,
                display_name: name,
                kind,
                was_symlink,
            });
        }
        out
    }

    /// Somente o núcleo chama isto; nunca exposto por IPC.
    pub fn resolve(&self, id: &str) -> Option<&Path> {
        self.by_id.get(id).map(|p| p.as_path())
    }

    pub fn clear(&mut self) {
        self.by_id.clear();
        self.by_path.clear();
    }

    pub fn len(&self) -> usize {
        self.by_id.len()
    }
    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }
}

pub fn display_name(p: &Path) -> String {
    p.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "(sem nome)".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn tmp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("hf-poc-{}-{}", name, std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn accepts_unicode_spaces_emoji_and_hides_full_path() {
        let d = tmp("uni");
        let names = ["foto café 😀.png", "a b\"c'd;$(x).txt", "日本語.jpg"];
        let paths: Vec<_> = names
            .iter()
            .map(|n| {
                let p = d.join(n);
                fs::write(&p, b"x").unwrap();
                p
            })
            .collect();
        let mut r = Registry::new();
        let out = r.add(paths);
        assert_eq!(out.accepted.len(), 3, "{:?}", out.rejected);
        for (v, n) in out.accepted.iter().zip(names) {
            assert_eq!(v.display_name, n);
            assert!(!v.id.contains('/') && !v.id.contains(n));
            assert!(r.resolve(&v.id).is_some());
        }
        fs::remove_dir_all(d).unwrap();
    }

    #[test]
    fn rejects_relative_missing_and_unknown_ids() {
        let mut r = Registry::new();
        let out = r.add([
            PathBuf::from("rel/x.png"),
            PathBuf::from("/nonexistent/hf/zzz"),
        ]);
        assert_eq!(out.rejected[0].reason, Rejection::NotAbsolute);
        assert_eq!(out.rejected[1].reason, Rejection::NotFound);
        assert!(r.resolve("in_forged").is_none());
        assert!(r.is_empty());
    }

    #[test]
    fn dedupes_same_file_including_via_symlink() {
        let d = tmp("dup");
        let f = d.join("a.png");
        fs::write(&f, b"x").unwrap();
        let mut r = Registry::new();
        let mut paths = vec![f.clone(), f.clone()];
        #[cfg(unix)]
        {
            let l = d.join("link.png");
            std::os::unix::fs::symlink(&f, &l).unwrap();
            paths.push(l);
        }
        let out = r.add(paths);
        assert_eq!(out.accepted.len(), 1);
        assert!(out.duplicates >= 1);
        fs::remove_dir_all(d).unwrap();
    }

    #[test]
    fn symlink_is_flagged_and_resolved_to_target() {
        #[cfg(unix)]
        {
            let d = tmp("sym");
            let f = d.join("real.txt");
            fs::write(&f, b"x").unwrap();
            let l = d.join("l.txt");
            std::os::unix::fs::symlink(&f, &l).unwrap();
            let mut r = Registry::new();
            let out = r.add([l]);
            assert!(out.accepted[0].was_symlink);
            assert_eq!(
                r.resolve(&out.accepted[0].id).unwrap(),
                fs::canonicalize(&f).unwrap()
            );
            fs::remove_dir_all(d).unwrap();
        }
    }

    #[test]
    fn directories_are_reported_as_such_and_limit_applies() {
        let d = tmp("lim");
        let mut r = Registry::new();
        let out = r.add([d.clone()]);
        assert_eq!(out.accepted[0].kind, Kind::Directory);
        let files: Vec<_> = (0..MAX_INPUTS + 5)
            .map(|i| {
                let p = d.join(format!("f{i}"));
                fs::write(&p, b"").unwrap();
                p
            })
            .collect();
        let out = r.add(files);
        assert_eq!(r.len(), MAX_INPUTS);
        assert!(out
            .rejected
            .iter()
            .all(|x| x.reason == Rejection::LimitReached));
        assert_eq!(out.rejected.len(), 6);
        fs::remove_dir_all(d).unwrap();
    }
}
