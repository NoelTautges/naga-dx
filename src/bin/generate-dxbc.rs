use anyhow::{Context, Result};
use find_winsdk::{SdkInfo, SdkVersion};
use glob::glob;
use indicatif::ParallelProgressIterator;
use owo_colors::OwoColorize;
use rayon::prelude::*;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

#[derive(Clone, Copy, Debug)]
enum ShaderType {
    Vertex,
    Pixel,
}

impl ShaderType {
    fn get_entry_point(&self) -> &str {
        match self {
            Self::Vertex => "VSMain",
            Self::Pixel => "PSMain",
        }
    }
}

impl fmt::Display for ShaderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Vertex => "vs",
                Self::Pixel => "ps",
            }
        )
    }
}

#[derive(Clone, Copy, Debug)]
enum ShaderModel {
    V4_1,
    V5_1,
}

impl fmt::Display for ShaderModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::V4_1 => "4_1",
                Self::V5_1 => "5_1",
            }
        )
    }
}

#[derive(Clone, Copy, Debug)]
struct Shader {
    ty: ShaderType,
    model: ShaderModel,
}

impl fmt::Display for Shader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}_{}", self.ty, self.model)
    }
}

#[derive(Debug)]
struct ShaderJob {
    shader: Shader,
    absolute_input_path: PathBuf,
    relative_input_path: PathBuf,
    output_path: PathBuf,
}

/// Finds fxc.exe.
fn get_fxc_path() -> Result<PathBuf> {
    let sdk = SdkInfo::find(SdkVersion::Any)
        .with_context(|| "Error while finding Windows SDK!")?
        .with_context(|| "Couldn't find Windows SDK!")?;
    let sdk_path = sdk.installation_folder();
    let arch = match std::env::consts::ARCH {
        "x86_64" => "x64",
        "aarch64" => "arm64",
        s => s,
    };
    let fxc_glob = format!("{}/bin/*/{}/fxc.exe", sdk_path.display(), arch);
    let fxc = glob(&fxc_glob)?
        .last()
        .with_context(|| format!("Failed to find fxc.exe! Looked at: {}", fxc_glob))??;
    Ok(fxc)
}

/// Returns the relative path of the input file and absolute path of the intended output file if the shader needs compilation.
///
/// If the output file doesn't exist or it exists and has a newer modification time than the input file, returns [None].
fn get_compiled_path(
    input_path: &Path,
    output_dir: &Path,
    shader: &Shader,
) -> Option<(PathBuf, PathBuf)> {
    let file_name: OsString = format!(
        "{}_{}.dxbc",
        input_path.file_stem().unwrap().to_string_lossy(),
        shader,
    )
    .into();

    let relative_source_path = input_path
        .strip_prefix(output_dir.parent().unwrap())
        .unwrap();
    let relative_compiled_path = relative_source_path.with_file_name(&file_name);
    let mut compiled_path = output_dir.to_path_buf();
    compiled_path.push(&relative_compiled_path);

    if !compiled_path.exists()
        || match (fs::metadata(&input_path), fs::metadata(&compiled_path)) {
            (Ok(input_metadata), Ok(output_metadata)) => {
                match (input_metadata.modified(), output_metadata.modified()) {
                    (Ok(input_time), Ok(output_time)) => input_time > output_time,
                    _ => false,
                }
            }
            _ => false,
        }
    {
        Some((relative_source_path.to_path_buf(), compiled_path))
    } else {
        None
    }
}

#[cfg(target_os = "windows")]
fn main() -> Result<()> {
    let fxc = get_fxc_path()?;

    let shader_dir = fs::canonicalize(std::env::current_exe()?.join("../../../shaders"))?;
    let mut output_dir = shader_dir.clone();
    output_dir.push("compiled");
    let mut jobs: Vec<ShaderJob> = vec![];
    println!("Finding shaders...");

    for entry in WalkDir::new(&shader_dir).into_iter().filter_map(|e| e.ok()) {
        let path = match fs::canonicalize(entry.into_path()) {
            Ok(p) => p,
            Err(_) => continue,
        };
        let ext = match path.extension() {
            Some(e) => e,
            None => continue,
        };
        if ext.to_string_lossy() != "hlsl" {
            continue;
        }

        let text = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let first_line = match text.lines().next() {
            Some(l) => l,
            None => continue,
        };

        let mut types = Vec::with_capacity(2);
        if text.contains("VSMain") {
            types.push(ShaderType::Vertex);
        }
        if text.contains("PSMain") {
            types.push(ShaderType::Pixel);
        }

        let mut models = Vec::with_capacity(2);
        if first_line.contains("4_1") {
            models.push(ShaderModel::V4_1);
        }
        if first_line.contains("5_1") {
            models.push(ShaderModel::V5_1);
        }

        for ty in &types {
            for model in &models {
                let shader = Shader {
                    ty: *ty,
                    model: *model,
                };
                if let Some((relative_input_path, output_path)) =
                    get_compiled_path(&path, &output_dir, &shader)
                {
                    jobs.push(ShaderJob {
                        shader,
                        absolute_input_path: path.clone(),
                        relative_input_path,
                        output_path,
                    });
                }
            }
        }
    }

    println!("Shaders to compile: {}\n", jobs.len());

    jobs.retain(|job| {
        if let Some(parent) = job.output_path.parent() {
            if let Err(_) = fs::create_dir_all(parent) {
                eprintln!("Error creating output directory {}!", parent.display());
                return false;
            }
        }

        return true;
    });

    jobs.par_iter()
        .progress_count(jobs.len() as u64)
        .for_each(|job| {
            let output = Command::new(&fxc)
                .args([
                    "/T",
                    &format!("{}", job.shader),
                    "/E",
                    job.shader.ty.get_entry_point(),
                    "/Fo",
                    &job.output_path.to_string_lossy(),
                    &job.absolute_input_path.to_string_lossy(),
                ])
                .output();
            let output = match output {
                Ok(o) => o,
                Err(_) => return,
            };

            let stderr = match std::str::from_utf8(&output.stderr) {
                Ok(s) => s,
                Err(_) => {
                    eprintln!("UTF-8 error while getting compilation stderr!");
                    return;
                }
            };
            if !output.status.success() || stderr.contains("failed") {
                // Indent lines from compiler stderr
                eprintln!(
                    "{}{}",
                    format!(
                        "Failed to compile {} with profile {} (exit code {}):\n    ",
                        job.relative_input_path.display(),
                        job.shader,
                        output.status.code().unwrap_or(-1),
                    )
                    .red(),
                    stderr.replace("\n", "\n    "),
                );
            } else {
                println!(
                    "{}",
                    format!(
                        "Compiled {} with profile {}!\n",
                        job.relative_input_path.display(),
                        job.shader
                    )
                    .green()
                );
            }
        });

    println!("{}", "\nFinished!".green());
    Ok(())
}
