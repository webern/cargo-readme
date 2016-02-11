If a project has no 'lib.rs' nor 'main.rs' and no entrypoint was specified, fail:

  $ mkdir $CRAMTMP/no-entrypoint
  $ cat > $CRAMTMP/no-entrypoint/Cargo.toml <<EOF
  > [package]
  > name = "no-entrypoint"
  > version = "0.1.0"
  > EOF
  $ cd $CRAMTMP/no-entrypoint
  $ $TESTDIR/../target/debug/cargo-readme readme
  Error: No entrypoint found
  [1]
