macro_rules! declare_module {
    ($mod_name:ident) => {
        #[allow(unused_imports)]
        #[allow(unused_variables)]
        #[allow(dead_code)]
        #[allow(unused_mut)]
        pub mod $mod_name;
    };
}

declare_module!(day1);
declare_module!(day2);
declare_module!(day3);
declare_module!(day4);
declare_module!(day5);
declare_module!(day6);
declare_module!(day7);
declare_module!(day8);
declare_module!(day9);
declare_module!(day10);
declare_module!(day11);
declare_module!(day12);
declare_module!(day13);
declare_module!(day14);