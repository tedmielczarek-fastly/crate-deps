# crate-deps

Determine the full transitive set of new dependencies for a cargo workspace between versions.

# Usage

Run `cargo metadata` on the two versions to be compared:

```shell
git checkout v1
cargo metadata > /tmp/v1.metadata.json

git checkout v2
cargo metadata > /tmp/v2.metadata.json
```

Now run `crate-deps`, passing the two versions as commandline arguments. The first one specified is taken to be the baseline version, the second the new version:

```shell
crate-deps /tmp/v1.metadata.json /tmp/v2.metadata.json
```

The output will be a list of crates added in the new version.
