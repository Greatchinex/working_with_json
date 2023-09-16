pub mod work_with_json;

use work_with_json::read::read_json;
use work_with_json::write::write_json;

fn main() {
    read_json();
    println!("========WRITE JSON==========");
    write_json()
}
