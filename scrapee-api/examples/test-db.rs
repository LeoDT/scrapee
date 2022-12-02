use std::env;
use std::sync::Arc;

use futures::future::join_all;

use sea_orm::*;

use scrapee_api::collector;
use scrapee_api::dao::{field, page, run_migrate, site, Dao};

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let _ = std::fs::remove_file("a.db");

    let uri = "sqlite:a.db";
    let _ = run_migrate(uri).await;

    let conn = Database::connect(uri).await.unwrap();
    let dao = Dao::new(conn);

    let new_site = dao.add_site("Saraba1st".to_string(), false).await.unwrap();

    let new_page = dao
        .add_page(
            "index".to_string(),
            "https://bbs.saraba1st.com/2b/forum-75-{{page}}.html".to_string(),
            new_site.id,
            Some("https://bbs.saraba1st.com/2b/forum-75-1.html".to_string()),
        )
        .await
        .unwrap();

    let fields = vec![(
        "排名".to_string(),
        r#"/html/body/div[8]/div[4]/div/div/div[1]/div[1]/h1/span/strong[3]/text()"#.to_string(),
        false,
        None,
    ), (
        "版规".to_string(),
        r#"/html/body/div[8]/div[4]/div/div/div[1]/div[2]/div[2]/div/text()"#
                    .to_string(),
        false,
        None,
    ), (
        "链接".to_string(),
        r#"//table[@id="threadlisttableid"]/tbody[contains(@id, "normalthread")]/tr/th/a[2]/@href"#.to_string(),
        true,
        Some("帖子".to_string()),
    ), (
        "标题".to_string(),
        r#"//table[@id="threadlisttableid"]/tbody[contains(@id, "normalthread")]/tr/th/a[2]/text()"#.to_string(),
        false,
        Some("帖子".to_string()),
    )];

    let fields = fields.iter().map(|f| {
        let (name, xpath, try_follow, group_to) = f.to_owned();

        dao.add_field(name, xpath, try_follow, new_page.id, group_to)
    });

    join_all(fields).await;

    let new_page = dao
        .add_page(
            "detail".to_string(),
            "https://bbs.saraba1st.com/2b/thread-{{id}}-{{page}}-1.html".to_string(),
            new_site.id,
            None,
        )
        .await
        .unwrap();

    let fields = vec![(
        "内容".to_string(),
        r#"(//td[contains(@id, "postmessage")])[1]"#.to_string(),
        false,
        None,
    )];

    let fields = fields.iter().map(|f| {
        let (name, xpath, try_follow, group_to) = f.to_owned();

        dao.add_field(name, xpath, try_follow, new_page.id, group_to)
    });

    join_all(fields).await;

    let new_site = dao.get_site(1).await.unwrap().unwrap();
    let new_pages = page::Entity::find()
        .filter(page::Column::SiteId.eq(new_site.id))
        .find_with_related(field::Entity)
        .all(&dao.db)
        .await
        .unwrap();

    let pages = new_pages
        .into_iter()
        .map(|(p, fields)| {
            let fields = fields
                .into_iter()
                .map(|f| collector::site::Field {
                    id: f.id,
                    name: f.name,
                    xpath: f.xpath,
                    try_follow: f.try_follow,
                    group_to: f.group_to,
                })
                .collect();

            Arc::new(collector::site::Page {
                id: p.id,
                name: p.name,
                url: p.url,
                url_pattern: collector::site::make_url_pattern(p.url_pattern).unwrap(),
                fields,
            })
        })
        .collect();

    let a_site = Arc::new(collector::site::Site {
        id: new_site.id,
        name: new_site.name,
        save_context: new_site.save_context,
        pages,
    });

    let collector = collector::Collector::new(a_site);

    collector.collect().await;

    // log::debug!("{:?} {:?}", new_site, new_pages);
}
