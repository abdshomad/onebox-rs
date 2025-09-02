# Changelog Directory

This directory contains the changelog files for the onebox-rs project, organized by version and following the [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format.

## File Structure

```
changelog/
├── README.md           # This file - explains the changelog structure
├── CHANGELOG.md        # Main changelog index and overview
├── TEMPLATE.md         # Template for creating new version changelogs
├── UNRELEASED.md       # Upcoming changes and planned features
└── 0.1.0.md           # Version 0.1.0 release notes
```

## Changelog Organization

### Main Changelog
- **`CHANGELOG.md`**: The primary changelog file that provides an overview of all versions and links to individual version files.

### Version-Specific Files
- **`0.1.0.md`**: Detailed changelog for version 0.1.0 (Initial Release)
- **`UNRELEASED.md`**: Tracks upcoming changes and planned features for future releases

### Template
- **`TEMPLATE.md`**: Template file that can be copied and modified for new version releases

## Adding New Versions

When creating a new version release:

1. **Copy the template**: `cp TEMPLATE.md VERSION.md`
2. **Update the template**: Replace `VERSION` with the actual version number (e.g., `0.2.0`)
3. **Fill in the details**: Update all sections with relevant information
4. **Update main changelog**: Add the new version to `CHANGELOG.md`
5. **Update links**: Ensure all relative links are correct

## Changelog Format

Each version follows the standard Keep a Changelog format:

- **Added** for new features
- **Changed** for changes in existing functionality
- **Deprecated** for soon-to-be removed features
- **Removed** for now removed features
- **Fixed** for any bug fixes
- **Security** in case of vulnerabilities

## Version Naming Convention

- **Major.Minor.Patch** format (e.g., 0.1.0, 0.2.0, 1.0.0)
- **Pre-release versions** can use suffixes (e.g., 0.1.0-alpha, 0.1.0-beta)
- **Release candidates** use -rc suffix (e.g., 0.1.0-rc1)

## Maintenance

- Keep the main `CHANGELOG.md` updated with new versions
- Ensure all links between files are working correctly
- Remove entries from `UNRELEASED.md` when they are released
- Update the template if the format needs to evolve

## Links

- [Main Changelog](./CHANGELOG.md)
- [Unreleased Changes](./UNRELEASED.md)
- [Version 0.1.0](./0.1.0.md)
- [Template](./TEMPLATE.md)
- [Project Root](../README.md)
