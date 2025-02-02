# Change log

# 0.5.9

* Fix syntax error

# 0.5.8

* Feature - check revision state after deployment

# 0.5.7

* Fix secrets scripts - single command for all env vars instead of 1 for each

# 0.5.6

* eprintln for config because this is where revision startup fails

## 0.5.5

* /log from MongoDB for watched github repos

## 0.5.4

* fix - /generate/uniqueid as JSON

## 0.5.3

* feature - /generate/uniqueid

## 0.5.2

* fix - /config admin_key comparisons

## 0.5.1

* fix - /config admin_key smoke test fails, local smoke test succeeds

## 0.5.0

* feature - add, get, delete user config

## 0.4.9

* fix smoketest - remove .only

## 0.4.8

* paging from body to querystring

## 0.4.7

* fix paging for users and watches

## 0.4.6

* fix paging for user watches list

## 0.4.5

* .route("/user/:db_github_user_id/watches/list", get(db_watches_by_user_all_handler))

##  0.4.4

* /github/user/rate-limit - user's rate limit
* /github/user/token - get user by token

## 0.4.3

* `x-source-board-version` from toml

## 0.4.2

* `x-source-board-version`: 0.4.2 - the /config route has its own version `x-source-board-version-config: 0.4.1`

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
