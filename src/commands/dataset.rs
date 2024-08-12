#[path = "../resource.rs"]
mod resource;

use resource::{IOResource, Resource, ResourceEngine, ResourceTrait, ResourceURI};
use serde::{Deserialize, Serialize};

const URI_TEMPLATE: &str = "users/:user_id/datasets/";

#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    id: String,
    uri: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DatasetUriParams {
    user_id: String,
}

impl ResourceTrait<Dataset> for Dataset {
    fn from_resource(resource: Resource) -> Dataset {
        let metadata = resource.metadata;
        Dataset {
            id: resource.id,
            uri: resource.uri,
            name: metadata["name"].to_string(),
        }
    }
}

#[tauri::command]
pub fn get_dataset(
    app_handle: tauri::AppHandle,
    user_id: String,
    dataset_id: String,
) -> IOResource<Dataset> {
    ResourceEngine::new::<DatasetUriParams>(
        app_handle,
        ResourceURI {
            template: URI_TEMPLATE.to_string(),
            params: Some(DatasetUriParams { user_id }),
        },
    )
    .get(&dataset_id)
}

#[tauri::command]
pub fn list_datasets(app_handle: tauri::AppHandle, user_id: String) -> IOResource<Vec<Dataset>> {
    ResourceEngine::new::<DatasetUriParams>(
        app_handle,
        ResourceURI {
            template: URI_TEMPLATE.to_string(),
            params: Some(DatasetUriParams { user_id }),
        },
    )
    .list()
}

#[tauri::command]
pub fn create_dataset(
    app_handle: tauri::AppHandle,
    user_id: String,
    metadata: serde_json::Value,
) -> IOResource<Dataset> {
    ResourceEngine::new::<DatasetUriParams>(
        app_handle,
        ResourceURI {
            template: URI_TEMPLATE.to_string(),
            params: Some(DatasetUriParams { user_id }),
        },
    )
    .create(metadata)
}
