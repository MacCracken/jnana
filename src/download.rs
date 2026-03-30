//! Source download and verification.
//!
//! Behind the `download` feature flag. Provides blocking download with
//! progress reporting and SHA-256 checksum verification.

use crate::error::{JnanaError, Result};
use crate::source::Source;
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::{Path, PathBuf};

/// Verify a file's SHA-256 checksum against an expected value.
///
/// The expected string may optionally start with `sha256:`.
pub fn verify_checksum(path: &Path, expected: &str) -> Result<bool> {
    let expected = expected.strip_prefix("sha256:").unwrap_or(expected);
    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    let hash = format!("{:x}", hasher.finalize());
    Ok(hash == expected)
}

/// Compute the SHA-256 checksum of a file.
pub fn compute_checksum(path: &Path) -> Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(format!("sha256:{:x}", hasher.finalize()))
}

/// Download a source to a destination directory.
///
/// Returns the path to the downloaded file. Calls `progress` with
/// `(bytes_downloaded, total_bytes)` — total is 0 if unknown.
///
/// This is a placeholder that currently copies from a local path for
/// testing. Full HTTP download will be added when a networking dep
/// is available.
pub fn download_source<F>(source: &Source, dest: &Path, mut progress: F) -> Result<PathBuf>
where
    F: FnMut(u64, u64),
{
    if source.url.is_empty() {
        return Err(JnanaError::Source(format!(
            "no download URL for source '{}'",
            source.id
        )));
    }

    // For local file:// URLs or paths, copy directly
    let src_path = if let Some(path) = source.url.strip_prefix("file://") {
        PathBuf::from(path)
    } else if Path::new(&source.url).exists() {
        PathBuf::from(&source.url)
    } else {
        return Err(JnanaError::Source(format!(
            "HTTP download not yet implemented — source '{}' requires a network fetch from {}",
            source.id, source.url
        )));
    };

    let file_name = src_path
        .file_name()
        .ok_or_else(|| JnanaError::Source("invalid source path".into()))?;
    let dest_path = dest.join(file_name);

    let metadata = std::fs::metadata(&src_path)?;
    let total = metadata.len();
    let mut reader = std::fs::File::open(&src_path)?;
    let mut writer = std::fs::File::create(&dest_path)?;
    let mut buf = [0u8; 8192];
    let mut downloaded: u64 = 0;

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        std::io::Write::write_all(&mut writer, &buf[..n])?;
        downloaded += n as u64;
        progress(downloaded, total);
    }

    tracing::info!(
        source = %source.id,
        dest = %dest_path.display(),
        bytes = downloaded,
        "download complete"
    );

    // Verify checksum if available
    if let Some(checksum) = &source.checksum
        && checksum != "sha256:placeholder"
    {
        if !verify_checksum(&dest_path, checksum)? {
            return Err(JnanaError::IntegrityFailed {
                name: source.id.clone(),
                reason: "SHA-256 checksum mismatch".into(),
            });
        }
        tracing::info!(source = %source.id, "checksum verified");
    }

    Ok(dest_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Domain;
    use crate::source::SourceKind;
    use std::io::Write;

    #[test]
    fn compute_and_verify_checksum() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");
        {
            let mut f = std::fs::File::create(&path).unwrap();
            f.write_all(b"hello world").unwrap();
        }
        let checksum = compute_checksum(&path).unwrap();
        assert!(checksum.starts_with("sha256:"));
        assert!(verify_checksum(&path, &checksum).unwrap());
        assert!(!verify_checksum(&path, "sha256:0000").unwrap());
    }

    #[test]
    fn verify_with_and_without_prefix() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");
        std::fs::write(&path, b"test data").unwrap();

        let checksum = compute_checksum(&path).unwrap();
        let hash = checksum.strip_prefix("sha256:").unwrap();

        // Both forms should work
        assert!(verify_checksum(&path, &checksum).unwrap());
        assert!(verify_checksum(&path, hash).unwrap());
    }

    #[test]
    fn download_local_file() {
        let src_dir = tempfile::tempdir().unwrap();
        let dest_dir = tempfile::tempdir().unwrap();
        let src_path = src_dir.path().join("data.zim");
        std::fs::write(&src_path, b"fake zim content").unwrap();

        let source = Source::new(
            "test",
            "Test",
            Domain::Medicine,
            SourceKind::Zim,
            format!("file://{}", src_path.display()),
            1,
        );

        let mut progress_calls = 0u32;
        let result = download_source(&source, dest_dir.path(), |_, _| progress_calls += 1);
        let dest_path = result.unwrap();
        assert!(dest_path.exists());
        assert_eq!(std::fs::read(&dest_path).unwrap(), b"fake zim content");
    }

    #[test]
    fn download_with_checksum_verification() {
        let src_dir = tempfile::tempdir().unwrap();
        let dest_dir = tempfile::tempdir().unwrap();
        let src_path = src_dir.path().join("data.pdf");
        std::fs::write(&src_path, b"verified content").unwrap();

        let checksum = compute_checksum(&src_path).unwrap();
        let source = Source::new(
            "verified",
            "Verified",
            Domain::Medicine,
            SourceKind::Pdf,
            format!("file://{}", src_path.display()),
            1,
        )
        .with_checksum(&checksum);

        let result = download_source(&source, dest_dir.path(), |_, _| {});
        assert!(result.is_ok());
    }

    #[test]
    fn download_bad_checksum_fails() {
        let src_dir = tempfile::tempdir().unwrap();
        let dest_dir = tempfile::tempdir().unwrap();
        let src_path = src_dir.path().join("data.pdf");
        std::fs::write(&src_path, b"some content").unwrap();

        let source = Source::new(
            "bad",
            "Bad",
            Domain::Medicine,
            SourceKind::Pdf,
            format!("file://{}", src_path.display()),
            1,
        )
        .with_checksum("sha256:0000000000000000000000000000000000000000000000000000000000000000");

        let result = download_source(&source, dest_dir.path(), |_, _| {});
        assert!(result.is_err());
    }

    #[test]
    fn download_no_url_fails() {
        let dest_dir = tempfile::tempdir().unwrap();
        let source = Source::new("empty", "Empty", Domain::Medicine, SourceKind::Pdf, "", 1);
        let result = download_source(&source, dest_dir.path(), |_, _| {});
        assert!(result.is_err());
    }
}
