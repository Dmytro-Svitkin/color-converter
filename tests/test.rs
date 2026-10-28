use color_converter::*;

#[test]
fn test(){
    let b:Color<u8>=Color::from_u8_rgb(12,123,44);
    println!("{:?}",b)
}