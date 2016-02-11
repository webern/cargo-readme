If a project has no 'lib.rs' nor 'main.rs' and there are multiple '[[bin]]'
sections in 'Cargo.toml', fail listing the binaries available:

  $ mkdir $CRAMTMP/multiple-bin
  $ cat > $CRAMTMP/multiple-bin/Cargo.toml <<EOF
  > [package]
  > name = "multiple-bin"
  > version = "0.1.0"
  > [[bin]]
  > name = "entry1"
  > path = "src/entry1.rs"
  > [[bin]]
  > name = "entry2"
  > path = "src/entry2.rs"
  > EOF
  $ cd $CRAMTMP/multiple-bin
  $ $TESTDIR/../target/debug/cargo-readme readme
  Error: Multiple binaries found, choose one: [src/entry1.rs, src/entry2.rs]
  [1]
