use regex::Regex;

use crate::cmd_wrapper::execute;

// -----------------------------------------------------------------------------
pub(crate) fn purge_feature_branches() {
    let branches = get_local_branches();

    remove_dev_branches(&branches);

    for line in branches {
        println!("{}", line);
    }
}

// -----------------------------------------------------------------------------
fn get_local_branches() -> Vec<String> {
    let re = Regex::new(r"^(\* |  )").unwrap();
    let mut args = Vec::new();
    args.push("branch");
    args.push("--list");

    return execute("git", &args)
        .lines()
        .map(|line| re.replace(line, "").into_owned())
        .filter(|line| line != "master" && line != "main" && line.starts_with("* "))
        .collect();
}

fn remove_dev_branches(branches: &Vec<String>) -> () {
    for branch in branches {
        let mut args = Vec::new();
        args.push("branch");
        args.push("-D");
        args.push(branch);

        let output = execute("git", &args);

        println!("{}", output);
    }
}
