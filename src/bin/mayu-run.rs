// The same program as `mayu`, under a name that the Python package's `mayu` console script can't shadow on PATH.
fn main() -> std::process::ExitCode {
    mayu::main()
}
