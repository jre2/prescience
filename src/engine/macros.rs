#[macro_export]
macro_rules! group {
    ( $v:ident; $( $e:expr ),* ) => {
        const $v : usize = ($( ( 1 << ($e as usize) ) + )*  0);
    }
}
