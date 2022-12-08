use core::time::Duration;
// use getrandom;

// fn custom_getrandom(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
//     Ok(())
// }

// getrandom::register_custom_getrandom!(custom_getrandom);

#[ic_cdk_macros::update]
pub fn main() -> () {
    ic_cdk::println!("Hello World");
    ic_cdk::timer::set_timer(Duration::new(5, 0), log_timer_called);
}

fn log_timer_called() {
    ic_cdk::println!("The timer was called!")
}

candid::export_service!();

#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn write_candid_to_disk() {
        std::fs::write("index.did", export_candid()).unwrap();
    }
}
