use anyhow::{bail, Result};
use guppy::{
    graph::{DependencyDirection, PackageGraph, PackageSet},
    CargoMetadata,
};
use std::collections::BTreeSet;
use std::path::Path;

fn without_dev(package_graph: &PackageGraph) -> PackageSet {
    package_graph
        .query_workspace()
        .resolve_with_fn(|_query, link| !link.dev_only())
}

fn graph_from_metadata_file(path: impl AsRef<Path>) -> Result<PackageGraph> {
    let path = path.as_ref();
    let metadata_json = std::fs::read_to_string(path)?;
    let metadata = CargoMetadata::parse_json(&metadata_json)?;
    Ok(metadata.build_graph()?)
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let (base_metadata_path, new_metadata_path) = match &args[..] {
        [a, b] => (a, b),
        _ => bail!("Usage: crate-deps <path to base metadata.json> <path to new metadata.json>"),
    };
    let base_package_graph = graph_from_metadata_file(&base_metadata_path)?;
    let base_packages: BTreeSet<_> = without_dev(&base_package_graph)
        .package_ids(DependencyDirection::Forward)
        .collect();
    let new_package_graph = graph_from_metadata_file(&new_metadata_path)?;
    let new_packages: BTreeSet<_> = without_dev(&new_package_graph)
        .package_ids(DependencyDirection::Forward)
        .collect();
    // Get just the new packages that were introduced.
    for p in new_packages.difference(&base_packages) {
        println!("{p}");
    }

    Ok(())
}
