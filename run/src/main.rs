use utils::*;
mod queries;

#[tokio::main]
async fn main() {
    let mercury = format_to_vec(queries::mercury_past_month().await);
    println!("Mercury past month:");
    for line in mercury.into_iter() {
        let (time, angle) = (line.0, line.1);
        println!("{}\t{}", time.value, angle);
    }
}
