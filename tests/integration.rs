extern crate assert_cmd;

mod integration {
    use assert_cmd::Command;

    #[test]
    fn calling_without_args() {
        let mut cmd = Command::cargo_bin("brainfuckr").unwrap();
        cmd.assert().failure();
    }

    #[test]
    fn calling_with_invalid_path() {
        let mut cmd = Command::cargo_bin("brainfuckr").unwrap();
        let assert = cmd.arg("--src").arg("doesnotexist.b").assert();
        assert.failure();
    }

    #[test]
    fn array_length() {
        let mut cmd = Command::cargo_bin("brainfuckr").unwrap();
        let assert = cmd
            .arg("--src")
            .arg("tests/test_files/array_len.b")
            .assert();
        assert.success().stdout("#\n");
    }

    #[test]
    fn io() {
        let mut cmd = Command::cargo_bin("brainfuckr").unwrap();
        let assert = cmd
            .arg("--src")
            .arg("tests/test_files/io.b")
            .write_stdin("\n")
            .assert();
        assert.success().stdout("LK\nLK\n");
    }

    #[test]
    fn unbalanced_left() {
        let mut cmd = Command::cargo_bin("brainfuckr").unwrap();
        let assert = cmd
            .arg("--src")
            .arg("tests/test_files/unbalanced_left.b")
            .assert();
        assert.failure();
    }

    #[test]
    fn unbalanced_right() {
        let mut cmd = Command::cargo_bin("brainfuckr").unwrap();
        let assert = cmd
            .arg("--src")
            .arg("tests/test_files/unbalanced_right.b")
            .assert();
        assert.failure();
    }

    #[test]
    fn obscure_code_check() {
        let mut cmd = Command::cargo_bin("brainfuckr").unwrap();
        let assert = cmd.arg("--src").arg("tests/test_files/obscure.b").assert();
        assert.success().stdout("H\n");
    }

    #[test]
    fn hello_world() {
        let mut cmd = Command::cargo_bin("brainfuckr").unwrap();
        let assert = cmd
            .arg("--src")
            .arg("tests/test_files/hello_world.b")
            .assert();
        assert.success().stdout("Hello World!\n");
    }

    #[test]
    fn addition() {
        let mut cmd = Command::cargo_bin("brainfuckr").unwrap();
        let assert = cmd.arg("--src").arg("tests/test_files/add2.b").assert();
        assert.success().stdout("7");
    }
}
