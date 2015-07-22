#![allow(unused_attributes)]
#![allow(dead_code)]

macro_rules! test {
    ($i:ident,$j:ident) => {
        mod $i;

        #[test]
        fn $j() {
            $i::main();
        }
    }
}

test!(example01, test01);

test!(example01, test01);

// test!(example02, test02);
test!(example03, test03);
test!(example04, test04);
test!(example05, test05);
// test!(example06, test06);
// test!(example07, test07);
test!(example08, test08);
