#[cfg(feature = "gui")]
use fltk::app;
use log::{error, info, warn};

#[cfg(feature = "gui")]
use crate::globals::statics::LOGCHANNEL;

/// `ui_log`
/// - log a messgae to the terminal and the logfile
/// - send a logmessage to the textbox on the Crossbeam LOGCHANNEL ig runing the GUI
pub fn ui_log(s: &str) {
    let cat: &str = &s[..2];
    match cat {
        "*W" => warn!("tb_log: {}", s),
        "*E" => error!("tb_log: {}", s),
        _ => info!("tb_log: {}", s),
    };
    #[cfg(feature = "gui")]
    {
        let logger = &LOGCHANNEL.read().0;
        logger.send(s.to_string()).unwrap();
        app::awake();
        std::thread::yield_now();
    }
}
