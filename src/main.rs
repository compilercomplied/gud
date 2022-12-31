use features::feature_branches::purge_feature_branches;

mod cmd_wrapper;
mod features;

fn main() {
    purge_feature_branches();
}
