//! This crate demonstates Rust comment styles.

pub mod arithmetic {
    /*! This modules contains some demo functions. */

    /// Returns the sum of `left` and `right`.
    ///
    /// # Examples
    ///
    /// ```
    /// use comments::arithmetic::add;
    ///
    /// assert_eq!(add(2, 3), 5);
    /// ```
    ///
    /// # Panics
    ///
    /// May panic on overflow when overflow checks are enabled.
    pub fn add(left: u64, right: u64) -> u64 {
        // A line comment
        /* A multi-line
            block comment
            /* with a nested block comment */
        */
        left + right
    }

    /**
     * The double function returns the double of the argument.
     */
    pub fn double(num: u64) -> u64 {
        add(num, num)
    }
}
