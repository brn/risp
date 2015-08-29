///!
///!
///!


macro_rules! kb {
    ($val:expr) => {
        $val * 1024
    }
}


macro_rules! mb {
    ($val:expr) => {
        kb!($val) * 1024
    }
}


macro_rules! gb {
    ($val:expr) => {
        mb!($val) * 1024
    }
}


macro_rules! ast_add_child {
    ($ast: expr, $child: expr) => {
        match *$ast {
            Ast::Form(ref f) => {
                f.add_child($child);
            }
            Ast::Module(ref f) => {
                f.add_child($child);
            },
            _ => {}
        }
    }
}
