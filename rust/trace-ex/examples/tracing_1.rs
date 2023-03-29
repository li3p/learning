use tracing::{debug, info, instrument};
use tracing_subscriber::{self, EnvFilter};

// 一个简单的函数，用于演示记录事件
#[instrument]
fn my_function() {
    info!("this is a event.");
}

fn main() {
    // 使用默认设置创建并安装订阅者
    use tracing_subscriber::fmt;

    // Configure a custom event formatter
    let format = fmt::format()
        .with_level(false) // don't include levels in formatted output
        .with_target(false) // don't include targets
        .with_thread_ids(true) // include the thread ID of the current thread
        .with_thread_names(true) // include the name of the current thread
        .compact(); // use the `Compact` formatting style.

    // Create a `fmt` subscriber that uses our custom event format, and set it
    // as the default.
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .event_format(format)
        .init();

    debug!("main started.");

    // 调用记录事件的函数
    my_function();
}
