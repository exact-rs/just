use std::pin::Pin;

use colored::Colorize;
use data_url::DataUrl;
use deno_core::anyhow::{bail, Error};
use deno_core::futures::FutureExt;
use deno_core::resolve_import;
use deno_core::ModuleLoader;
use deno_core::ModuleSource;
use deno_core::ModuleSourceFuture;
use deno_core::ModuleSpecifier;
use deno_core::ModuleType;

pub struct RuntimeImport;

impl ModuleLoader for RuntimeImport {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _is_main: bool,
    ) -> Result<ModuleSpecifier, Error> {
        Ok(resolve_import(specifier, referrer)?)
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<ModuleSpecifier>,
        _is_dyn_import: bool,
    ) -> Pin<Box<ModuleSourceFuture>> {
        let module_specifier = module_specifier.clone();
        let string_specifier = module_specifier.to_string();

        async {
            let mut module_type = ModuleType::JavaScript;
            let bytes = match module_specifier.scheme() {
                "http" | "https" => {
                    println!("{} {module_specifier}", "download".green(),);
                    let res = reqwest::get(module_specifier).await?;
                    let res = res.error_for_status()?;
                    res.bytes().await?
                }
                "file" => {
                    let path = match module_specifier.to_file_path() {
                        Ok(path) => path,
                        Err(_) => bail!("Invalid file URL."),
                    };
                    module_type = if let Some(extension) = path.extension() {
                        let ext = extension.to_string_lossy().to_lowercase();
                        if ext == "json" {
                            ModuleType::Json
                        } else {
                            ModuleType::JavaScript
                        }
                    } else {
                        ModuleType::JavaScript
                    };
                    let bytes = tokio::fs::read(path).await?;
                    bytes.into()
                }
                "data" => {
                    let url = match DataUrl::process(module_specifier.as_str()) {
                        Ok(url) => url,
                        Err(_) => bail!("Not a valid data URL."),
                    };
                    let bytes = match url.decode_to_vec() {
                        Ok((bytes, _)) => bytes,
                        Err(_) => bail!("Not a valid data URL."),
                    };
                    bytes.into()
                }
                schema => bail!("Invalid schema {}", schema),
            };

            let bytes = if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
                bytes.slice(3..)
            } else {
                bytes
            };

            Ok(ModuleSource {
                code: bytes.to_vec().into_boxed_slice(),
                module_type: module_type,
                module_url_specified: string_specifier.clone(),
                module_url_found: string_specifier,
            })
        }
        .boxed_local()
    }
}