use sqlx::{sqlite::SqliteQueryResult, Pool, Row, Sqlite, SqlitePool};

#[derive(serde::Deserialize, Debug)]
pub struct TaskRegisterd {
    pub id: i64,
    pub task: String,
    pub status: i32,
    pub created_at: Option<String>,
    pub due_at: Option<String>,
    pub started_at: Option<String>,
    pub done_at: Option<String>,
}

pub async fn init_db_pool(database_url: &str) -> Pool<Sqlite> {
    // 接続プールの作成
    SqlitePool::connect(database_url).await.unwrap()
}

pub async fn get_task_list(pool: &SqlitePool, status: i32) -> Vec<TaskRegisterd> {
    const SQL: &str = include_str!("sql/get_task_list.sql");
    let rows = sqlx::query(SQL).bind(status).fetch_all(pool).await.unwrap();
    let tasks: Vec<TaskRegisterd> = rows
        .iter()
        .map(|row| TaskRegisterd {
            id: row.get::<i64, _>("id"),
            task: row.get::<String, _>("task"),
            status: row.get::<i32, _>("status"),
            created_at: row
                .try_get::<Option<String>, _>("created_at")
                .unwrap_or(None),
            due_at: row.try_get::<Option<String>, _>("due_at").unwrap_or(None),
            started_at: row
                .try_get::<Option<String>, _>("started_at")
                .unwrap_or(None),
            done_at: row.try_get::<Option<String>, _>("done_at").unwrap_or(None),
        })
        .collect();
    tasks
}

pub async fn add_task(
    pool: &SqlitePool,
    task_value: String,
    due_at_value: String,
) -> Result<i64, sqlx::Error> {
    const SQL: &str = include_str!("sql/add_task.sql");
    sqlx::query(SQL)
        .bind(task_value)
        .bind(due_at_value)
        .execute(pool)
        .await?;

    // last_insert_rowidを取得
    let id = sqlx::query_scalar("SELECT last_insert_rowid()")
        .fetch_one(pool)
        .await?;

    Ok(id)
}

pub async fn start_task(pool: &SqlitePool, id: i64) -> SqliteQueryResult {
    const SQL: &str = include_str!("sql/start_task.sql");
    sqlx::query(SQL).bind(id).execute(pool).await.unwrap()
}

pub async fn done_task(pool: &SqlitePool, id: i64) -> SqliteQueryResult {
    const SQL: &str = include_str!("sql/done_task.sql");
    sqlx::query(SQL).bind(id).execute(pool).await.unwrap()
}

/// Reverts the status of a task to "not started".
///
/// This function updates the `status` of a task in the `tasks` table to 0 and sets
/// the `started_at` column to NULL, effectively marking the task as not started.
///
/// # Arguments
/// - `pool`: A reference to the `SqlitePool` wrapped in Actix Web's `web::Data`.
/// - `id`: The unique identifier of the task to be undone.
///
/// # Returns
/// Returns a `SqliteQueryResult` representing the result of the SQL query.
///
/// # Errors
/// This function will panic if the SQL query execution fails.
///
/// Note: This function is intended to be used within an Actix Web application,
/// where the `SqlitePool` is properly configured and managed by the framework.
pub async fn undo_task(pool: &SqlitePool, id: i64) -> SqliteQueryResult {
    const SQL: &str = include_str!("sql/undo_task.sql");
    sqlx::query(SQL).bind(id).execute(pool).await.unwrap()
}

pub async fn doing_task(pool: &SqlitePool, id: i64) -> SqliteQueryResult {
    const SQL: &str = include_str!("sql/doing_task.sql");
    sqlx::query(SQL).bind(id).execute(pool).await.unwrap()
}

pub async fn remove_task(pool: &SqlitePool, id: i64) -> SqliteQueryResult {
    const SQL: &str = include_str!("sql/remove_task.sql");
    sqlx::query(SQL).bind(id).execute(pool).await.unwrap()
}

pub async fn get_task(pool: &SqlitePool, id: i64) -> Result<TaskRegisterd, sqlx::Error> {
    const SQL: &str = include_str!("sql/get_task.sql");
    let row = sqlx::query(SQL).bind(id).fetch_one(pool).await?;

    // TaskRegisterd構造体に変換
    let task = TaskRegisterd {
        id: row.get::<i64, _>("id"),
        task: row.get::<String, _>("task"),
        status: row.get::<i32, _>("status"),
        created_at: row
            .try_get::<Option<String>, _>("created_at")
            .unwrap_or(None),
        due_at: row.try_get::<Option<String>, _>("due_at").unwrap_or(None),
        started_at: row
            .try_get::<Option<String>, _>("started_at")
            .unwrap_or(None),
        done_at: row.try_get::<Option<String>, _>("done_at").unwrap_or(None),
    };

    Ok(task)
}

