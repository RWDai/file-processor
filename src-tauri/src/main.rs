#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::info;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{env, sync::Mutex};
use tardis::{basic::result::TardisResult, tokio, TardisFuns};
mod tauri;
mod uploader;

pub static PARAMS: Lazy<Mutex<FileProcessParams>> = Lazy::new(|| {
    Mutex::new(FileProcessParams {
        title: "".to_string(),
        upload: None,
    })
});

#[tokio::main]
async fn main() -> TardisResult<()> {
    env::set_var("RUST_LOG", "debug");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut raw_params = args[1].as_str();
        if raw_params.contains("//") {
            let index = raw_params.find("//").unwrap();
            raw_params = &raw_params[index + 2..];
        }
        if raw_params.ends_with("/") {
            raw_params = &raw_params[..raw_params.len() - 1];
        }
        let params = TardisFuns::json
            .str_to_obj::<FileProcessParams>(
                &TardisFuns::crypto
                    .base64
                    .decode_to_string(raw_params)
                    .unwrap(),
            )
            .unwrap();
        info!("params: {:?}", params);
        let mut params_set = PARAMS.lock().unwrap();
        *params_set = params;
    } else {
        // mock
        let mut params_set = PARAMS.lock().unwrap();
        *params_set = FileProcessParams {
            title: "请按使用文档调用（以下为示例）".to_string(),
            upload: Some(FileUploadProcessParams {
                target_kind_key: "".to_string(),
                target_obj_key: "".to_string(),
                overwrite: false,
                upload_metadata_url: "".to_string(),
            }),
        };
    }

    tauri::build();

    TardisFuns::init(Some("config")).await?;

    info!("started program.");

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileProcessParams {
    pub title: String,
    pub upload: Option<FileUploadProcessParams>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileUploadProcessParams {
    pub target_kind_key: String,
    pub target_obj_key: String,
    pub overwrite: bool,
    pub upload_metadata_url: String,
}
