//! Phase 0 spike 3: our own `Db`, built on rok-ui's runtime and handed to the
//! app, feeds a `use_query` list on screen; a write invalidates the key and the
//! list refreshes.
//!
//! The tenant filter is spelled out in the query here because spike 2 already
//! proved that row level security stops a cross-business read even when the
//! query forgets it.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::{Context, IntoElement, Render, Window};
use rok_ui::Cx;
use rok_ui::db::{
    self,
    rok_db::{Db, raw},
};
use rok_ui::prelude::*;
use rok_ui::query::{self, QueryState};
use rok_ui::query_key;
use sqlx::Row;
use uuid::Uuid;

const TABLE: &str = "rok_pos_spike3_batches";

/// What the screen last showed, so the test can read it without walking the
/// element tree.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Rendered {
    pending: bool,
    failed: Option<String>,
    rows: Vec<String>,
}

fn batches_query(organization_id: Uuid) -> query::QueryOptions<Vec<String>> {
    db::db_query(query_key!["rok_pos_spike3_batches"], move |db| async move {
        let rows = sqlx::query(
            "select batch_number from rok_pos_spike3_batches \
             where organization_id = $1 order by batch_number",
        )
        .bind(organization_id)
        .fetch_all(db.pool())
        .await?;
        Ok(rows
            .iter()
            .map(|row| row.try_get::<String, _>("batch_number"))
            .collect::<Result<Vec<String>, sqlx::Error>>()?)
    })
}

struct BatchList {
    organization_id: Uuid,
    rendered: Rc<RefCell<Rendered>>,
}

impl Render for BatchList {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut cx = Cx::new(window, cx);
        let result = query::use_query(&mut cx, batches_query(self.organization_id));
        let rendered = match result.state() {
            QueryState::Pending => Rendered {
                pending: true,
                failed: None,
                rows: Vec::new(),
            },
            QueryState::Error(error) => Rendered {
                pending: false,
                failed: Some(error.to_string()),
                rows: Vec::new(),
            },
            QueryState::Success(rows) => Rendered {
                pending: false,
                failed: None,
                rows: rows.clone(),
            },
        };
        *self.rendered.borrow_mut() = rendered;

        div()
            .flex_col()
            .gap_1()
            .children(self.rendered.borrow().rows.iter().cloned())
    }
}

/// Draw until the screen shows `wanted`, or give up.
fn draw_until(
    window: &mut gpui::VisualTestContext,
    rendered: &Rc<RefCell<Rendered>>,
    wanted: &dyn Fn(&Rendered) -> bool,
) {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        window.update(|window, cx| {
            let _ = window.draw(cx);
        });
        window.run_until_parked();
        if wanted(&rendered.borrow()) {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "the list never settled: {:?}",
            rendered.borrow()
        );
        std::thread::sleep(Duration::from_millis(50));
    }
}

#[gpui::test]
fn our_own_connection_feeds_a_list_and_a_write_refreshes_it(cx: &mut gpui::TestAppContext) {
    let Ok(url) = std::env::var("DATABASE_URL") else {
        eprintln!("DATABASE_URL is not set; skipping");
        return;
    };
    cx.executor().allow_parking();
    cx.update(rok_ui::init);

    // The app's own pool, built on rok-ui's runtime and settings.
    let our_own = db::runtime()
        .block_on(async { Db::builder().max_connections(4).connect(&url).await })
        .expect("our own pool connects");
    cx.update(|cx| db::set_connection(our_own.clone(), cx));
    assert!(
        cx.update(|cx| db::connection(cx).is_some()),
        "the app is using the connection it was given"
    );

    let afya = Uuid::now_v7();
    let setup = cx.update(|cx| {
        db::run(cx, |db| async move {
            db.execute(&format!("drop table if exists {TABLE}")).await?;
            db.execute(&format!(
                "create table {TABLE} (\
                   id uuid primary key, \
                   organization_id uuid not null, \
                   batch_number text not null)"
            ))
            .await?;
            for (organization_id, batch_number) in [
                (afya, "AMS-2404"),
                (afya, "AMX-2409"),
                (Uuid::now_v7(), "TRM-2411"),
            ] {
                raw(format!(
                    "insert into {TABLE} (id, organization_id, batch_number) values (?, ?, ?)"
                ))
                .bind(Uuid::now_v7())
                .bind(organization_id)
                .bind(batch_number)
                .execute(&db)
                .await?;
            }
            Ok(())
        })
    });
    cx.executor()
        .block_test(setup)
        .expect("the spike table is ready");

    let rendered = Rc::new(RefCell::new(Rendered::default()));
    let (_list, window) = cx.add_window_view(|_, _| BatchList {
        organization_id: afya,
        rendered: rendered.clone(),
    });

    draw_until(window, &rendered, &|rendered| {
        rendered.rows == vec!["AMS-2404".to_string(), "AMX-2409".to_string()]
    });
    assert!(
        !rendered.borrow().pending && rendered.borrow().failed.is_none(),
        "the list settled on this business's batches"
    );

    // A write, then an invalidation of the key the query reads under. The write goes
    // through our own pool on the database runtime, so the window's borrow of the app
    // is untouched.
    db::runtime()
        .block_on(async {
            raw(format!(
                "insert into {TABLE} (id, organization_id, batch_number) values (?, ?, ?)"
            ))
            .bind(Uuid::now_v7())
            .bind(afya)
            .bind("AFA-2413")
            .execute(our_own.pool())
            .await
        })
        .expect("the batch is written");
    window.update(|_, app| db::invalidate(TABLE, app));

    draw_until(window, &rendered, &|rendered| {
        rendered.rows.contains(&"AFA-2413".to_string())
    });

    db::runtime()
        .block_on(async {
            our_own
                .execute(&format!("drop table if exists {TABLE}"))
                .await
        })
        .expect("the spike table is dropped");
}
