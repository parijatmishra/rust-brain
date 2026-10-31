fn main() {
    // ANCHOR: output
    #[derive(Debug)]
    struct Job {
        name: &'static str,
        completed: u32,
    }

    let job = Job {
        name: "import",
        completed: 2,
    };
    println!("job: {:?}", job); // compact debug output on stdout
    println!("job (pretty): {:#?}", job); // pretty debug output on stdout
    eprintln!("checking {}: {} completed", job.name, job.completed); // stderr
    // ANCHOR_END: output

    // ANCHOR: dbg
    let name = String::from("import");
    dbg!(&name); // inspect without moving the string
    let name = dbg!(name); // moves the string, then returns it
    println!("inspected: {name}");
    // ANCHOR_END: dbg

    // ANCHOR: assertions
    let actual = 2;
    let expected = 2;
    assert!(actual > 0, "completed count must be positive");
    assert_eq!(actual, expected, "completed count mismatch");
    assert_ne!(actual, 0, "completed count must not be zero");
    // ANCHOR_END: assertions

    // ANCHOR: debug_assertions
    let count = 2;
    debug_assert!(count > 0, "completed count must be positive");
    debug_assert_eq!(count, 2);
    debug_assert_ne!(count, 0);
    // ANCHOR_END: debug_assertions

    if std::env::args().any(|arg| arg == "--fail") {
        // ANCHOR: failed_assertion
        let actual = 2;
        let expected = 3;
        assert_eq!(actual, expected, "completed count mismatch");
        // ANCHOR_END: failed_assertion
    }

    if std::env::args().any(|arg| arg == "--debug-fail") {
        // ANCHOR: failed_debug_assertion
        debug_assert_eq!(2, 3, "debug assertions are enabled");
        // ANCHOR_END: failed_debug_assertion
    }
}
