use std::{
    env,
    path::{Path, PathBuf},
};

use hex_literal::hex;

const ORT_EXTRACT_DIR: &str = "onnx-runtime";
const LINUX_X64_CHECKSUM: [u8; 32] =
    hex!("aac5f22695168f089af0bfd129b5ac2bad86a3cfaba0457a536e21f30f0c155a");
const LINUX_AARCH64_CHECKSUM: [u8; 32] =
    hex!("7dded316f8a80c7bf5f91a0c9a4ab8ce854530c8ece40828a448c06e7e8fc453");
const MACOS_UNIVERSAL2_CHECKSUM: [u8; 32] =
    hex!("ecb7651c216fe6ffaf4c578e135d98341bc5bc944c5dc6b725ef85b0d7747be0");
const WINDOWS_X64_CHECKSUM: [u8; 32] =
    hex!("07c58b7842e288caa7610b16a9feac13e16b4d5a2f0e938de064b3cc83928107");

#[cfg(feature = "onnx")]
fn check_checksum(buf: &[u8], file_hash: &[u8]) {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(buf);

    let hash_result = hasher.finalize();

    if hash_result[..] != file_hash[..] {
        panic!("Expected file hash does not match the downloaded file")
    }
}

#[cfg(feature = "onnx")]
fn fetch_file(source_url: &str) -> Vec<u8> {
    let resp = ureq::get(source_url)
        .timeout(std::time::Duration::from_secs(1800))
        .call()
        .unwrap_or_else(|err| panic!("failed to download {source_url}: {err:?}"));

    let len = resp
        .header("Content-Length")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap();
    let mut reader = resp.into_reader();
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    assert_eq!(buffer.len(), len);
    buffer
}

#[cfg(feature = "onnx")]
fn extract_tgz(buf: &[u8], output: &Path) {
    let buf: std::io::BufReader<&[u8]> = std::io::BufReader::new(buf);
    let tar = flate2::read::GzDecoder::new(buf);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(output).unwrap();
}

#[cfg(feature = "onnx")]
fn extract_zip(buf: &[u8], output: &Path) {
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(buf)).unwrap();
    archive.extract(output).unwrap();
}

enum ArchiveType {
    Tgz,
    Zip,
}

fn main() {
    if !cfg!(feature = "onnx") {
        return;
    }

    let (archive_type, url, file_hash) = match env::var("TARGET").unwrap().as_str() {
        "x86_64-apple-darwin" => (ArchiveType::Tgz, "https://github.com/supertone-inc/onnxruntime-build/releases/download/v1.15.1/onnxruntime-osx-universal2-static_lib-1.15.1.tgz", MACOS_UNIVERSAL2_CHECKSUM),
        "x86_64-unknown-linux-gnu" => (ArchiveType::Tgz, "https://github.com/supertone-inc/onnxruntime-build/releases/download/v1.15.1/onnxruntime-linux-x64-static_lib-1.15.1.tgz", LINUX_X64_CHECKSUM),
        "aarch64-unknown-linux-gnu" => (ArchiveType::Tgz, "https://github.com/supertone-inc/onnxruntime-build/releases/download/v1.15.1/onnxruntime-linux-aarch64-static_lib-1.15.1.tgz", LINUX_AARCH64_CHECKSUM),
        "x86_64-pc-windows-msvc" => (ArchiveType::Zip,"https://github.com/supertone-inc/onnxruntime-build/releases/download/v1.15.1/onnxruntime-win-x64-static_lib-1.15.1.zip", WINDOWS_X64_CHECKSUM),
        "x86_64-pc-windows-gnu" => (ArchiveType::Zip, "https://github.com/supertone-inc/onnxruntime-build/releases/download/v1.15.1/onnxruntime-win-x64-static_lib-1.15.1.zip", WINDOWS_X64_CHECKSUM),
        t => panic!("Unsupported target:  {t}"),
    };

    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(ORT_EXTRACT_DIR);
    if !out_dir.exists() {
        let buf = fetch_file(url);

        check_checksum(buf.as_slice(), &file_hash);

        match archive_type {
            ArchiveType::Tgz => extract_tgz(&buf, &out_dir),
            ArchiveType::Zip => extract_zip(&buf, &out_dir),
        }
    }
}
