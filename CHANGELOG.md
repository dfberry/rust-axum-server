# Change log

## 0.4.2

* .route("/github/user/rate-limit", get(github_get_user_rate_limit_handler))
* Debug output JSON to file
* Stage resource, stage deploy action

## 0.4.1

* Token is temporarily optional in json body, suplimented by BE PAT

## 0.4.0

* New routes
* Middleware tracing
* REST client http files
* Direct to GitHub API with reqwest

### Old routes

```
let app = Router::new()
    .route("/", get(root_get_handler))
    .route("/github/user", get(github_get_user_handler))
    .route("/github/repo", post(github_post_repo_handler))
    .route("/github/query/issue", post(github_post_query_issue_handler))
    .route("/github/repos/stats", post(github_post_repo_stats_handler))
    .route("/user", post(db_user_new_handler))
    .route("/users", get(db_users_all_handler))
    .route("/user/:username/watch", post(db_watch_new_handler))
    .route("/user/:username/watches", get(db_watches_all_handler))
    .route("/config", get(handler_get_config))
    .layer(Extension(shared_state.clone()));
```

### New routes

```
let app = Router::new()
    .route("/", get(root_get_handler))
    .route("/github/user/:username", get(github_get_user_profile_handler))   // NEW
    .route("/github/repo/issues", get(github_get_repo_issues_handler))       // NEW
    .route("/github/repo/prs", get(github_get_repo_prs_handler))             // NEW
    .route("/github/query", get(github_get_query_handler))                   // NEW
    .route("/github/user", post(github_get_user_handler))
    .route("/github/repo", post(github_post_repo_handler))
    .route("/github/query/issue", post(github_post_query_issue_handler))
    .route("/github/repos/stats", post(github_post_repo_stats_handler))
    .route("/user", post(db_user_new_handler))
    .route("/users", get(db_users_all_handler))
    .route("/user/:username/watch", post(db_watch_new_handler))
    .route("/user/:username/watches", get(db_watches_all_handler))
    .route("/config", get(handler_get_config))
    .layer(Extension(shared_state.clone()))
    .layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .into_inner(),
    );
```

## 0.3.4

* New Repo properties

## 0.3.3

* Remove dotenvy - required/failed on file not found

## 0.3.1

* Remove reading toml for now

## 0.3.0

Both client and server in Azure Container Apps

* /admin returns UTC timestamp in JSON

## 0.2.4

* dotenv -> dotenvy

## 0.2.3

* Last commit date in stats
* `x-source-board-version` header for version as middleware
* string utils for org_or_user and repo name
* Fix: return 400, 401 when relevant
