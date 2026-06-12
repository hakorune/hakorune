//! Nyash Plugin Tester v2 - nyash.toml中心設計対応版
//!
//! 究極のシンプル設計:
//! - Host VTable廃止
//! - nyash_plugin_invokeのみ使用
//! - すべてのメタ情報はnyash.tomlから取得

use clap::{Parser, Subcommand};
use colored::*;
use libloading::{Library, Symbol};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

// ============ nyash.toml v2 Types ============

#[derive(Debug, Deserialize)]
struct NyashConfigV2 {
    libraries: HashMap<String, LibraryDefinition>,
}

#[derive(Debug, Deserialize)]
struct LibraryDefinition {
    boxes: Vec<String>,
    path: String,
}

#[derive(Debug, Deserialize)]
struct BoxTypeConfig {
    type_id: u32,
    #[serde(default = "default_abi_version")]
    abi_version: u32,
    methods: HashMap<String, MethodDefinition>,
}

fn default_abi_version() -> u32 {
    1
}

#[derive(Debug, Deserialize)]
struct MethodDefinition {
    method_id: u32,
}

// ============ CLI ============

#[derive(Parser)]
#[command(name = "plugin-tester-v2")]
#[command(about = "Nyash plugin testing tool v2 - nyash.toml centric", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check plugin with nyash.toml v2
    Check {
        /// Path to nyash.toml file
        #[arg(short, long, default_value = "../../nyash.toml")]
        config: PathBuf,

        /// Library name (e.g., "libnyash_filebox_plugin.so")
        #[arg(short, long)]
        library: Option<String>,
    },
    /// Test Box lifecycle with nyash.toml v2
    Lifecycle {
        /// Path to nyash.toml file
        #[arg(short, long, default_value = "../../nyash.toml")]
        config: PathBuf,

        /// Box type name (e.g., "FileBox")
        box_type: String,
    },
    /// Validate all plugins in nyash.toml
    ValidateAll {
        /// Path to nyash.toml file
        #[arg(short, long, default_value = "../../nyash.toml")]
        config: PathBuf,
    },
    /// Phase 15.5: Safety check with ChatGPT recommended features
    SafetyCheck {
        /// Path to nyash.toml file
        #[arg(short, long, default_value = "../../nyash.toml")]
        config: PathBuf,

        /// Library name to check (optional, checks all if not specified)
        #[arg(short, long)]
        library: Option<String>,

        /// Box type to check (optional, checks all if not specified)
        #[arg(short, long)]
        box_type: Option<String>,
    },
}

// ============ TLV Helpers ============

fn tlv_encode_empty() -> Vec<u8> {
    vec![1, 0, 0, 0] // version=1, argc=0
}

fn tlv_encode_one_handle(type_id: u32, instance_id: u32) -> Vec<u8> {
    // BID-1 TLV header: u16 ver=1, u16 argc=1
    // Entry: tag=8(Handle), rsv=0, size=u16(8), payload=[type_id(4), instance_id(4)]
    let mut buf = Vec::with_capacity(4 + 4 + 8);
    buf.extend_from_slice(&1u16.to_le_bytes()); // ver
    buf.extend_from_slice(&1u16.to_le_bytes()); // argc
    buf.push(8u8); // tag=Handle
    buf.push(0u8); // reserved
    buf.extend_from_slice(&(8u16).to_le_bytes()); // size
    buf.extend_from_slice(&type_id.to_le_bytes());
    buf.extend_from_slice(&instance_id.to_le_bytes());
    buf
}

fn tlv_encode_two_strings(a: &str, b: &str) -> Vec<u8> {
    let ab = a.as_bytes();
    let bb = b.as_bytes();
    let mut buf = Vec::with_capacity(4 + 2 * (4 + ab.len().min(u16::MAX as usize)));
    buf.extend_from_slice(&1u16.to_le_bytes()); // ver
    buf.extend_from_slice(&2u16.to_le_bytes()); // argc=2
                                                // first string
    buf.push(6u8);
    buf.push(0u8);
    buf.extend_from_slice(&((ab.len().min(u16::MAX as usize) as u16).to_le_bytes()));
    buf.extend_from_slice(ab);
    // second string
    buf.push(6u8);
    buf.push(0u8);
    buf.extend_from_slice(&((bb.len().min(u16::MAX as usize) as u16).to_le_bytes()));
    buf.extend_from_slice(bb);
    buf
}

fn tlv_encode_bytes(data: &[u8]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(4 + 4 + data.len());
    buf.extend_from_slice(&1u16.to_le_bytes()); // ver
    buf.extend_from_slice(&1u16.to_le_bytes()); // argc=1
    buf.push(7u8);
    buf.push(0u8);
    buf.extend_from_slice(&((data.len().min(u16::MAX as usize) as u16).to_le_bytes()));
    buf.extend_from_slice(data);
    buf
}

fn tlv_decode_u32(data: &[u8]) -> Result<u32, String> {
    if data.len() >= 4 {
        Ok(u32::from_le_bytes([data[0], data[1], data[2], data[3]]))
    } else {
        Err("Buffer too short for u32".to_string())
    }
}

// ============ Main Functions ============

mod check;
mod lifecycle;
mod safety;

pub(crate) fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Check { config, library } => check::check_v2(&config, library.as_deref()),
        Commands::Lifecycle { config, box_type } => {
            lifecycle::test_lifecycle_v2(&config, &box_type)
        }
        Commands::ValidateAll { config } => check::validate_all(&config),
        Commands::SafetyCheck {
            config,
            library,
            box_type,
        } => safety::safety_check_v2(&config, library.as_deref(), box_type.as_deref()),
    }
}

fn resolve_plugin_path(base: &Path, raw: &str) -> PathBuf {
    let candidate = if Path::new(raw).is_absolute() {
        PathBuf::from(raw)
    } else {
        base.join(raw)
    };
    if candidate.exists() {
        return candidate;
    }
    let ext = if cfg!(target_os = "windows") {
        "dll"
    } else if cfg!(target_os = "macos") {
        "dylib"
    } else {
        "so"
    };
    let mut with_ext = candidate.clone();
    with_ext.set_extension(ext);
    with_ext
}

fn load_config(config_path: &PathBuf) -> Result<(NyashConfigV2, toml::Value), String> {
    let config_content =
        fs::read_to_string(config_path).map_err(|e| format!("Failed to read config: {}", e))?;

    let config: NyashConfigV2 = toml::from_str(&config_content)
        .map_err(|e| format!("Failed to parse nyash.toml v2: {}", e))?;

    let raw_config: toml::Value = toml::from_str(&config_content)
        .map_err(|e| format!("Failed to parse TOML value: {}", e))?;

    Ok((config, raw_config))
}

fn find_library_for_box<'a>(
    config: &'a NyashConfigV2,
    box_type: &str,
) -> Option<(&'a str, &'a LibraryDefinition)> {
    config
        .libraries
        .iter()
        .find(|(_, lib)| lib.boxes.contains(&box_type.to_string()))
        .map(|(name, lib)| (name.as_str(), lib))
}

fn get_box_config(
    raw_config: &toml::Value,
    lib_name: &str,
    box_name: &str,
) -> Option<BoxTypeConfig> {
    raw_config
        .get("libraries")
        .and_then(|v| v.get(lib_name))
        .and_then(|v| v.get(box_name))
        .and_then(|v| v.clone().try_into::<BoxTypeConfig>().ok())
}
