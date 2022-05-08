/// This files writes outside OUT_DIR because
///
/// 1. I want people running non-Windows operating systems to be able to run
/// the test cases
///
/// 2. I don't want to recompile all the shaders in case I have to clone this
/// repository again
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
    V5_0,
}

impl fmt::Display for ShaderModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::V4_1 => "4_1",
                Self::V5_0 => "5_0",
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
    input_dir: &Path,
    output_dir: &Path,
    shader: &Shader,
) -> Option<(PathBuf, PathBuf)> {
    let file_name: OsString = format!(
        "{}_{}.dxbc",
        input_path.file_stem().unwrap().to_string_lossy(),
        shader,
    )
    .into();

    let relative_source_path = input_path.strip_prefix(input_dir).unwrap();
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

/// Returns shader paths and profiles to compile.
fn find_shaders(input_dir: &PathBuf, output_dir: &Path) -> Vec<ShaderJob> {
    let mut jobs = Vec::new();

    for entry in WalkDir::new(&input_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.into_path();
        let ext = match path.extension() {
            Some(e) => e,
            None => continue,
        };
        if ext.to_string_lossy() != "hlsl" {
            continue;
        }
        // Unwrap safety: if strip_prefix doesn't return a correct result,
        // walkdir isn't working right
        let relative_path = path
            .as_path()
            .strip_prefix(input_dir)
            .unwrap()
            .to_string_lossy();

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

        if types.is_empty() {
            eprintln!(
                "Skipping shader because no entry point was found: {}",
                relative_path
            );
        }

        let mut models = Vec::with_capacity(2);
        if first_line.contains("4_1") {
            models.push(ShaderModel::V4_1);
        }
        if first_line.contains("5_0") {
            models.push(ShaderModel::V5_0);
        }

        if types.is_empty() {
            eprintln!(
                "Skipping shader because no shader models found: {}",
                relative_path
            );
        }

        for ty in &types {
            for model in &models {
                let shader = Shader {
                    ty: *ty,
                    model: *model,
                };
                if let Some((relative_input_path, output_path)) =
                    get_compiled_path(&path, input_dir, output_dir, &shader)
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

    jobs
}

fn compile_shaders() -> Result<()> {
    println!("cargo:rerun-if-changed=shaders");

    // Bail early if the prerequisite tools aren't installed
    let fxc = get_fxc_path()?;

    let shader_dir = PathBuf::from("shaders");
    let input_dir = shader_dir.join("source");
    let output_dir = shader_dir.join("compiled");

    println!("Finding shaders...");

    let mut jobs: Vec<ShaderJob> = find_shaders(&input_dir, &output_dir);

    println!("Shaders to compile: {}\n", jobs.len());

    // Create the parent directories of all compiled shaders

    use std::borrow::Cow;
    jobs.retain(|job| {
        if let Some(parent) = job.output_path.parent() {
            if fs::create_dir_all(parent).is_err() {
                eprintln!("Error creating output directory {}!", parent.display());
                return false;
            }
        }

        true
    });

    jobs.par_iter()
        .progress_count(jobs.len() as u64)
        .for_each(|job| {
            let output = Command::new(&fxc)
                .args([
                    // Syntax: https://docs.microsoft.com/en-us/windows/win32/direct3dtools/dx-graphics-tools-fxc-syntax
                    // Input file
                    &job.absolute_input_path.to_string_lossy(),
                    // Output bytecode
                    "/Fo",
                    &job.output_path.to_string_lossy(),
                    // Output assembly
                    "/Fc",
                    &job.output_path.with_extension("asm").to_string_lossy(),
                    // Profile
                    "/T",
                    // This bullshit is necessary because if it's first, the
                    // compiler derefs everything as a Cow, but if it's after,
                    // I need to do it manually
                    // TODO: find a better way to do this
                    &Cow::Owned(format!("{}", job.shader)),
                    // Entry point
                    "/E",
                    job.shader.ty.get_entry_point(),
                    // Include dir
                    "/I",
                    "CreateMacroShaders/CGIncludes",
                    // Enable backward compatibility
                    "/Gec",
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
                    stderr.replace('\n', "\n    "),
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

fn main() -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        Ok(compile_shaders()?)
    }
    #[cfg(not(target_os = "windows"))]
    {
        Ok(())
    }
}
