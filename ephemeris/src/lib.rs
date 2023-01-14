pub mod quantities;
pub mod query;
pub mod step_size;
pub mod target;
pub mod time;
pub mod data_type;
pub mod alignment;
pub mod origin;

pub use quantities::*;
pub use query::*;
pub use step_size::*;
pub use target::*;
pub use time::*;
pub use data_type::*;
pub use alignment::*;
pub use origin::*;

pub const TICKER_DATA_PATH: &str = "BTCUSD.csv";
pub const PRICE_PLANET_RESULTS_PATH: &str = "price_planet_results.txt";
pub const PLANET_MATRIX_RESULTS_PATH: &str = "planet_matrix_results.txt";
pub const RETROGRADE_RESULTS_CSV: &str = "retrograde_results.csv";
pub const TICKER_DATAFRAME_CSV: &str = "ticker_dataframe.csv";
pub const SOLAR_ECLIPSE_CSV: &str = "./solar_eclipse.csv";
pub const LUNAR_ECLIPSE_CSV: &str = "./lunar_eclipse.csv";