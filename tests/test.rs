use color_converter::Color;

#[test]
fn test(){
    let b:Color<u8>=Color::<u8>::from_u8_rgb(12,123,44);
    println!("{:?}",b.as_raw_f32_rgb())
}