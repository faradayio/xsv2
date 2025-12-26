# How to release xsv2

This document describes the release process for xsv2.

## Prerequisites

- Write access to the GitHub repository
- Local clone of the repository
- Git configured with appropriate credentials

## Release process

### 1. Prepare the release

#### Update version in Cargo.toml

Update the version number in `Cargo.toml`:

```toml
version = "0.14.0"  #:version
```

Remove the `-pre1` or any other pre-release suffix when making a stable release.

#### Update CHANGELOG.md

1. Move all items from the `[Unreleased]` section to a new version section
2. Add the version number and release date
3. Create a new empty `[Unreleased]` section at the top

Example:

```markdown
## [Unreleased]

## [0.14.0] - 2025-12-26

### Added

- Feature X
- Feature Y

### Changed

- Change A
- Change B
```

#### Test locally

Before releasing, make sure everything works:

```bash
cargo test --verbose
cargo clippy --verbose -- -D warnings
cargo build --release
```

#### Commit changes

Commit the version bump and changelog updates:

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "Release version 0.14.0"
```

### 2. Create and push the release tag

Create an annotated tag with the version number (prefixed with `v`):

```bash
git tag -a v0.14.0 -m "Release version 0.14.0"
```

Push the tag to GitHub:

```bash
git push origin v0.14.0
```

**Important:** Make sure to push the commit first if you haven't already:

```bash
git push origin master
git push origin v0.14.0
```

### 3. Wait for the automated release workflow

When you push a tag starting with `v`, GitHub Actions will automatically:

1. **Validate** that the tag version matches the version in `Cargo.toml`
2. **Test** the code by running:
   - `cargo test`
   - `cargo clippy`
3. **Build** release binaries for all supported platforms:
   - Linux (x86_64-unknown-linux-musl)
   - macOS Intel (x86_64-apple-darwin)
   - macOS Apple Silicon (aarch64-apple-darwin)
   - Windows (x86_64-pc-windows-msvc)
4. **Create a GitHub Release** with:
   - Auto-generated release notes
   - Attached binary archives for all platforms

You can monitor the progress at: `https://github.com/faradayio/xsv2/actions`

### 4. Verify the release

Once the workflow completes:

1. Check the [releases page](https://github.com/faradayio/xsv2/releases) to verify the new release is published
2. Verify all platform binaries are attached
3. Review the auto-generated release notes
4. Optionally, edit the release notes to add any additional context or highlights

### 5. Post-release tasks

#### Update version for next development cycle

After a release, it's a good practice to update the version in `Cargo.toml` to the next version with a pre-release suffix:

```toml
version = "0.15.0-pre1"  #:version
```

Commit this change:

```bash
git add Cargo.toml
git commit -m "Bump version to 0.15.0-pre1 for development"
git push origin master
```

## Troubleshooting

### Release workflow fails on version validation

If the workflow fails with a version mismatch error, it means the tag version doesn't match the version in `Cargo.toml`. To fix:

1. Delete the tag locally and remotely:
   ```bash
   git tag -d v0.14.0
   git push origin :refs/tags/v0.14.0
   ```
2. Update the version in `Cargo.toml` to match
3. Commit and push the change
4. Create and push the tag again

### Release workflow fails on tests

If tests fail:

1. The automated release workflow will stop and not create a release
2. Fix the failing tests
3. Delete the tag (as described above)
4. Commit and push the fix
5. Create and push the tag again

### Need to delete a release

If you need to delete a release:

1. Go to the [releases page](https://github.com/faradayio/xsv2/releases)
2. Click on the release
3. Click "Delete this release"
4. Optionally delete the tag:
   ```bash
   git tag -d v0.14.0
   git push origin :refs/tags/v0.14.0
   ```

## Release checklist

Use this checklist when making a release:

- [ ] Update version in `Cargo.toml` (remove pre-release suffix)
- [ ] Update `CHANGELOG.md` with release notes
- [ ] Test locally: `cargo test && cargo clippy -- -D warnings`
- [ ] Commit changes
- [ ] Create annotated tag: `git tag -a vX.Y.Z -m "Release version X.Y.Z"`
- [ ] Push commit and tag to GitHub
- [ ] Monitor GitHub Actions workflow
- [ ] Verify release on GitHub
- [ ] Update version in `Cargo.toml` for next development cycle
- [ ] Commit and push version bump

## Version numbering

xsv2 uses [Semantic Versioning](https://semver.org/):

- **Major version** (X.0.0): Breaking changes
- **Minor version** (0.X.0): New features, backward compatible
- **Patch version** (0.0.X): Bug fixes, backward compatible

During development, use pre-release suffixes like `-pre1`, `-pre2`, etc.
