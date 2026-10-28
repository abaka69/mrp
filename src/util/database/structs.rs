pub struct RemotePackage {
    pub name: String,
    pub version: String,
    pub epoch: i32,
    pub description: String,
    pub groups: Vec<String>,
    pub url: String,
    pub license: Vec<String>,
    pub depends: Vec<String>,
    pub optional_depends: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub replaces: Vec<String>,
    pub sha512sum: String,
}

#[derive(Clone, PartiaEq, Eq, Hash)]
pub struct InstalledPackages {
    pub name: String,
    pub groups: Vec<String>,
    pub source: String,
    pub version: String,
    pub epoch: i32,
    pub installed_files: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub dependencies: Vec<String>,
}

#[derive(PartiaEq, Eq, Hash, Clone)]
pub struct Source {
    pub name: String,
    pub url: Option<String>,
}