#[cfg(test)]
mod tests {
    use super::*; // undo_task関数をインポート
    use sqlx::SqlitePool;

    /// ヘルパー関数: インメモリーモードでSQLiteデータベースを初期化
    async fn setup_test_db() -> SqlitePool {
        // SQLiteのインメモリーモードを使用
        let database_url = "sqlite::memory:";

        // データベース接続プールを作成
        let pool = init_db_pool(database_url).await;

        // // マイグレーション用のパスを設定（migrations ディレクトリ）
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }

    #[tokio::test]
    async fn test_add_task() {
        let pool = setup_test_db().await;

        // 関数を呼び出して、タスクを追加
        let _id = add_task(
            &pool,
            "test_task001".to_string(),
            "2025-02-23T00:00:00Z".to_string(),
        )
        .await
        .unwrap();

        // 結果を検証
        let result = get_task(&pool, 1).await.unwrap();
        // println!("Task found: {:?}", result);

        // 結果検証
        assert_eq!(result.task, "test_task001".to_string()); // タスク
        assert_eq!(result.status, 0); // ステータスが未着手
        assert!(result.created_at.is_some()); // created_atはNULL以外
        assert!(result.due_at.is_some()); // due_atはNULL以外
        assert!(result.started_at.is_none()); // started_atはNULL
        assert!(result.done_at.is_none()); // done_atはNULL
    }

    #[tokio::test]
    async fn test_start_task() {
        let pool = setup_test_db().await;

        // 関数を呼び出して、タスクを追加
        let _id = add_task(
            &pool,
            "test_task001".to_string(),
            "2025-02-23T00:00:00Z".to_string(),
        )
        .await
        .unwrap();

        // タスクを開始
        start_task(&pool, 1).await;

        // 結果を検証
        let result = get_task(&pool, 1).await.unwrap();

        // 結果検証
        assert_eq!(result.task, "test_task001".to_string()); // タスク
        assert_eq!(result.status, 1); // ステータスが着手
        assert!(result.created_at.is_some()); // created_atはNULL以外
        assert!(result.due_at.is_some()); // due_atはNULL以外
        assert!(result.started_at.is_some()); // started_atはNULL以外
        assert!(result.done_at.is_none()); // done_atはNULL
    }

    #[tokio::test]
    async fn test_done_task() {
        let pool = setup_test_db().await;

        // 関数を呼び出して、タスクを追加
        let _id = add_task(
            &pool,
            "test_task001".to_string(),
            "2025-02-23T00:00:00Z".to_string(),
        )
        .await
        .unwrap();

        // タスクを開始
        start_task(&pool, 1).await;

        // タスクを終了
        done_task(&pool, 1).await;

        // 結果を検証
        let result = get_task(&pool, 1).await.unwrap();

        // 結果検証
        assert_eq!(result.task, "test_task001".to_string()); // タスク
        assert_eq!(result.status, 9); // ステータスが完了
        assert!(result.created_at.is_some()); // created_atはNULL以外
        assert!(result.due_at.is_some()); // due_atはNULL以外
        assert!(result.started_at.is_some()); // started_atはNULL以外
        assert!(result.done_at.is_some()); // done_atはNULL以外
    }

    #[tokio::test]
    async fn test_undo_task() {
        let pool = setup_test_db().await;

        // 関数を呼び出して、タスクを追加
        let _id = add_task(
            &pool,
            "test_task001".to_string(),
            "2025-02-23T00:00:00Z".to_string(),
        )
        .await
        .unwrap();

        // タスクを開始
        start_task(&pool, 1).await;

        // タスクを未着手に戻す
        undo_task(&pool, 1).await;

        // 結果を検証
        let result = get_task(&pool, 1).await.unwrap();

        // 結果検証
        assert_eq!(result.task, "test_task001".to_string()); // タスク
        assert_eq!(result.status, 0); // ステータスが未着手
        assert!(result.created_at.is_some()); // created_atはNULL以外
        assert!(result.due_at.is_some()); // due_atはNULL以外
        assert!(result.started_at.is_none()); // started_atはNULL
        assert!(result.done_at.is_none()); // done_atはNULL
    }

