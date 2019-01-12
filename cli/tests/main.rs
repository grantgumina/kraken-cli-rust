extern crate assert_cli;

#[cfg(test)]
mod integration {
    use assert_cli;

    #[test]
    fn calling_logout() {
        assert_cli::Assert::main_binary().with_args(&["logout"])
            .succeeds()
            .unwrap();
    }

    #[test]
    fn calling_show_jobs_with_invalid_token() {
        assert_cli::Assert::main_binary()
            .with_args(&["show", "jobs"])
            .unwrap();
    }

}