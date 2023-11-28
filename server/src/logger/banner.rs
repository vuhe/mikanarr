static BANNER: &str = r"
           _ _                               
          (_) |                              
 _ __ ___  _| | ____ _ _ __   __ _ _ __ _ __ 
| '_ ` _ \| | |/ / _` | '_ \ / _` | '__| '__|
| | | | | | |   < (_| | | | | (_| | |  | |   
|_| |_| |_|_|_|\_\__,_|_| |_|\__,_|_|  |_|   
";

pub(super) fn print_banner() {
    tracing::info!("version: {} {}", env!("CARGO_PKG_VERSION"), BANNER);
}
