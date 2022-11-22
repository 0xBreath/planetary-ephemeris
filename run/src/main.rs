
mod queries;
use utils::*;
use log::LevelFilter;
use simplelog::{
  ColorChoice, Config, TermLogger, TerminalMode,
};

#[tokio::main]
async fn main() {
  init_logger();

  let matrix = queries::alignment_matrix().await;
  for (planet_a, planet_b, vec) in matrix {
    println!("{} - {}", planet_a.to_str(), planet_b.to_str());

    for (time, angle, alignment) in vec {
      //println!("{:?}", time.as_string());
      //println!("\t{}°, {:?}", angle, alignment);
    }
  }
}


fn init_logger() {
  TermLogger::init(
    LevelFilter::Info,
    Config::default(),
    TerminalMode::Mixed,
    ColorChoice::Auto,
  ).expect("failed to initialize logger");
}
