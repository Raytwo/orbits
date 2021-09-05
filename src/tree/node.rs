use std::path::{Path, PathBuf};

use super::{TreeError, TreeNode};

/// The `Node` structure is used to represent a singular path in the filesystem
/// This structure is not responsible for determining if a path is a file or a directory
pub(crate) struct Node {
    pub name: String,
    pub local_path: PathBuf,
    pub root_path: PathBuf
}

impl TreeNode for Node {
    type TreeKey = String;
    type ErrorType = TreeError;

    fn get_key(&self) -> Self::TreeKey {
        self.name.clone()
    }
}

impl Node {
    fn get_file_name(path: &Path) -> Result<String, TreeError> {
        match path.file_name() {
            Some(name) => {
                if let Some(name) = name.to_str() {
                    Ok(name.to_string())
                } else {
                    Err(TreeError::InvalidFileName(path.to_path_buf()))
                }
            },
            None => {
                Err(TreeError::NoFileName(path.to_path_buf()))
            }
        }
    }

    /// Returns a `Node` structure setup with the `root_path`, `local_path`, and key.
    /// Important to note that the local path is immutable once the struct
    /// This is because changing the local path will change the key.
    pub fn new<A: AsRef<Path>, B: AsRef<Path>>(root_path: A, local_path: B) -> Result<Self, TreeError> {
        let local_path = local_path.as_ref().to_path_buf();
        let root_path = root_path.as_ref().to_path_buf();

        let name = Self::get_file_name(&local_path)?;
        Ok(Self {
            name,
            local_path,
            root_path
        })
    }

    /// Changes the root this `Node` is based in. Changing the root has no impact over its location in the file tree.
    pub fn change_root<A: AsRef<Path>>(&mut self, new_root: A) -> Result<(), TreeError> {
        let new_root = new_root.as_ref();

        self.root_path = new_root.to_path_buf();
        Ok(())
    }
}