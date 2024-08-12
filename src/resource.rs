#[path = "./helpers.rs"]
mod helpers;

use glob::glob;
use helpers::{capture_values, random_id, read_json_file, write_json_file};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct IOResourceError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IOResource<T> {
    pub data: Option<T>,
    pub error: Option<IOResourceError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    pub id: String,
    pub uri: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct ResourceEngine {
    pub base_path: String,
}

pub struct ResourceURI<TParams> {
    pub template: String,
    pub params: Option<TParams>,
}

pub trait ResourceTrait<T> {
    fn from_resource(resource: Resource) -> T;
}

impl ResourceEngine {
    pub fn new<TParams: Serialize>(
        app_handle: tauri::AppHandle,
        uri: ResourceURI<TParams>,
    ) -> ResourceEngine {
        if uri.params.is_none() {
            return ResourceEngine {
                base_path: app_handle
                    .path_resolver()
                    .app_data_dir()
                    .expect("Failed to read app data dir")
                    .join(&uri.template)
                    .to_str()
                    .unwrap()
                    .to_string(),
            };
        }

        let v = serde_json::to_value(&uri.params).unwrap();
        let params: HashMap<String, String> = serde_json::from_value(v).unwrap();

        let uri_template = app_handle
            .path_resolver()
            .app_data_dir()
            .expect("Failed to read app data dir")
            .join(&uri.template)
            .to_str()
            .unwrap()
            .to_string();

        let base_path = params.iter().fold(uri_template, |acc, (key, value)| {
            acc.replace(&format!(":{}", key), value)
        });

        ResourceEngine { base_path }
    }

    pub fn get<T: ResourceTrait<T>>(self, resource_id: &str) -> IOResource<T> {
        let uri = PathBuf::new()
            .join(&self.base_path)
            .join(resource_id)
            .to_str()
            .expect("Failed to read resource path")
            .to_string();

        let file_path = PathBuf::new()
            .join(&uri)
            .join("metadata.json")
            .to_str()
            .expect("Failed to read file path")
            .to_string();

        match read_json_file(file_path) {
            Ok(metadata) => IOResource {
                data: Some(T::from_resource(Resource {
                    id: resource_id.to_string(),
                    uri,
                    metadata,
                })),
                error: None,
            },
            Err(_e) => IOResource {
                data: None,
                error: Some(IOResourceError {
                    code: "404".to_string(),
                    message: "Resource not found".to_string(),
                }),
            },
        }
    }

    pub fn list<T: ResourceTrait<T>>(self) -> IOResource<Vec<T>> {
        let glob_path = PathBuf::new()
            .join(&self.base_path)
            .join("*")
            .join("metadata.json")
            .to_str()
            .unwrap()
            .to_string();

        let mut sub_directories: Vec<String> = [].to_vec();
        let files = glob(&glob_path).expect("Failed to read glob pattern");

        for entry in files {
            match entry {
                Ok(path) => sub_directories.push(path.to_str().unwrap().to_string()),
                Err(e) => panic!("Error reading file: {:?}", e),
            };
        }

        let results: Vec<T> = sub_directories
            .into_iter()
            .filter_map(|path| {
                let re_string = PathBuf::new()
                    .join(&self.base_path)
                    .join(r"(\w+)")
                    .join("metadata.json")
                    .to_str()
                    .unwrap()
                    .to_string();

                let vars = capture_values(&re_string, path);
                let resource_id = vars[1].clone();

                self.clone().get::<T>(&resource_id).data
            })
            .collect();

        IOResource {
            data: Some(results),
            error: None,
        }
    }

    pub fn create<T: ResourceTrait<T>>(self, metadata: serde_json::Value) -> IOResource<T> {
        let resource_id = random_id();
        let resource_uri = PathBuf::new()
            .join(&self.base_path)
            .join(&resource_id)
            .to_str()
            .expect("Failed to read resource path")
            .to_string();

        fs::create_dir_all(&resource_uri).expect(&format!(
            "Failed to create resource directory: {}",
            &resource_uri
        ));

        let file_path = PathBuf::from(&resource_uri)
            .join("metadata.json")
            .to_str()
            .expect("Failed to read file path")
            .to_string();

        write_json_file::<serde_json::Value>(file_path, &metadata);

        let resource = self
            .get::<T>(&resource_id)
            .data
            .expect(&format!("Could not get resource after create",));

        IOResource {
            data: Some(resource),
            error: None,
        }
    }
}
