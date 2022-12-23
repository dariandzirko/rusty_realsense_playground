mod pull_realsense_data;
mod sanity_check;

fn main() {
    sanity_check::run_sanity_check();
    pull_realsense_data::stream_data();
}
