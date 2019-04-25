# Changelog

## 6.1.2

- Demangling support for Swift 5.
- Fix a performance regression in 6.1.1

## 6.1.1

- Expose PDB file names from PE object files.
- Fix incorrect CFI extraction from ELF files.
- Fix broken symcache lookups for certain optimized files.

## 6.1.0

- Support PDB file and line information.
- Support stack unwind info in PDB files (32-bit).
- Support stack unwind info in PE files (64-bit).
- Fix breakpad CFI generation for functions pushing machine frames.

## 6.0.6

- Add `normalize_code_id` in the Python package and C layer.
- Add `ByteView::map_file` to create a memory map directly from a file handle.
- Add size attribute to streams returned from Minidumps / UE4 crash reports.

## 6.0.5

- Normalize code identifiers to lowercase (#133).

## 6.0.4

- Exposes code identifiers and debug file names for minidumps in Python. Previously, this was only
  available in the Rust Crate.
- `ObjectLookup` now supports `code_file` and `debug_id` in in Python.

## 6.0.3

Re-release on crates.io.

## 6.0.2

**This release is broken on crates.io**

- Fix Rust features: The `serde` feature activated minidump and unreal unintentionally. This is
  addressed by providing separate features for modules with serde. See the Readme for more information.
- Include breakpad sources in `symbolic-minidump`.

## 6.0.0

This is a complete rewrite of `symbolic`. The aim of this release is to make the Rust version, the
C-API and the Python package more convenient to use in different scenarios. As a result, there have
been quite a few breaking changes.

**Breaking Changes:**

- `ByteViewHandle` has been replaced with the slightly safer type `SelfCell`. It allows to create a
  self-referential pair of an owning object and a derived object.
- `Archive` and `Object` are the new types to interface with debug information. There are also
  direct types exposed for Breakpad, ELF, MachO, PE and PDB; as well as traits to abstract over
  them.
- `SymCache` has a cleaner API, and the writing part has been moved to `SymCacheWriter`.
- Some common types have received better names: `ObjectKind` is now called `FileFormat`.
  `ObjectClass` is now called `ObjectKind`.
- Many more small signature changes, such as zero-copy return values or iterators instead of
  collections.

**New Features:**

- Initial support for PE and PDB is here. It is not complete yet, and will be expanded over the next
  releases.
- Symbol tables for ELF are now supported. On the bottom line, this will improve symbolication
  results on Linux.
- GNU-style compressed debug information (e.g. `.zdebug_info`) is now supported.
- Support for most of DWARF 5, thanks to the amazing work on the `gimli` crate.
- More lenient parsing of Breakpad symbols now handles certain edge cases more gracefully and gives
  much better error messages.
- More utilities to join or split paths from any platform.

**Bug Fixes:**

- Fix invalid function name resolution for certain DWARF files.
- Fix errors on DWARF files generated with LTO.
- Fix memory leaks when processing Minidumps from Python.
- Skip STAB symbol entries in MachO files, potentially leading to wrong function names.
- Do not error on "negative" line numbers in Breakpad symbols.

**Internal Changes:**

- Greatly simplified build process and better documentation for the C library.
- Improved test suite, docs and READMEs. While there can never be enough tests, this is a
  significant step to improving the overall quality of symbolic.
- Automatic cloning of submodules during the build. This should make it easier to start developing
  on `symbolic`.
