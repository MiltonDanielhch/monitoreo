// crates/infrastructure/src/handlers/infrastructure_file_handler.rs
// Handlers HTTP para archivos de infraestructura de red
// Vinculado con ADR-0003-stack-backend-rust-axum.md

use crate::AppState;
use axum::{
    extract::{Multipart, Path, State},
    http::header::{CONTENT_DISPOSITION, CONTENT_TYPE},
    response::Response,
    Json,
};
use database::NetworkFileRepository;
use domain::models::infrastructure_file::{InfrastructureFile, NetworkFileType, NetworkStoragePort};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::uuid;
use crate::storage::RegionalStorageAdapter;

/// DTO para respuesta de subida de archivo
#[derive(Serialize)]
pub struct FileUploadResponse {
    pub id: String,
    pub filename: String,
    pub file_type: String,
    pub storage_key: String,
    pub sha256_checksum: String,
    pub message: String,
}

/// DTO para respuesta de error
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// DTO para lista de archivos
#[derive(Serialize)]
pub struct FileListResponse {
    pub files: Vec<FileMetadata>,
}

#[derive(Serialize)]
pub struct FileMetadata {
    pub id: String,
    pub filename: String,
    pub file_type: String,
    pub file_size_bytes: u64,
}

/// Endpoint para subir archivos de infraestructura (streaming)
pub async fn upload_file(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<FileUploadResponse>, (axum::http::StatusCode, Json<ErrorResponse>)> {
    let storage_adapter = RegionalStorageAdapter::new(PathBuf::from("./storage"));
    let file_repo = NetworkFileRepository::new(state.db.clone());

    // Procesar el multipart
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (
            axum::http::StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Error processing multipart: {}", e),
            }),
        )
    })? {
        let filename = field.file_name().unwrap_or("unknown").to_string();
        let file_data = field.bytes().await.map_err(|e| {
            (
                axum::http::StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("Error reading file data: {}", e),
                }),
            )
        })?;

        // Determinar tipo de archivo
        let file_type = if filename.to_lowercase().ends_with(".svg") {
            NetworkFileType::TopologySvg
        } else if filename.to_lowercase().ends_with(".png")
            || filename.to_lowercase().ends_with(".jpg")
            || filename.to_lowercase().ends_with(".jpeg")
        {
            NetworkFileType::RackImage
        } else if filename.to_lowercase().ends_with(".cfg")
            || filename.to_lowercase().ends_with(".txt")
        {
            NetworkFileType::ConfigBackup
        } else {
            return Err((
                axum::http::StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Unsupported file type".to_string(),
                }),
            ));
        };

        // Validar contenido
        let temp_file = InfrastructureFile::new(
            uuid!("550e8400-e29b-41d4-a716-446655440000").to_string(),
            filename.clone(),
            file_type,
            file_data.len() as u64,
            "".to_string(),
            "".to_string(),
            "trinidad".to_string(), // TODO: obtener sede_id del usuario
            None,
        );

        if let Err(e) = temp_file.validate_content(&file_data) {
            return Err((
                axum::http::StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("Invalid file content: {}", e),
                }),
            ));
        }

        // Calcular checksum
        let checksum = RegionalStorageAdapter::calculate_checksum(&file_data);

        // Verificar deduplicación
        if let Ok(exists) = file_repo.file_exists_by_checksum(&checksum).await {
            if exists {
                return Err((
                    axum::http::StatusCode::CONFLICT,
                    Json(ErrorResponse {
                        error: "File with same content already exists".to_string(),
                    }),
                ));
            }
        }

        // Guardar en almacenamiento
        let storage_key = storage_adapter
            .save_file(&temp_file, &file_data)
            .await
            .map_err(|e| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Error saving file: {}", e),
                    }),
                )
            })?;

        // Guardar metadatos en base de datos
        let file_metadata = InfrastructureFile::new(
            temp_file.id.clone(),
            filename.clone(),
            file_type,
            file_data.len() as u64,
            storage_key.clone(),
            checksum.clone(),
            "trinidad".to_string(), // TODO: obtener sede_id del usuario
            None, // TODO: obtener user_id del token
        );

        let file_id = file_repo
            .insert_file(&file_metadata)
            .await
            .map_err(|e| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Error saving file metadata: {}", e),
                    }),
                )
            })?;

        return Ok(Json(FileUploadResponse {
            id: file_id,
            filename,
            file_type: file_type.to_string(),
            storage_key,
            sha256_checksum: checksum,
            message: "File uploaded successfully".to_string(),
        }));
    }

    Err((
        axum::http::StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            error: "No file provided".to_string(),
        }),
    ))
}

/// Endpoint para descargar un archivo
pub async fn download_file(
    State(state): State<AppState>,
    Path(file_id): Path<String>,
) -> Result<Response, (axum::http::StatusCode, Json<ErrorResponse>)> {
    let storage_adapter = RegionalStorageAdapter::new(PathBuf::from("./storage"));
    let file_repo = NetworkFileRepository::new(state.db.clone());

    // Obtener metadatos del archivo
    let files = file_repo
        .get_files_by_sede("trinidad") // TODO: filtrar por sede del usuario
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Error fetching file: {}", e),
                }),
            )
        })?;

    let file_metadata = files
        .iter()
        .find(|f| f.id == file_id)
        .ok_or_else(|| {
            (
                axum::http::StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "File not found".to_string(),
                }),
            )
        })?;

    // Obtener contenido del archivo
    let content = storage_adapter
        .get_file(&file_metadata.storage_key)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Error reading file: {}", e),
                }),
            )
        })?;

    // Determinar content type
    let content_type = match file_metadata.file_type {
        NetworkFileType::TopologySvg => "image/svg+xml",
        NetworkFileType::RackImage => {
            if file_metadata.filename.to_lowercase().ends_with(".png") {
                "image/png"
            } else {
                "image/jpeg"
            }
        }
        NetworkFileType::ConfigBackup => "text/plain",
    };

    let response = axum::response::Response::builder()
        .status(axum::http::StatusCode::OK)
        .header(CONTENT_TYPE, content_type)
        .header(
            CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", file_metadata.filename),
        )
        .body(axum::body::Body::from(content))
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Error building response: {}", e),
                }),
            )
        })?;

    Ok(response)
}

/// Endpoint para listar archivos por sede
pub async fn list_files(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<ListFilesQuery>,
) -> Result<Json<FileListResponse>, (axum::http::StatusCode, Json<ErrorResponse>)> {
    let file_repo = NetworkFileRepository::new(state.db.clone());

    let sede_id = params.sede_id.unwrap_or_else(|| "trinidad".to_string());
    let files = file_repo
        .get_files_by_sede(&sede_id)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Error fetching files: {}", e),
                }),
            )
        })?;

    let file_metadata: Vec<FileMetadata> = files
        .into_iter()
        .map(|f| FileMetadata {
            id: f.id,
            filename: f.filename,
            file_type: f.file_type.to_string(),
            file_size_bytes: f.file_size_bytes,
        })
        .collect();

    Ok(Json(FileListResponse { files: file_metadata }))
}

#[derive(Deserialize)]
pub struct ListFilesQuery {
    pub sede_id: Option<String>,
}