    #[tokio::test]
    async fn test_doing_task() {
        let pool = setup_test_db().await;

        // 関数を呼び出して、タスクを追加
        let _id = add_task(
            &pool,
            "test_task001".to_string(),
            "2025-02-23T00:00:00Z".to_string(),
        )
        .await
        .unwrap();

        // タスクを開始
        start_task(&pool, 1).await;

        // タスクを終了
        done_task(&pool, 1).await;

        // タスクを仕掛かり中に戻す
        doing_task(&pool, 1).await;

        // 結果を検証
        let result = get_task(&pool, 1).await.unwrap();

        // 結果検証
        assert_eq!(result.task, "test_task001".to_string()); // タスク
        assert_eq!(result.status, 1); // ステータスが仕掛かり中
        assert!(result.created_at.is_some()); // created_atはNULL以外
        assert!(result.due_at.is_some()); // due_atはNULL以外
        assert!(result.started_at.is_some()); // started_atはNULL以外
        assert!(result.done_at.is_none()); // done_atはNULL
    }

    #[tokio::test]
    async fn test_remove_task() {
        let pool = setup_test_db().await;

        // 関数を呼び出して、タスクを追加
        let _id = add_task(
            &pool,
            "test_task001".to_string(),
            "2025-02-23T00:00:00Z".to_string(),
        )
        .await
        .unwrap();

        // タスクを削除
        remove_task(&pool, 1).await;

        // 結果を検証
        let result = get_task(&pool, 1).await;

        // 結果検証
        assert!(result.is_err()); // 該当なし
    }

    #[tokio::test]
    async fn test_get_task_list_unstarted() {
        let pool = setup_test_db().await;

        // タスクを追加
        add_task(
            &pool,
            "test_task001".to_string(),
            "2025-02-23T00:00:00Z".to_string(),
        )
        .await
        .unwrap();

        // 未着手のタスクを取得
        let unstarted_tasks = get_task_list(&pool, 0).await;
        let unstarted_task = unstarted_tasks.first().unwrap();

        // 結果検証
        assert_eq!(unstarted_task.task, "test_task001".to_string()); // タスク
        assert_eq!(unstarted_task.status, 0); // ステータスが未着手
        assert!(unstarted_task.created_at.is_some()); // created_atはNULL以外
        assert!(unstarted_task.due_at.is_some()); // due_atはNULL以外
        assert!(unstarted_task.started_at.is_none()); // started_atはNULL
        assert!(unstarted_task.done_at.is_none()); // done_atはNULL
    }

    #[tokio::test]
    async fn test_get_task_list_in_progress() {
        let pool = setup_test_db().await;

        // 関数を呼び出して、タスクを追加
        add_task(
            &pool,
            "test_task001".to_string(),
            "2025-02-23T00:00:00Z".to_string(),
        )
        .await
        .unwrap();

        add_task(
            &pool,
            "test_task002".to_string(),
            "2025-02-23T00:00:00Z".to_string(),
        )
        .await
        .unwrap();

        add_task(
            &pool,
            "test_task003".to_string(),
            "2025-02-23T00:00:00Z".to_string(),
        )
        .await
        .unwrap();

        // タスクを開始
        start_task(&pool, 1).await;

        // 仕掛かり中のタスクを取得
        let in_progress_tasks = get_task_list(&pool, 1).await;
        let in_progress_task = in_progress_tasks.first().unwrap();

        // 結果検証
        assert_eq!(in_progress_task.task, "test_task001".to_string()); // タスク
        assert_eq!(in_progress_task.status, 1); // ステータスが仕掛かり中
        assert!(in_progress_task.created_at.is_some()); // created_atはNULL以外
        assert!(in_progress_task.due_at.is_some()); // due_atはNULL以外
        assert!(in_progress_task.started_at.is_some()); // started_atはNULL以外
        assert!(in_progress_task.done_at.is_none()); // done_atはNULL
    }

    #[tokio::test]
    async fn test_get_task_list_in_completed() {
        let pool = setup_test_db().await;

        // 関数を呼び出して、タスクを追加
        add_task(
            &pool,
            "test_task001".to_string(),
            "2025-02-23T00:00:00Z".to_string(),
        )
        .await
        .unwrap();

        // タスクを開始
        start_task(&pool, 1).await;

        // タスクを終了
        done_task(&pool, 1).await;

        // // 完了タスクを取得
        let completed_tasks = get_task_list(&pool, 9).await;
        let completed_task = completed_tasks.first().unwrap();

        // 結果検証
        assert_eq!(completed_task.task, "test_task001".to_string()); // タスク
        assert_eq!(completed_task.status, 9); // ステータスが完了
        assert!(completed_task.created_at.is_some()); // created_atはNULL以外
        assert!(completed_task.due_at.is_some()); // due_atはNULL以外
        assert!(completed_task.started_at.is_some()); // started_atはNULL以外
        assert!(completed_task.done_at.is_some()); // done_atはNULL
    }
}
