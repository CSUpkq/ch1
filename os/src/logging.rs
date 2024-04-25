use log::{self, Level, LevelFilter, Log, Metadata, Record};

struct SimpleLogger;

//用Loggers 实现 Log trait
impl Log for SimpleLogger {
	//确定是否记录，低于设置level的元数据都不会被记录，由于允许最低Trace，直接设置为True
    fn enabled(&self, _metadata: &Metadata) -> bool {
		//metadata.level() <= Level::Trace
        true
    }
    fn log(&self, record: &Record) {
        //if !self.enabled(record.metadata()) {
        //    return;
        //}
        let color = match record.level() {
            Level::Error => 31, // Red
            Level::Warn => 93,  // BrightYellow
            Level::Info => 34,  // Blue
            Level::Debug => 32, // Green
            Level::Trace => 90, // BrightBlack
        };
        println!(
            "\u{1B}[{}m[{:>5}] {}\u{1B}[0m",
            color,
            record.level(),
            record.args(),
        );
    }
	//刷新缓冲记录
    fn flush(&self) {}
}

pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
	//通过set_logger安装logger，把全局日志记录器设置为静态日志
    log::set_logger(&LOGGER).unwrap();
	//设置max级别以过滤以下级别
	//环境变量LOG
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => LevelFilter::Error,
        Some("WARN") => LevelFilter::Warn,
        Some("INFO") => LevelFilter::Info,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        _ => LevelFilter::Trace, // 默认输出等级为 INFO
    });
}


