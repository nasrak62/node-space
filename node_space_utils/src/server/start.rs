use axum::body::Body;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use axum::{extract::Path as AxumPath, response::Response};
use http_body_util::StreamBody;
use hyper::StatusCode;
use tokio_util::io::ReaderStream;

use crate::{
    args::server_args::StartServerArgs,
    errors::node_space::NodeSpaceError,
    modals::{config_file::ConfigFile, server_config::ServerConfig},
    package_utils::get_base_package_data,
};

pub fn get_config_name(args: &StartServerArgs) -> Result<String, NodeSpaceError> {
    if args.name.is_some() {
        return Ok(args.name.clone().unwrap());
    }

    let (_, package_name, _) = get_base_package_data(None)?;

    Ok(package_name)
}

pub fn get_default_config(config: Option<&ServerConfig>) -> Result<ServerConfig, NodeSpaceError> {
    if config.is_some() {
        return Ok(config.unwrap().clone());
    }

    let (_, _, current_path) = get_base_package_data(None)?;

    Ok(ServerConfig::default(current_path))
}

async fn build_file_body_stream(path: String) -> Result<Body, NodeSpaceError> {
    let file_result = tokio::fs::File::open(&path).await;

    if let Err(error) = file_result {
        let message = format!("File not found: {}, error: {}", &path, error.to_string());

        return Err(NodeSpaceError::ServerError(message));
    }

    let file = file_result.unwrap();
    let reader_stream = ReaderStream::new(file);
    let stream_body = StreamBody::new(reader_stream);

    Ok(Body::from_stream(stream_body))
}

/// gets the outdir real path on the file system: /home/user/dev/project1/dist
/// gets the request path localhost:3000/project1/index.js -> project1/index.js
async fn serve_files(output_dir: &str, file_path: String) -> Response {
    let mut path = String::from(output_dir) + "/" + &file_path;
    path = path.replace("//", "/");

    let body = match build_file_body_stream(path.clone()).await {
        Err(error) => {
            return (
                axum::http::StatusCode::NOT_FOUND,
                format!("File not found: {}, error: {}", path, error),
            )
                .into_response();
        }

        Ok(body) => body,
    };

    match Response::builder().status(StatusCode::OK).body(body) {
        Ok(value) => value,
        Err(error) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("response error: {}, error: {}", path, error),
            )
                .into_response();
        }
    }
}

async fn serve_html(main_route: String) -> Response {
    dbg!("in serve_html");
    let mut path = main_route + "/index.html";
    path = path.replace("//", "/");

    let body = match build_file_body_stream(path.clone()).await {
        Err(error) => {
            return (
                axum::http::StatusCode::NOT_FOUND,
                format!("File not found: {}, error: {}", path, error),
            )
                .into_response();
        }

        Ok(body) => body,
    };

    match Response::builder().status(StatusCode::OK).body(body) {
        Ok(value) => value,
        Err(error) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("response error: {}, error: {}", path, error),
            )
                .into_response();
        }
    }
}

pub async fn handle_server_start(args: &StartServerArgs) -> Result<bool, NodeSpaceError> {
    let config_file = ConfigFile::new()?;
    let name = get_config_name(args)?;
    let server_config = config_file.server_config.get(&name);

    if server_config.is_none() && args.name.is_some() {
        return Err(NodeSpaceError::InvalidRoutesConfig(
            "project wasn't found in configs".to_string(),
        ));
    }

    let server_config = get_default_config(server_config)?;
    let routes = server_config.routes.clone();
    let main_route = server_config.get_main_route_output_dir()?;

    let port = match args.port.clone() {
        Some(value) => value,
        None => server_config.port.clone(),
    };

    let mut app = Router::new();

    for (route, output_dir) in routes.iter() {
        let output_clone = output_dir.clone();
        let files_route = (route.to_string() + "/{*file_path}").replace("//", "/");
        let base_route = (route.to_string() + "/").replace("//", "/");
        let main_route_copy = main_route.clone();

        dbg!(route, &base_route);

        app = app
            .route(
                &files_route,
                get(move |AxumPath(path): AxumPath<String>| async move {
                    serve_files(&output_clone, path).await
                }),
            )
            .route(
                &base_route,
                get(move || async move { serve_html(main_route_copy).await }),
            )
    }

    let host = String::from("0.0.0.0:") + &port;

    dbg!(&host);

    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(true)
}
