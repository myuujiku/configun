// Copyright 2023 myujiku (https://github.com/myuujiku)

use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{self, Error, ErrorKind, Write};
use std::path::Path;
use std::sync::mpsc;
use std::thread;

use sha2::{Digest, Sha256};
use zstd::stream::encode_all;

use crate::{utils::project_root, EXTENSION};

#[derive(clap::Args)]
pub struct Args;

impl Args {
    pub fn eval(&self) {
        let project_root = project_root::get();

        if let Ok(root) = project_root {
            let result = read_bullet_dir(&root);

            if let Ok(data_data) = result {
            } else if let Some(error) = result.unwrap_err().into_inner() {
                println!("Error: {}", error);
            }
        } else if let Some(error) = project_root.unwrap_err().into_inner() {
            println!("Error: {}", error);
        }
    }
}

fn read_bullet_dir(path: &Path) -> io::Result<HashMap<String, BulletData>> {
    let mut result = HashMap::new();
    let mut handles = Vec::new();

    let target = path.join(".pack");
    create_dir_all(&target)?;

    // Iterate over entries in `root/bullets`.
    if let Ok(read_dir) = path.join("bullets").read_dir() {
        let (tx, rx) = mpsc::channel();

        for entry in read_dir {
            let tx = tx.clone();
            let handle = thread::spawn(move || {
                if let Ok(entry) = entry {
                    // Get name of the entry as a String.
                    if let Ok(file_name) = entry.file_name().into_string() {
                        // Try to compress the entry if it's a valid bullet directory.
                        tx.send(Some((compress_bullet(&entry.path(), 0), file_name)))
                            .expect("Failed to send");
                    } else {
                        println!("Skipped: Invalid name.");
                    }
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Failed to join");
        }

        tx.send(None).expect("Failed to send");

        for received in rx {
            if let Some(received) = received {
                let (res, file_name) = received;
                if let Ok((data, compressed)) = res {
                    // Write to file.
                    let file = File::create(target.join(&file_name).with_extension(EXTENSION));
                    if let Ok(mut file) = file {
                        if let Err(error) = file.write(&compressed) {
                            println!(
                                "Skipped `{}`: Failed to write to `{}` ({}).",
                                file_name,
                                target.join(&file_name).display(),
                                error,
                            );
                        }
                    } else if let Some(error) = file.unwrap_err().into_inner() {
                        println!(
                            "Skipped `{}`: Failed to create `{}` ({}).",
                            file_name,
                            target.join(&file_name).display(),
                            error,
                        );
                    }

                    result.insert(file_name, data);
                } else if let Some(warning) = res.unwrap_err().into_inner() {
                    println!("Skipped `{}`: {}", file_name, warning);
                }
            } else {
                break;
            }
        }
    }

    println!("{result:#?}");

    Ok(result)
}

fn compress_bullet(path: &Path, ratio: i32) -> io::Result<(BulletData, Vec<u8>)> {
    if path.is_dir() {
        if path.join("bullet.toml").is_file() {
            // Archive directory as tar.
            let mut tar = tar::Builder::new(Vec::new());
            tar.append_dir_all(
                path.file_name()
                    .ok_or(Error::new(ErrorKind::Other, "Failed to read file name"))?,
                path,
            )?;
            let tar = tar.into_inner()?;

            // Compress tar with zstd.
            let compressed = encode_all(&*tar, ratio)?;

            // Calculate sha256sum.
            let mut hasher = Sha256::new();
            hasher.update(&compressed);
            let result = hasher.finalize();
            let sha = result[..]
                .as_ref()
                .iter()
                .map(|b| format!("{:x}", b))
                .collect::<String>();

            Ok((BulletData { sha }, compressed))
        } else {
            Err(Error::new(ErrorKind::Other, "`bullet.toml` not found."))
        }
    } else {
        Err(Error::new(ErrorKind::Other, "Not a directory."))
    }
}

#[derive(Debug)]
struct BulletData {
    sha: String,
}
