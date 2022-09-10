#![allow(dead_code, unused, unused_macros)]

#[macro_use]
extern crate eager;
#[macro_use]
extern crate pixel_caster;

use pixel_caster::{PixelsCollection, bgra_management::ColorAlteration, PixelValues};

#[test]
fn main() {
    /*let image_u8_bgra = PixelsCollection::from_png("media/Logo_MK7_Transparent_Bg_ColorsWithHalfAlpha.png").unwrap();

    //BYTES_MATCHERS_VEC[0].1


    // functions are faster than closures as parameters for functions
    // functions are faster than closures as parameters for macros
    // macros with functions/closures as parameters seems as fast as/slightly slower than functions with functions/closures as parameters
    // macros with macros inside them are 30% faster than macros with functions/closures as parameters
    // actual code instead of macros/functions is faster
    // actual code > macros > functions (functions with functions/closures as parameters > macros with functions/closures as parameters)

    create_color_matcher!(not_fully_opaque (_,_,_,a) bool, (a < 255));
    create_color_matcher_or_alterator!(low_red (slice, index), (b,g,r,a) (), (b = 0, g = 0, r = 100, a = 255));


    // fastest
    let now = std::time::Instant::now();
    for _ in 0..1000 {
        let mut img = image_u8_bgra.bytes.clone();
        let mut i = 0;
        for _ in 0..img.len()/(PixelValues::get_units_per_pixel(&img[0]) as usize) {
            if img[i+3] < 255 {
                img[i] = 0; img[i+1] = 0; img[i+2] = 100; img[i+3] = 255;
            }
            i += 4;
        }
    }
    println!("{}", now.elapsed().as_millis());

    // fast
    let now = std::time::Instant::now();
    for _ in 0..1000 {
        let mut img = image_u8_bgra.bytes.clone();
        color_match_and_alter!(img, not_fully_opaque => low_red);
    }
    println!("{}", now.elapsed().as_millis());

    // slow
    let now = std::time::Instant::now();
    for _ in 0..1000 {
        let mut img = image_u8_bgra.bytes.clone();
        img.color_matcher_and_alterator(bytes_matchers::not_fully_opaque, bytes_alterators::low_red);
    }
    println!("{}", now.elapsed().as_millis());

    // slowest
    let now = std::time::Instant::now();
    for _ in 0..1000 {
        let mut img = image_u8_bgra.bytes.clone();
        img.color_matcher_and_alterator(|_:u8,_:u8,_:u8,v3:u8| -> bool {v3 < 255}, |bgra: &mut[u8]| { bgra[0] = 0; bgra[1] = 0; bgra[2] = 100; bgra[3] = 255;});
    }
    println!("{}", now.elapsed().as_millis());

    
    // new
    let now = std::time::Instant::now();
    for _ in 0..1000 {
        let mut img = image_u8_bgra.bytes.clone();
        color_match_and_alter!(img, not_fully_opaque => low_red);
    }
    println!("{}", now.elapsed().as_millis());*/
    
    create_color_matcher_or_alterator!(full_opacity (_:u8,_:u8,_:u8,a:u8) bool, (a >= 255));
    let bgra_vec = vec![30_u8,120,120,255];
    assert!(full_opacity!(bgra_vec[0], bgra_vec[1], bgra_vec[2], bgra_vec[3]));
    assert!(full_opacity(bgra_vec[0], bgra_vec[1], bgra_vec[2], bgra_vec[3]));

    /// Now we can pass the created function to a Vec method that takes a function as a statement to check on each 4 values chunks of the vector and change their values to the 4 provided ones when the statement was matched
    /// In this case all the 3 values chunks are fully opaque (Alpha = 255), therefore they all will have their 4 values alterated into the provided ones (255,255,255,255)
    let mut bgra_3vec = vec![30_u8,120,120,255, 30_u8,120,120,255, 30_u8,120,120,255];
    bgra_3vec.color_matcher_and_new_color(full_opacity, 255,255,255,255);
    assert_eq!(bgra_3vec, vec![255,255,255,255, 255,255,255,255, 255,255,255,255]);

    create_color_matcher_or_alterator!(max_blue_full_opacity (a:u8,b:u8,_:u8,_:u8) bool, (b == 255, a == 255));
    let abgr_vec = vec![255,255,0,0];
    assert!(max_blue_full_opacity!(abgr_vec[0], abgr_vec[1], abgr_vec[2], abgr_vec[3]));
    assert!(max_blue_full_opacity(abgr_vec[0], abgr_vec[1], abgr_vec[2], abgr_vec[3]));
    
    create_color_matcher_or_alterator!(slice_full_opacity (slice, &[u8]), (b,g,r,a) bool, (a >= 255));
    let bgra_vec = vec![30_u8,120,120,255];
    let bgra_vec_slice = &bgra_vec[0..=3];
    assert!(slice_full_opacity!(bgra_vec_slice));
    assert!(slice_full_opacity(bgra_vec_slice));

    ///// Now we can pass the created function to a Vec method that takes a function as a statement to check on each 4 values chunks of the vector and change their values to the 4 provided ones when the statement was matched
    ///// In this case all the 3 values chunks are fully opaque (Alpha = 255), therefore they all will have their 4 values alterated into the provided ones (255,255,255,255)
    let mut bgra_3vec = vec![30_u8,120,120,255, 30_u8,120,120,255, 30_u8,120,120,255];
    bgra_3vec.color_matcher_and_new_color(full_opacity, 255,255,255,255);
    assert_eq!(bgra_3vec, vec![255,255,255,255, 255,255,255,255, 255,255,255,255]);

    create_color_matcher_or_alterator!(slice_max_blue_full_opacity (slice, &[u8]), (a,b,g,r) bool, (b == 255, a == 255));
    let abgr_vec = vec![255_u8,255,0,0];
    let abgr_vec_slice = &abgr_vec[0..=3];
    assert!(slice_max_blue_full_opacity!(abgr_vec_slice));
    assert!(slice_max_blue_full_opacity(abgr_vec_slice));

    create_color_matcher_or_alterator!(max_blue_full_opacity_inception (a:u8,b:u8,_:u8,_:u8) bool, (b == 255, a == 255));
    max_blue_full_opacity_inception!(255_u8,255,0,0);

    create_color_matcher_or_alterator!(max_blue_max_alpha (a:&mut u8,b:&mut u8,_:&mut u8,r:&mut u8) (), (b = 255_u8, a = 255_u8));
    let mut bb = 24_u8;
    let mut gg = 24_u8;
    let mut rr = 24_u8;
    let mut aa = 24_u8;
    max_blue_max_alpha!(bb,gg,rr,aa);
    println!("b{} g{} r{} a{}",bb,gg,rr,aa);
    let mut bbb = 24_u8;
    let mut ggg = 24_u8;
    let mut rrr = 24_u8;
    let mut aaa = 24_u8;
    max_blue_max_alpha(&mut bbb, &mut ggg, &mut rrr, &mut aaa);
    println!("b{} g{} r{} a{}",bbb,ggg,rrr,aaa);


    create_color_matcher_or_alterator!(slice_max_blue_max_alpha (slice, &mut[u8]), (b,g,r,a) (), (a = 255));
    let mut bgra_vec = vec![30_u8,120,120,33, 40,13,44,22];
    slice_max_blue_max_alpha!(bgra_vec[0..=3]);
    assert_eq!(bgra_vec[0..=3], [30_u8,120,120,255]); // asserts the macro
    slice_max_blue_max_alpha(&mut bgra_vec[4..=7]);
    assert_eq!(bgra_vec[4..=7], [40,13,44,255]); // asserts the function
    println!("{:?}", bgra_vec);

    create_color_matcher_or_alterator!(not_fully_opaque (_:u8,_:u8,_:u8,a:u8) bool, (a < 255));
    create_color_matcher_or_alterator!(low_red (slice &mut [u8], index), (b,g,r,a) (), (b = 0, g = 0, r = 100, a = 255));
    let mut bgra_vec = vec![30_u8,120,120,255, 40,13,44,22];
    for _ in 0..bgra_vec.len()/4 {
        color_match_and_alter!(bgra_vec, not_fully_opaque => low_red);
    }
    println!("{:?}", bgra_vec);
    assert_eq!(bgra_vec[0..=3], [30_u8,120,120,255]);
    assert_eq!(bgra_vec[4..=7], [0,0,100,255]);


    println!("ended");
    
    
    create_color_matcher_or_alterator!(slice_max_blue_max_alphaxxx (slice, &[u8]), (b,g,r,a) bool, (a == 255, b == 33));
    let mut tmp = vec![33,120,120,255, 40,13,44,22];
    let res = slice_max_blue_max_alphaxxx!(tmp[0..=3]);
    println!("{}", res);
    create_color_matcher_or_alterator!(slice_max_blue_max_alphayyy (slice, &mut [u8]), (b,g,r,a) (), (a = 100, b = 22));



    create_color_matcher_or_alterator!(not_fully_opaquexxxx (b:u8,g:u8,r:u8,a:u8) bool, (a < 255));
    let mut tmpxxxx = vec![33,120,120,233, 40,13,44,22];
    println!("{}", not_fully_opaquexxxx(0,0,0,tmpxxxx[3]));
    create_color_matcher_or_alterator!(max_blue_max_alphayyyy (b:&mut u8,g:&mut u8,r:&mut u8,a:&mut u8) (), (b = 200_u8, a = 255_u8));
    let mut bb = 10_u8;
    let mut aa = 233_u8;
    max_blue_max_alphayyyy(&mut bb,&mut 0,&mut 0,&mut aa);
    max_blue_max_alphayyyy!(bb,0,0,aa);
    println!("bb{} aa{}", bb, aa);
    create_color_matcher_or_alterator!(nobbb (b:u8,g:u8,r:u8,a:u8) bool, (a < 255));
    let mut tmpxxxx = vec![33_u8,120,120,233, 40,13,44,22];
    println!("fn {}", nobbb(0,0,0,tmpxxxx[3]));
    println!("macro {}", nobbb!(0,0,0,tmpxxxx[3]));
    create_color_matcher_or_alterator!(noxxx (b:&mut u8,g:&mut u8,r:&mut u8,a:&mut u8) (), (b = 255_u8, a = 255_u8));

    
    create_color_matcher_or_alterator!(tsts (b:u8,g:u8,_:u8,a:u8) bool, (a < 255));
    let mut tmpxxxx = vec![33,120,120,233, 40,13,44,22];
    println!("{}", tsts(0,0,0,tmpxxxx[3]));
    println!("{}", tsts!(0,0,0,tmpxxxx[3]));
    print_item_resolve_later_demonstration!(test_x (b,g,_,a), (a < 33, g > 4));
    println!("fn (0,5,40,22) : {}", test_x(0,5,40,22));
    println!("macro (2,4,100,66) : {}", test_x!(2,4,100,66));
    
    create_color_matcher_or_alterator!(all_to_zero (slice, &mut [u8]), (b,g,r,a) (), (b = 0, g = 0, r = 0));

    //quiqui
    
}



