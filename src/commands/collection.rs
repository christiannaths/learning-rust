#[path = "../resource.rs"]
mod resource;

use std::path::PathBuf;

use resource::{IOResource, Resource, ResourceEngine, ResourceTrait, ResourceURI};
use serde::{Deserialize, Serialize};

const URI_TEMPLATE: &str = "users/:user_id/datasets/:dataset_id/collections";

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionData(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    id: String,
    uri: String,
    name: String,
    data: Option<CollectionData>,
    schema: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CollectionUriParams {
    user_id: String,
    dataset_id: String,
}

impl ResourceTrait<Collection> for Collection {
    fn from_resource(resource: Resource) -> Collection {
        let Resource { id, uri, metadata } = resource;
        let name = metadata["name"].to_string();

        let data_file = PathBuf::new().join(&uri).join("data.csv");

        let data = match std::fs::read_to_string(data_file) {
            Ok(data) => Some(CollectionData(data)),
            Err(_) => None,
        };

        let schema_file = PathBuf::new().join(&uri).join("schema.json");

        let schema = match std::fs::read_to_string(schema_file) {
            Ok(schema) => Some(CollectionData(schema)),
            Err(_) => None,
        };

        Collection {
            id,
            uri,
            name,
            data,
            schema,
        }
    }
}

#[tauri::command]
pub fn get_collection(
    app_handle: tauri::AppHandle,
    user_id: String,
    dataset_id: String,
    collection_id: String,
) -> IOResource<Collection> {
    let collection = ResourceEngine::new::<CollectionUriParams>(
        app_handle,
        ResourceURI {
            template: URI_TEMPLATE.to_string(),
            params: Some(CollectionUriParams {
                user_id,
                dataset_id,
            }),
        },
    )
    .get(&collection_id);

    collection
}

#[tauri::command]
pub fn list_collections(
    app_handle: tauri::AppHandle,
    user_id: String,
    dataset_id: String,
) -> IOResource<Vec<Collection>> {
    ResourceEngine::new::<CollectionUriParams>(
        app_handle,
        ResourceURI {
            template: URI_TEMPLATE.to_string(),
            params: Some(CollectionUriParams {
                user_id,
                dataset_id,
            }),
        },
    )
    .list()
}

#[tauri::command]
pub fn create_collection(
    app_handle: tauri::AppHandle,
    user_id: String,
    dataset_id: String,
    metadata: serde_json::Value,
) -> IOResource<Collection> {
    ResourceEngine::new::<CollectionUriParams>(
        app_handle,
        ResourceURI {
            template: URI_TEMPLATE.to_string(),
            params: Some(CollectionUriParams {
                user_id,
                dataset_id,
            }),
        },
    )
    .create(metadata)
}
