fn main() {
    {
        // ANCHOR: qualified
        assert_eq!(modules01::arithmetic::addition::add(2, 3), 5);
        assert_eq!(modules01::report::summary(), "TOTAL: 5");
        assert_eq!(modules01::summary(), "TOTAL: 5");
        // ANCHOR_END: qualified
    }
    {
        // ANCHOR: module_import
        use modules01::arithmetic::addition;
        assert_eq!(addition::add(2, 3), 5);
        // ANCHOR_END: module_import
    }
    {
        // ANCHOR: item_import
        use modules01::arithmetic::addition::add;
        assert_eq!(add(2, 3), 5);
        // ANCHOR_END: item_import
    }
    {
        // ANCHOR: use_as
        use modules01::arithmetic::addition::add as sum;
        assert_eq!(sum(2, 3), 5);
        // ANCHOR_END: use_as
    }
    {
        // ANCHOR: grouped_import
        use modules01::arithmetic::{self, addition::add as sum};
        assert_eq!(arithmetic::double(2), 4);
        assert_eq!(sum(2, 3), 5)
        // ANCHOR_END: grouped_import
    }
}
