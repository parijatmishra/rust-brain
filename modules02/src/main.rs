fn main() {
    {
        assert_eq!(modules02::arithmetic::addition::add(2, 3), 5);
        assert_eq!(modules02::report::summary(), "TOTAL: 5");
        assert_eq!(modules02::summary(), "TOTAL: 5");
    }
    {
        use modules02::arithmetic::addition;
        assert_eq!(addition::add(2, 3), 5);
    }
    {
        use modules02::arithmetic::addition::add;
        assert_eq!(add(2, 3), 5);
    }
    {
        use modules02::arithmetic::addition::add as sum;
        assert_eq!(sum(2, 3), 5);
    }
    {
        use modules02::arithmetic::{self, addition::add as sum};
        assert_eq!(arithmetic::double(2), 4);
        assert_eq!(sum(2, 3), 5)
    }
}
