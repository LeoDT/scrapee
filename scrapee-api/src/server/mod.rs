pub mod responses;

use warp::Filter;

use crate::{app_state::AppContext, dao::Dao};

use self::responses::*;

fn with_dao(dao: Dao) -> impl Filter<Extract = (Dao,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || dao.clone())
}

pub fn serve(app_context: AppContext) {
    let port = app_context.server_port;
    let dao = Dao::new(app_context);

    let cors = warp::cors().allow_any_origin();

    let site = warp::path("sites")
        .and(warp::get())
        .and(with_dao(dao.clone()))
        .and_then(|dao: Dao| async move {
            match dao.get_all_sites().await {
                Ok(m) => {
                    let response = SitesResponse::from_model(m);

                    Ok(warp::reply::json(&response))
                }
                Err(_) => Err(warp::reject::reject()),
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
                Err(_) => Err(warp::reject::reject()),
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
                Err(_) => Err(warp::reject::reject()),
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
                Err(_) => Err(warp::reject::reject()),
            }
        });

    let routes = warp::any()
        .and(site.or(page).or(page_content).or(job))
        .with(cors);

    tokio::spawn(async move {
        warp::serve(routes).run(([127, 0, 0, 1], port)).await;
    });
}
