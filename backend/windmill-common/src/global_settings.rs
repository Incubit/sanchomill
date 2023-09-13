pub const WORKER_S3_BUCKET_SYNC: &str = "worker_s3_bucket_sync";
pub const CUSTOM_TAGS_SETTING: &str = "custom_tags";

pub const ENV_SETTINGS: [&str; 54] = [
    "DISABLE_NSJAIL",
    "DISABLE_SERVER",
    "NUM_WORKERS",
    "METRICS_ADDR",
    "JSON_FMT",
    "BASE_URL",
    "TIMEOUT",
    "ZOMBIE_JOB_TIMEOUT",
    "RESTART_ZOMBIE_JOBS",
    "SLEEP_QUEUE",
    "MAX_LOG_SIZE",
    "SERVER_BIND_ADDR",
    "PORT",
    "KEEP_JOB_DIR",
    "S3_CACHE_BUCKET",
    "TAR_CACHE_RATE",
    "COOKIE_DOMAIN",
    "PYTHON_PATH",
    "DENO_PATH",
    "GO_PATH",
    "GOPRIVATE",
    "GOPROXY",
    "NETRC",
    "PIP_INDEX_URL",
    "PIP_EXTRA_INDEX_URL",
    "PIP_TRUSTED_HOST",
    "PATH",
    "HOME",
    "DATABASE_CONNECTIONS",
    "TIMEOUT_WAIT_RESULT",
    "QUEUE_LIMIT_WAIT_RESULT",
    "DENO_AUTH_TOKENS",
    "DENO_FLAGS",
    "NPM_CONFIG_REGISTRY",
    "PIP_LOCAL_DEPENDENCIES",
    "ADDITIONAL_PYTHON_PATHS",
    "INCLUDE_HEADERS",
    "INSTANCE_EVENTS_WEBHOOK",
    "CLOUD_HOSTED",
    "GLOBAL_CACHE_INTERVAL",
    "JOB_RETENTION_SECS",
    "WAIT_RESULT_FAST_POLL_DURATION_SECS",
    "WAIT_RESULT_SLOW_POLL_INTERVAL_MS",
    "WAIT_RESULT_FAST_POLL_INTERVAL_MS",
    "EXIT_AFTER_NO_JOB_FOR_SECS",
    "REQUEST_SIZE_LIMIT",
    "SMTP_HOST",
    "SMTP_USERNAME",
    "SMTP_PORT",
    "SMTP_TLS_IMPLICIT",
    "CREATE_WORKSPACE_REQUIRE_SUPERADMIN",
    "GLOBAL_ERROR_HANDLER_PATH_IN_ADMINS_WORKSPACE",
    "MAX_WAIT_FOR_SIGTERM",
    "WORKER_GROUP",
];
