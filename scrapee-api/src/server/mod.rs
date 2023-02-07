pub mod requests;
pub mod responses;

use std::convert::Infallible;

use serde::Serialize;
use warp::{http::StatusCode, Filter, Rejection, Reply};

use crate::{app_state::AppContext, dao::Dao, error::ScrapeeDbError};

use self::requests::*;
use self::responses::*;

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

fn with_dao(dao: Dao) -> impl Filter<Extract = (Dao,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || dao.clone())
}

pub fn serve(app_context: AppContext) {
    let port = app_context.server_port;
    let token = app_context.server_client_token;

    let dao = Dao::new(app_context);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type", "X-Token"])
        .allow_methods(vec!["GET", "POST"]);

    let sites = warp::path("sites")
        .and(warp::get())
        .and(with_dao(dao.clone()))
        .and_then(|dao: Dao| async move {
            match dao.get_all_sites().await {
                Ok(m) => {
                    let response = SitesResponse::from_model(m);

                    Ok(warp::reply::json(&response))
                }
                Err(err) => Err(warp::reject::custom(err)),
            }
        });

    let site = warp::path!("site" / i32)
        .and(warp::get())
        .and(with_dao(dao.clone()))
        .and_then(|id: i32, dao: Dao| async move {
            match dao.get_site_by_id(id).await {
                Ok(m) => {
                    let response = SiteResponse::from_model(m);

                    Ok(warp::reply::json(&response))
                }
                Err(err) => Err(warp::reject::custom(err)),
            }
        });

    let page = warp::path!("site" / i32 / "pages")
        .and(warp::get())
        .and(with_dao(dao.clone()))
        .and_then(|site_id: i32, dao: Dao| async move {
            match dao.find_pages_by_site_id_with_fields(site_id).await {
                Ok(m) => {
                    let response = PagesResponse::from_model(m);

                    Ok(warp::reply::json(&response))
                }
                Err(err) => Err(warp::reject::custom(err)),
            }
        });

    let page_content = warp::path!("page" / i32 / "contents")
        .and(warp::get())
        .and(with_dao(dao.clone()))
        .and_then(|page_id: i32, dao: Dao| async move {
            match dao.get_all_page_content_by_page_id(page_id).await {
                Ok(m) => {
                    let response = PageContentsResponse::from_model(m);

                    Ok(warp::reply::json(&response))
                }
                Err(err) => Err(warp::reject::custom(err)),
            }
        });

    let job = warp::path!("jobs")
        .and(warp::get())
        .and(with_dao(dao.clone()))
        .and_then(|dao: Dao| async move {
            match dao.find_jobs().await {
                Ok(m) => {
                    let response = JobsResponse::from_model(m);

                    Ok(warp::reply::json(&response))
                }
                Err(err) => Err(warp::reject::custom(err)),
            }
        });

    let create_job = warp::path!("job")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_dao(dao.clone()))
        .and_then(|body: CreateJobRequest, dao: Dao| async move {
            match dao.add_job(body.kind, body.message).await {
                Ok(m) => {
                    let response = JobResponse::from_model(m);

                    Ok(warp::reply::json(&response))
                }
                Err(err) => Err(warp::reject::custom(err)),
            }
        });

    let readers = warp::path("readers")
        .and(warp::get())
        .and(with_dao(dao.clone()))
        .and_then(|dao: Dao| async move {
            match dao.find_readers_with_blocks().await {
                Ok(m) => {
                    let response = ReadersResponse::from_model(m);

                    Ok(warp::reply::json(&response))
                }
                Err(err) => Err(warp::reject::custom(err)),
            }
        });

    let reader = warp::path!("reader" / i32)
        .and(warp::get())
        .and(with_dao(dao.clone()))
        .and_then(|reader_id: i32, dao: Dao| async move {
            match dao.get_reader_by_id_with_blocks(reader_id).await {
                Ok(m) => {
                    let response = ReaderResponse::from_model(m);

                    Ok(warp::reply::json(&response))
                }
                Err(err) => Err(warp::reject::custom(err)),
            }
        });

    let create_reader = warp::path!("reader")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_dao(dao.clone()))
        .and_then(|body: CreateReaderRequest, dao: Dao| async move {
            match dao.add_reader(body.name).await {
                Ok(m) => {
                    let response = ReaderResponse::from_model(m);

                    Ok(warp::reply::json(&response))
                }
                Err(err) => Err(warp::reject::custom(err)),
            }
        });

    let create_reader_block = warp::path!("reader" / i32)
        .and(warp::post())
        .and(warp::body::json())
        .and(with_dao(dao.clone()))
        .and_then(
            |reader_id: i32, body: CreateReaderBlockRequest, dao: Dao| async move {
                match dao.add_reader_block(reader_id, body.config).await {
                    Ok(m) => {
                        let response = ReaderBlockResponse::from_model(m);

                        Ok(warp::reply::json(&response))
                    }
                    Err(err) => Err(warp::reject::custom(err)),
                }
            },
        );

    let log = warp::log::custom(|info| {
        log::info!(
            "{} {} {} {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.request_headers(),
        );
    });

    let routes = warp::any()
        .and(warp::header::exact("X-Token", token))
        .and(
            sites
                .or(site)
                .or(page)
                .or(page_content)
                .or(job)
                .or(create_job)
                .or(readers)
                .or(reader)
                .or(create_reader)
                .or(create_reader_block),
        )
        .recover(handle_error)
        .with(cors)
        .with(log);

    tokio::spawn(async move {
        warp::serve(routes).run(([127, 0, 0, 1], port)).await;
    });
}

async fn handle_error(err: Rejection) -> Result<impl Reply, Infallible> {
    let code: warp::http::StatusCode;
    let message: String;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND".into();
    } else if let Some(e) = err.find::<ScrapeeDbError>() {
        match e {
            ScrapeeDbError::NotExist(id, table) => {
                code = StatusCode::NOT_FOUND;
                message = format!("can not find {} with id '{}'", table, id);
            }
            _ => {
                log::error!("internal error: {:?}", e);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "INTERNAL_SERVER_ERROR".into();
            }
        }
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        code = StatusCode::BAD_REQUEST;
        message = "BAD_REQUEST".into();
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED".into();
    } else if let Some(e) = err.find::<warp::reject::MissingHeader>() {
        if e.name().contains("X-Token") {
            code = StatusCode::FORBIDDEN;
            message = "FORBIDDEN".into();
        } else {
            code = StatusCode::BAD_REQUEST;
            message = e.name().into();
        }
    } else {
        // We should have expected this... Just log and say its a 500
        log::error!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION".into();
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message,
    });

    Ok(warp::reply::with_status(json, code))
}