#[macro_export]
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

#[macro_export]
macro_rules! make_println {
    ($name:ident, $fmt:expr) => {
        with_dollar_sign! {
            ($d:tt) => {
                macro_rules! $name {
                    ($d($d args:expr),*) => {
                        println!($fmt, $d($d args),*);
                    }
                }
            }
        }
    };
}


#[macro_export]
macro_rules! __resolve_at_run {
    ( $name:tt ) => { $name };
}

eager_macro_rules!{ $eager_colors_matchers_and_alterators
    #[macro_export]
    macro_rules! and_operation_or_semicolon {
        ( bool ) => { && };
        ( () ) => { ; };
    }
    #[macro_export]
    macro_rules! ending {
        ( bool ) => { true };
        ( () ) => {};
    }
    #[macro_export]
    macro_rules! fn_creator_new {
        // specify by hand or $variable &[u8] -> bool because if obtained by calling a macro the macro call will be shown when the cursor goes above the function, instead of the actual type and return type
        ( $name:ident ($c1_name:tt : $c1_input_type:ty, $c2_name:tt : $c2_input_type:ty, $c3_name:tt : $c3_input_type:ty, $c4_name:tt : $c4_input_type:ty) bool ) => { paste::item! { pub fn [<$name>]([<$c1_name>]: $c1_input_type,[<$c2_name>]: $c2_input_type, [<$c3_name>]: $c3_input_type,[<$c4_name>]: $c4_input_type) -> bool { $name!(__resolve_at_run!([<$c1_name>]), __resolve_at_run!([<$c2_name>]), __resolve_at_run!([<$c3_name>]), __resolve_at_run!([<$c4_name>])) } } };
        ( $name:ident ($c1_name:tt : $c1_input_type:ty, $c2_name:tt : $c2_input_type:ty, $c3_name:tt : $c3_input_type:ty, $c4_name:tt : $c4_input_type:ty) $return_type:tt ) => { paste::item! { pub fn [<$name>]([<$c1_name>]: $c1_input_type,[<$c2_name>]: $c2_input_type, [<$c3_name>]: $c3_input_type,[<$c4_name>]: $c4_input_type) -> $return_type { $name!( $c1_input_type, __resolve_at_run!([<$c1_name>]), __resolve_at_run!([<$c2_name>]), __resolve_at_run!([<$c3_name>]), __resolve_at_run!([<$c4_name>])) } } };
        // find a way to set a macro case for when is a value and when is a &mut value and apply it to the calling macro, so that these 2 above can become just 1
        ( $name:ident $color_slice_name:tt $input_type:ty, $return_type:tt ) => { pub fn $name($color_slice_name: $input_type) -> $return_type { $name!(*$color_slice_name) } };
        //pub fn [<$name>]([<$color_slice_name>]: &mut [u8], index: usize) -> $return_type { $name!(*[<$color_slice_name>], index) }
        ( $name:ident $color_slice_name:tt $input_type:ty, $color_index_name:tt, $return_type:tt ) => { pub fn $name($color_slice_name: $input_type, $color_index_name: usize) -> $return_type { $name!(*$color_slice_name, $color_index_name) } };
        // println!("{}", $c1_input_type);
    }
    /// Creates a bytes matcher for 4 values (e.g. BGRA ordered bytes) in a macro and in a function form (the macro one is faster, the function one can be used as parameter)
    /// 
    /// # Examples
    /// 
    /// ```
    /// create_color_matcher_or_alterator!(full_opacity ( _, _, _, a), (a >= 255));
    /// let bgra_vec = vec![30_u8,120,120,255];
    /// assert!(full_opacity!(bgra_vec[0], bgra_vec[1], bgra_vec[2], bgra_vec[3])); // asserts the macro
    /// assert!(full_opacity(bgra_vec[0], bgra_vec[1], bgra_vec[2], bgra_vec[3])); // asserts the function
    /// ```
    /// 
    /// By using the underscores in full_opacity ( _, _, _, a) the resulting function will inherit them, therefore ignoring the bytes in those positions.
    /// To see the macros built ( need to use nightly ) : cargo expand --bin *name of this .rs file* . which will result in :
    /// ```
    /// pub fn full_opacity(_: u8, _: u8, _: u8, a: u8) -> bool {
    ///     a >= 255
    /// }
    /// ```
    /// 
    /// Now we can pass the created function to a Vec method that takes a function as a statement to check on each 4 values chunks of the vector and change their values to the 4 provided ones when the statement was matched.
    /// In this case all the 3 values chunks are fully opaque (Alpha = 255), therefore they all will have their 4 values alterated into the provided ones (255,255,255,255)
    /// ```
    /// let mut bgra_3vec = vec![30_u8,120,120,255, 30_u8,120,120,255, 30_u8,120,120,255];
    /// bgra_3vec.color_matcher_and_new_color(full_opacity, 255,255,255,255);
    /// assert_eq!(bgra_3vec, vec![255,255,255,255, 255,255,255,255, 255,255,255,255]);
    /// 
    /// create_color_matcher_or_alterator!(max_blue_full_opacity (a,b,g,r), (b == 255, a == 255));
    /// let abgr_vec = vec![255,255,0,0];
    /// assert!(max_blue_full_opacity!(abgr_vec[0], abgr_vec[1], abgr_vec[2], abgr_vec[3])); // asserts the macro
    /// assert!(max_blue_full_opacity(abgr_vec[0], abgr_vec[1], abgr_vec[2], abgr_vec[3])); // asserts the function
    /// ```
    /// TO DO DOCUMENTATION, example to update
    /// # Examples
    /// 
    /// ```
    /// create_color_matcher_or_alterator!(max_blue_max_alpha (a,b,g,r) (), (b = 255_u8, a = 255_u8));
    /// let mut bb = 24_u8;
    /// let mut gg = 24_u8;
    /// let mut rr = 24_u8;
    /// let mut aa = 24_u8;
    /// max_blue_max_alpha!(bb,gg,rr,aa);
    /// println!("b{} g{} r{} a{}",bb,gg,rr,aa);
    /// let mut bbb = 24_u8;
    /// let mut ggg = 24_u8;
    /// let mut rrr = 24_u8;
    /// let mut aaa = 24_u8;
    /// max_blue_max_alpha(&mut bbb, &mut ggg, &mut rrr, &mut aaa);
    /// println!("b{} g{} r{} a{}",bbb,ggg,rrr,aaa);
    /// ```
    /// 
    /// Creates a bytes matcher for 4 values (e.g. BGRA ordered bytes) in a macro and in a function form (the macro one is faster, the function one can be used as parameter)
    /// 
    /// # Examples
    /// 
    /// ```
    /// create_color_matcher_or_alterator!(full_opacity (slice), (b,g,r,a) bool, (a >= 255));
    /// let bgra_vec = vec![30_u8,120,120,255];
    /// let bgra_vec_slice = &bgra_vec[0..=3];
    /// assert!(full_opacity!(bgra_vec_slice)); // asserts the macro
    /// assert!(full_opacity(bgra_vec_slice)); // asserts the function
    /// 
    /// /// Now we can pass the created function to a Vec method that takes a function as a statement to check on each 4 values chunks of the vector and change their values to the 4 provided ones when the statement was matched
    /// /// In this case all the 3 values chunks are fully opaque (Alpha = 255), therefore they all will have their 4 values alterated into the provided ones (255,255,255,255)
    /// let mut bgra_3vec = vec![30_u8,120,120,255, 30_u8,120,120,255, 30_u8,120,120,255];
    /// bgra_3vec.color_matcher_and_new_color(full_opacity, 255,255,255,255);
    /// assert_eq!(bgra_3vec, vec![255,255,255,255, 255,255,255,255, 255,255,255,255]);
    /// 
    /// create_color_matcher_or_alterator!(max_blue_full_opacity (slice), (a,b,g,r) bool, (b == 255, a == 255));
    /// let abgr_vec = vec![255_u8,255,0,0];
    /// let abgr_vec_slice = &abgr_vec[0..=3];
    /// assert!(max_blue_full_opacity!(abgr_vec_slice)); // asserts the macro
    /// assert!(max_blue_full_opacity(abgr_vec_slice)); // asserts the function
    /// ```
    /// TO DO DOCUMENTATION, example to update
    /// # Examples
    /// 
    /// ```
    /// create_color_matcher_or_alterator!(slice_max_blue_max_alpha (slice), (b,g,r,a) (), (a = 255));
    /// let mut bgra_vec = vec![30_u8,120,120,33, 40,13,44,22];
    /// slice_max_blue_max_alpha!(bgra_vec[0..=3]);
    /// assert_eq!(bgra_vec[0..=3], [30_u8,120,120,255]); // asserts the macro
    /// slice_max_blue_max_alpha(&mut bgra_vec[4..=7]);
    /// assert_eq!(bgra_vec[4..=7], [40,13,44,255]); // asserts the function
    /// ```
    /// TO DO DOCUMENTATION, example to update
    /// 
    /// # Examples
    /// 
    /// ```
    /// create_color_matcher_or_alterator!(not_fully_opaque (_,_,_,a) bool, (a < 255));
    /// create_vec_index_alterator!(low_red (slice), (b,g,r,a) (), (b = 0, g = 0, r = 100, a = 255));
    /// let mut bgra_vec = vec![30_u8,120,120,255, 40,13,44,22];
    /// low_red!(bgra_vec, 0);
    /// low_red(&mut bgra_vec, 4);
    /// assert_eq!(bgra_vec[0..=3], [0,0,100,255]); // asserts the alteration done by the macro
    /// assert_eq!(bgra_vec[4..=7], [0,0,100,255]); // asserts the alteration done by the function
    /// ```
    #[macro_export]
    macro_rules! create_color_matcher_or_alterator {
        ( $name:ident ($c1_name:tt : $c1_input_type:ty, $c2_name:tt : $c2_input_type:ty, $c3_name:tt : $c3_input_type:ty, $c4_name:tt : $c4_input_type:ty) $return_type:tt, ($($color_name:tt $op:tt $value:expr),*)) => {
            paste::item! {
                eager_macro_rules!{ $eager_4_values
                    #[macro_export]
                    macro_rules! [<$name _indexes>] {
                        ( [<$c1_name>] , $c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { $c1_val };
                        ( [<$c2_name>] , $c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { $c2_val };
                        ( [<$c3_name>] , $c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { $c3_val };
                        ( [<$c4_name>] , $c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { $c4_val };
                    }
                    /// cargo expand --bin performance_test // to see the macros built
                    #[macro_export]
                    macro_rules! $name {
                        //($c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { $( [<$name _indexes>]!($color_name, $c1_val, $c2_val, $c3_val, $c4_val) $op $value  ) &&* };
                        ($functions_borrowed_mut_and_type:ty, $c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => {
                            eager!{ { $( *[<$name _indexes>]!($color_name, $c1_val, $c2_val, $c3_val, $c4_val) $op $value and_operation_or_semicolon!($return_type) )* ending!($return_type) } }
                            // With eager! {} we can call macros when this one is not evaluated yet, like calling a macro that returns "+ 1", where starting + is forbidden in standard macros
                            // so we use it here to add ad an && or ; in case the return type is a bool or a (). the same for adding the ending a true or ;
                            // this gives us the ability to use the same line of code for both bool or () return types, because with the bools we get them all togeter with many && and a final true
                            // the final true won't change the result of all the previously chained bools. In the () case we just add ; at the end of the operation
                        };
                        ($c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => {
                            eager!{ { $( ([<$name _indexes>]!($color_name, $c1_val, $c2_val, $c3_val, $c4_val)) $op $value and_operation_or_semicolon!($return_type) )* ending!($return_type) } }
                        };
                        // find a way to set a macro case for when is a value and when is a &mut value and apply it to the calling macro, so that the fn_creator_new! can be done with just 1 case
                    }
                    //pub fn [<$name>]([<$c1_name>]:u8,[<$c2_name>]:u8,[<$c3_name>]:u8,[<$c4_name>]:u8) -> bool { $name!(__resolve_at_run!([<$c1_name>]),__resolve_at_run!([<$c2_name>]),__resolve_at_run!([<$c3_name>]),__resolve_at_run!([<$c4_name>])) }
                }
                /// cargo expand --bin performance_test // to see the functions built by the macro
                fn_creator_new!($name ($c1_name : $c1_input_type, $c2_name : $c2_input_type, $c3_name : $c3_input_type, $c4_name : $c4_input_type) $return_type);
            }
        };
        ( $name:ident ($color_slice_name:tt, $input_type:ty), ($c1_name:tt, $c2_name:tt, $c3_name:tt, $c4_name:tt) bool, ($($color_name:tt $op:tt $value:expr),*)) => {
            paste::item! {
                #[macro_export]
                macro_rules! [<$name _indexes>] {
                    ( [<$c1_name>] , $color_slice_val:expr) => { $color_slice_val[0] };
                    ( [<$c2_name>] , $color_slice_val:expr) => { $color_slice_val[1] };
                    ( [<$c3_name>] , $color_slice_val:expr) => { $color_slice_val[2] };
                    ( [<$c4_name>] , $color_slice_val:expr) => { $color_slice_val[3] };
                }
                #[macro_export]
                macro_rules! $name {
                    ($color_slice_val:expr) => {
                        { $( [<$name _indexes>]!($color_name, $color_slice_val) $op $value && )* true }
                    };
                }
                /// cargo expand --bin performance_test // to see the functions built by the macro
                fn_creator_new!($name $color_slice_name $input_type, bool);
            }
        };
        ( $name:ident ($color_slice_name:tt, $input_type:ty), ($c1_name:tt, $c2_name:tt, $c3_name:tt, $c4_name:tt) (), ($($color_name:tt $op:tt $value:expr),*)) => {
            paste::item! {
                #[macro_export]
                macro_rules! [<$name _indexes>] {
                    ( [<$c1_name>] , $color_slice_val:expr) => { $color_slice_val[0] };
                    ( [<$c2_name>] , $color_slice_val:expr) => { $color_slice_val[1] };
                    ( [<$c3_name>] , $color_slice_val:expr) => { $color_slice_val[2] };
                    ( [<$c4_name>] , $color_slice_val:expr) => { $color_slice_val[3] };
                }
                #[macro_export]
                macro_rules! $name {
                    ($color_slice_val:expr) => {
                        { $( [<$name _indexes>]!($color_name, $color_slice_val) $op $value ; )* }
                    };
                }
                /// cargo expand --bin performance_test // to see the functions built by the macro
                fn_creator_new!($name $color_slice_name $input_type, ());
            }
        };
        ( $name:ident ($color_slice_name:tt $input_type:ty, $color_index_name:tt), ($c1_name:tt, $c2_name:tt, $c3_name:tt, $c4_name:tt) $return_type:tt, ($($color_name:tt $op:tt $new_val:expr),*)) => {
            paste::item! {
                eager_macro_rules!{ $eager_vec_with_4_indexes
                    #[macro_export]
                    macro_rules! [<$name _indexes>] {
                        ( [<$c1_name>] , $color_slice_val:expr, $i:expr) => { $color_slice_val[$i] };
                        ( [<$c2_name>] , $color_slice_val:expr, $i:expr) => { $color_slice_val[$i+1] };
                        ( [<$c3_name>] , $color_slice_val:expr, $i:expr) => { $color_slice_val[$i+2] };
                        ( [<$c4_name>] , $color_slice_val:expr, $i:expr) => { $color_slice_val[$i+3] };
                    }
                    #[macro_export]
                    macro_rules! $name {
                        /*($vec:expr, $i:expr) => {
                            $vec[$i] = 0; $vec[$i+1] = 0; $vec[$i+2] = 100; $vec[$i+3] = 255;
                        };*/
                        ($color_slice_val:expr, $i:expr) => { { $( [<$name _indexes>]!($color_name, $color_slice_val, $i) $op $new_val  ) ;* } };
                    }
                }
                // cargo expand --bin performance_test // to see the macros built
                //pub fn [<$name>]([<$color_slice_name>]: &mut [u8], index: usize) -> $return_type { $name!(*[<$color_slice_name>], index) }
                fn_creator_new!($name $color_slice_name $input_type, $color_index_name, $return_type);
            }
        };
    }
}



#[macro_export]
macro_rules! bytes_matching {
    ($bytes_matcher:ident, $b:expr, $g:expr, $r:expr, $a:expr) => {
        $bytes_matcher!($b, $g, $r, $a)
    };
}
#[macro_export]
macro_rules! bytes_alteration {
    ($bytes_alterator:ident $vec:expr, $i:expr) => {
        $bytes_alterator!($vec, $i)
    };
}
#[macro_export]
macro_rules! color_match_and_alter {
    ($vec:expr, $if_part:ident => $true_case:ident) => {
        let mut i = 0;
        for _ in 0..$vec.len()/(PixelValues::get_units_per_pixel(&($vec[0])) as usize) {
            if bytes_matching!{$if_part, $vec[i], $vec[i+1], $vec[i+2], $vec[i+3]} {
                bytes_alteration!{$true_case $vec, i}
            }
            i += 4;
        }
    };
    ($vec:expr, $if_part:ident => $true_case:ident ; $optional_false_case:ident) => {
        let mut i = 0;
        for _ in 0..$vec.len()/(PixelValues::get_units_per_pixel(&($vec[0])) as usize) {
            if bytes_matching!{$if_part, $vec[i], $vec[i+1], $vec[i+2], $vec[i+3]} {
                bytes_alteration!{$true_case $vec, i}
            }
            else {
                bytes_alteration!{$optional_false_case $vec, i}
            }
            i += 4;
        }
    };
}
/// Creates a bytes matcher for 4 values (e.g. BGRA ordered bytes) in a macro and in a function form (the macro one is faster, the function one can be used as parameter)
/// 
/// # Examples
/// 
/// ```
/// create_slice_color_matcher_or_alterator!(full_opacity (slice), (b,g,r,a) bool, (a >= 255));
/// let bgra_vec = vec![30_u8,120,120,255];
/// let bgra_vec_slice = &bgra_vec[0..=3];
/// assert!(full_opacity!(bgra_vec_slice)); // asserts the macro
/// assert!(full_opacity(bgra_vec_slice)); // asserts the function
/// 
/// /// Now we can pass the created function to a Vec method that takes a function as a statement to check on each 4 values chunks of the vector and change their values to the 4 provided ones when the statement was matched
/// /// In this case all the 3 values chunks are fully opaque (Alpha = 255), therefore they all will have their 4 values alterated into the provided ones (255,255,255,255)
/// let mut bgra_3vec = vec![30_u8,120,120,255, 30_u8,120,120,255, 30_u8,120,120,255];
/// bgra_3vec.color_matcher_and_new_color(full_opacity, 255,255,255,255);
/// assert_eq!(bgra_3vec, vec![255,255,255,255, 255,255,255,255, 255,255,255,255]);
/// 
/// create_slice_color_matcher_or_alterator!(max_blue_full_opacity (slice), (a,b,g,r) bool, (b == 255, a == 255));
/// let abgr_vec = vec![255_u8,255,0,0];
/// let abgr_vec_slice = &abgr_vec[0..=3];
/// assert!(max_blue_full_opacity!(abgr_vec_slice)); // asserts the macro
/// assert!(max_blue_full_opacity(abgr_vec_slice)); // asserts the function
/// ```
/// TO DO DOCUMENTATION, example to update
/// # Examples
/// 
/// ```
/// create_slice_color_matcher_or_alterator!(slice_max_blue_max_alpha (slice), (b,g,r,a) (), (a = 255));
/// let mut bgra_vec = vec![30_u8,120,120,33, 40,13,44,22];
/// slice_max_blue_max_alpha!(bgra_vec[0..=3]);
/// assert_eq!(bgra_vec[0..=3], [30_u8,120,120,255]); // asserts the macro
/// slice_max_blue_max_alpha(&mut bgra_vec[4..=7]);
/// assert_eq!(bgra_vec[4..=7], [40,13,44,255]); // asserts the function
/// ```
#[macro_export]
macro_rules! create_slice_color_matcher_or_alterator {
    ( $name:ident ($color_slice_name:tt), ($c1_name:tt, $c2_name:tt, $c3_name:tt, $c4_name:tt) bool, ($($color_name:tt $op:tt $compar_val:expr),*)) => {
        paste::item! {
            #[macro_export]
            macro_rules! [<$name _indexes>] {
                ( [<$c1_name>] , $color_slice_val:expr) => { $color_slice_val[0] };
                ( [<$c2_name>] , $color_slice_val:expr) => { $color_slice_val[1] };
                ( [<$c3_name>] , $color_slice_val:expr) => { $color_slice_val[2] };
                ( [<$c4_name>] , $color_slice_val:expr) => { $color_slice_val[3] };
            }
            /// cargo expand --bin performance_test // to see the macros built
            #[macro_export]
            macro_rules! $name {
                ($color_slice_val:expr) => { $( [<$name _indexes>]!($color_name, $color_slice_val) $op $compar_val ) &&* };
            }
            pub fn [<$name>]([<$color_slice_name>]:&[u8]) -> bool { $name!([<$color_slice_name>]) }
        }
    };
    ( $name:ident ($color_slice_name:tt), ($c1_name:tt, $c2_name:tt, $c3_name:tt, $c4_name:tt) (), ($($color_name:tt $op:tt $new_val:expr),*)) => {
        paste::item! {
            #[macro_export]
            macro_rules! [<$name _indexes>] {
                ( [<$c1_name>] , $color_slice_val:expr) => { $color_slice_val[0] };
                ( [<$c2_name>] , $color_slice_val:expr) => { $color_slice_val[1] };
                ( [<$c3_name>] , $color_slice_val:expr) => { $color_slice_val[2] };
                ( [<$c4_name>] , $color_slice_val:expr) => { $color_slice_val[3] };
            }
            #[macro_export]
            macro_rules! $name {
                ($color_slice_val:expr) => { { $( [<$name _indexes>]!($color_name, $color_slice_val) $op $new_val ) ;* } };
            }
            // cargo expand --bin performance_test // to see the macros built
            pub fn [<$name>]([<$color_slice_name>]:&mut [u8]) { $name!(*[<$color_slice_name>]) }
        }
    };
}

/// TO DO DOCUMENTATION, example to update
/// 
/// # Examples
/// 
/// ```
/// create_color_matcher!(not_fully_opaque (_,_,_,a) bool, (a < 255));
/// create_vec_index_alterator!(low_red (slice), (b,g,r,a) (), (b = 0, g = 0, r = 100, a = 255));
/// let mut bgra_vec = vec![30_u8,120,120,255, 40,13,44,22];
/// low_red!(bgra_vec, 0);
/// low_red(&mut bgra_vec, 4);
/// assert_eq!(bgra_vec[0..=3], [0,0,100,255]); // asserts the alteration done by the macro
/// assert_eq!(bgra_vec[4..=7], [0,0,100,255]); // asserts the alteration done by the function
/// ```
#[macro_export]
macro_rules! create_vec_index_alterator {
    ( $name:ident ($color_slice_name:tt, $color_index_name:tt), ($c1_name:tt, $c2_name:tt, $c3_name:tt, $c4_name:tt) $return_type:tt, ($($color_name:tt $op:tt $new_val:expr),*)) => {
        paste::item! {
            #[macro_export]
            macro_rules! [<$name _indexes>] {
                ( [<$c1_name>] , $color_slice_val:expr, $i:expr) => { $color_slice_val[$i] };
                ( [<$c2_name>] , $color_slice_val:expr, $i:expr) => { $color_slice_val[$i+1] };
                ( [<$c3_name>] , $color_slice_val:expr, $i:expr) => { $color_slice_val[$i+2] };
                ( [<$c4_name>] , $color_slice_val:expr, $i:expr) => { $color_slice_val[$i+3] };
            }
            #[macro_export]
            macro_rules! $name {
                /*($vec:expr, $i:expr) => {
                    $vec[$i] = 0; $vec[$i+1] = 0; $vec[$i+2] = 100; $vec[$i+3] = 255;
                };*/
                ($color_slice_val:expr, $i:expr) => { { $( [<$name _indexes>]!($color_name, $color_slice_val, $i) $op $new_val  ) ;* } };
            }
            // cargo expand --bin performance_test // to see the macros built ( need to use nightly nad to remove dep: from cargo.toml because it bothers the compiler when on nigthly)
            pub fn [<$name>]([<$color_slice_name>]: &mut [u8], index: usize) -> $return_type { $name!(*[<$color_slice_name>], index) }
        }
    };
}

/// Creates a bytes matcher for 4 values (e.g. BGRA ordered bytes) in a macro and in a function form (the macro one is faster, the function one can be used as parameter)
/// 
/// # Examples
/// 
/// ```
/// create_color_matcher_or_alterator_4_values!(full_opacity ( _, _, _, a), (a >= 255));
/// let bgra_vec = vec![30_u8,120,120,255];
/// assert!(full_opacity!(bgra_vec[0], bgra_vec[1], bgra_vec[2], bgra_vec[3])); // asserts the macro
/// assert!(full_opacity(bgra_vec[0], bgra_vec[1], bgra_vec[2], bgra_vec[3])); // asserts the function
/// ```
/// 
/// By using the underscores in full_opacity ( _, _, _, a) the resulting function will inherit them, therefore ignoring the bytes in those positions.
/// To see the macros built ( need to use nightly ) : cargo expand --bin *name of this .rs file* . which will result in :
/// ```
/// pub fn full_opacity(_: u8, _: u8, _: u8, a: u8) -> bool {
///     a >= 255
/// }
/// ```
/// 
/// Now we can pass the created function to a Vec method that takes a function as a statement to check on each 4 values chunks of the vector and change their values to the 4 provided ones when the statement was matched.
/// In this case all the 3 values chunks are fully opaque (Alpha = 255), therefore they all will have their 4 values alterated into the provided ones (255,255,255,255)
/// ```
/// let mut bgra_3vec = vec![30_u8,120,120,255, 30_u8,120,120,255, 30_u8,120,120,255];
/// bgra_3vec.color_matcher_and_new_color(full_opacity, 255,255,255,255);
/// assert_eq!(bgra_3vec, vec![255,255,255,255, 255,255,255,255, 255,255,255,255]);
/// 
/// create_color_matcher_or_alterator_4_values!(max_blue_full_opacity (a,b,g,r), (b == 255, a == 255));
/// let abgr_vec = vec![255,255,0,0];
/// assert!(max_blue_full_opacity!(abgr_vec[0], abgr_vec[1], abgr_vec[2], abgr_vec[3])); // asserts the macro
/// assert!(max_blue_full_opacity(abgr_vec[0], abgr_vec[1], abgr_vec[2], abgr_vec[3])); // asserts the function
/// ```
/// TO DO DOCUMENTATION, example to update
/// # Examples
/// 
/// ```
/// create_color_matcher_or_alterator_4_values!(max_blue_max_alpha (a,b,g,r) (), (b = 255_u8, a = 255_u8));
/// let mut bb = 24_u8;
/// let mut gg = 24_u8;
/// let mut rr = 24_u8;
/// let mut aa = 24_u8;
/// max_blue_max_alpha!(bb,gg,rr,aa);
/// println!("b{} g{} r{} a{}",bb,gg,rr,aa);
/// let mut bbb = 24_u8;
/// let mut ggg = 24_u8;
/// let mut rrr = 24_u8;
/// let mut aaa = 24_u8;
/// max_blue_max_alpha(&mut bbb, &mut ggg, &mut rrr, &mut aaa);
/// println!("b{} g{} r{} a{}",bbb,ggg,rrr,aaa);
/// ```
#[macro_export]
macro_rules! create_color_matcher_or_alterator_4_values {
    ( $name:ident ($c1_name:tt, $c2_name:tt, $c3_name:tt, $c4_name:tt) bool, ($($color_name:tt $op:tt $compar_val:expr),*)) => {
        paste::item! {
            #[macro_export]
            macro_rules! [<$name _indexes>] {
                ( [<$c1_name>] , $c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { $c1_val };
                ( [<$c2_name>] , $c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { $c2_val };
                ( [<$c3_name>] , $c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { $c3_val };
                ( [<$c4_name>] , $c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { $c4_val };
            }
            /// cargo expand --bin performance_test // to see the macros built
            #[macro_export]
            macro_rules! $name {
                ($c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { $( [<$name _indexes>]!($color_name, $c1_val, $c2_val, $c3_val, $c4_val) $op $compar_val  ) &&* };
                //($c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { {{$( [<$name _indexes>]!($color_name, $c1_val, $c2_val, $c3_val, $c4_val) $op $compar_val  ) &&*  && true } } };
            }
            //pub fn [<$name>]([<$c1_name>]:u8,[<$c2_name>]:u8,[<$c3_name>]:u8,[<$c4_name>]:u8) -> bool { println!("c4 name is {}, value is {}", stringify!($c4_name), [<$c1_name>]); $name!(__resolve_at_run!([<$c1_name>]),__resolve_at_run!([<$c2_name>]),__resolve_at_run!([<$c3_name>]),__resolve_at_run!([<$c4_name>])) }
            pub fn [<$name>]([<$c1_name>]:u8,[<$c2_name>]:u8,[<$c3_name>]:u8,[<$c4_name>]:u8) -> bool { $name!(__resolve_at_run!([<$c1_name>]),__resolve_at_run!([<$c2_name>]),__resolve_at_run!([<$c3_name>]),__resolve_at_run!([<$c4_name>])) }
        }
    };
    ( $name:ident ($c1_name:tt, $c2_name:tt, $c3_name:tt, $c4_name:tt) (), ($($color_name:tt $op:tt $compar_val:expr),*)) => {
        paste::item! {
            #[macro_export]
            macro_rules! [<$name _indexes>] {
                ( [<$c1_name>] , $c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { $c1_val };
                ( [<$c2_name>] , $c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { $c2_val };
                ( [<$c3_name>] , $c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { $c3_val };
                ( [<$c4_name>] , $c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { $c4_val };
            }
            /// cargo expand --bin performance_test // to see the macros built
            #[macro_export]
            macro_rules! $name {
                ($c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { { $( [<$name _indexes>]!($color_name, $c1_val, $c2_val, $c3_val, $c4_val) $op $compar_val  ) ;* } };
            }
            // cargo expand --bin performance_test // to see the macros built ( need to use nightly nad to remove dep: from cargo.toml because it bothers the compiler when on nigthly)
            pub fn [<$name>]([<$c1_name>]: &mut u8,[<$c2_name>]: &mut u8,[<$c3_name>]: &mut u8,[<$c4_name>]: &mut u8) { $name!(*__resolve_at_run!([<$c1_name>]), *__resolve_at_run!([<$c2_name>]), *__resolve_at_run!([<$c3_name>]), *__resolve_at_run!([<$c4_name>])) }
        }
    };
}

// using [<$item_name>] (usable inside paste::item!{} ) permits to overcome the "cannot find value `b` in this scope" / "not found in this scope" error, it just places the text resulting from [<$name _optional_added_text>] so it wont be tried to be elaborated right away
// [<$cx_name>] pastes just the name of the macro parameter, not its value
// example :
// print_item_resolve_later_demonstration!(test_x (b,g,_,a), (a < 33, g > 4));
// println!("fn (0,5,40,22) : {}", test_x(0,5,40,22));
// println!("macro (2,4,100,66) : {}", test_x!(2,4,100,66));
#[macro_export]
macro_rules! print_item_resolve_later_demonstration {
    ( $name:ident ($c1_name:tt, $c2_name:tt, $c3_name:tt, $c4_name:tt), ($($color_name:tt $op:tt $compar_val:expr),*)) => {
        paste::item! {
            #[macro_export]
            macro_rules! [<$name _indexes>] {
                ( [<$c1_name>] , $c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { $c1_val };
                ( [<$c2_name>] , $c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { $c2_val };
                ( [<$c3_name>] , $c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { $c3_val };
                ( [<$c4_name>] , $c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { $c4_val };
            }
            /// macro version
            #[macro_export]
            macro_rules! $name {
                ($c1_val:expr, $c2_val:expr, $c3_val:expr, $c4_val:expr) => { { $( [<$name _indexes>]!($color_name, $c1_val, $c2_val, $c3_val, $c4_val) $op $compar_val ) &&* } };
            }
            /// fn version
            pub fn [<$name>]([<$c1_name>]:u8,[<$c2_name>]:u8,[<$c3_name>]:u8,[<$c4_name>]:u8) -> bool {
                // use stringify! here because in this case r is _, which would complain : "in expressions, `_` can only be used on the left-hand side of an assignment" / "`_` not allowed here"
                // it will transform the name into a string without trying to evaluate its value in this context
                println!("parameters names are : {}, {}, {}, {}", stringify!($c1_name), stringify!($c2_name), stringify!($c3_name), stringify!($c4_name));
                // using $cx_name would result in error : "cannot find value `b` in this scope" / "not found in this scope" so use [<$cx_name>] so that it wont be resolver into b right away, b will just be pasted there
                // given that r is _ we can do nothing with it, other than assure what it is (_) with stringify!($c3_name)
                println!("parameters values are : {}, {}, {}, {}", [<$c1_name>], [<$c2_name>], "cannot be evaluated since there is none in _", [<$c4_name>]);
                // calling __resolve_it_at_run!([<$c3_name>]) (which simply returns the provided value) will not only postpone the evaluation of $c3_name but even its value evaluation
                $name!(__resolve_it_at_run!([<$c1_name>]),__resolve_it_at_run!([<$c2_name>]),__resolve_it_at_run!([<$c3_name>]),__resolve_it_at_run!([<$c4_name>]))
            }
        }
    };
}
#[macro_export]
macro_rules! __resolve_it_at_run {
    ( $name:tt ) => { $name };
}