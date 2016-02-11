Empty docs:

  $ cd $TESTDIR/test-project
  $ $TESTDIR/../target/debug/cargo-readme readme --no-template -i src/no_docs.rs
  # readme-test
  \s*? (re)
  \s*? (re)


Single line:

  $ cd $TESTDIR/test-project
  $ $TESTDIR/../target/debug/cargo-readme readme --no-template -i src/single_line.rs
  # readme-test
  
  Test crate for cargo-readme


A little bit longer:

  $ cd $TESTDIR/test-project
  $ $TESTDIR/../target/debug/cargo-readme readme --no-template -i src/other.rs
  # readme-test
  
  Test crate for cargo-readme
  
  ## Level 1 heading should become level 2
