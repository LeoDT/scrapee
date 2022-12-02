use futures::future::join_all;
use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseConnection, Statement};

use crate::app_state::AppContext;

use super::Dao;

pub async fn saraba(db: DatabaseConnection) {
    let tables = vec!["job", "page_content", "field", "page", "site"];
    let _ = db
        .execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            tables
                .iter()
                .map(|t| {
                    format!(
                        "delete from {}; delete from sqlite_sequence where name = '{}';",
                        t, t
                    )
                })
                .collect::<Vec<String>>()
                .join("\n"),
        ))
        .await;

    let dao = Dao::new(AppContext::new(db));

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
}
