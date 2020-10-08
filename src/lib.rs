use scroll_derive::SizeWith;

macro_rules! broken {
    (pub struct $name:ident { $( pub $field:ident: $t:ty, )* }) => {
        #[derive(SizeWith)]
        pub struct $name {
            $( pub $field: $t, )*
        }
    };
}

macro_rules! works {
    (pub struct $name:ident { $( pub $field:ident: $t:tt, )* }) => {
        #[derive(SizeWith)]
        pub struct $name {
            $( pub $field: $t, )*
        }
    };
}

broken! {
    pub struct BrokenStruct {
        pub build_string: [u16; 260],
        pub dbg_bld_str: [u16; 40],
    }
}

works! {
    pub struct StructThatWorks {
        pub build_string: [u16; 260],
        pub dbg_bld_str: [u16; 40],
    }
}
