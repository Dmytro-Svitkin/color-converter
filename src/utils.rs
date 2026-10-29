#![allow(unused)]

#[cfg(feature="f32_n0u8ix-lut")]
const F32_N0U8IX:[f32;255]={// Might remove that later, but unlikely. // 255 is a shorthand for explicit `u8::MAX as usize`.
    let mut array:[f32;255]=[0.0;255];
    let mut index_counter:usize=0;
    while index_counter<255{
        index_counter+=1;
        array[index_counter-1]=index_counter as f32/256.0;
    }
    array
};

#[cfg(feature="f32_n0u8ix-lut")]
pub(crate)const unsafe fn u8_f32_unchecked(value:u8)->f32{
    F32_N0U8IX[value as usize-1]
}

#[cfg(not(feature="f32_n0u8ix-lut"))]
pub(crate)const unsafe fn u8_f32_unchecked(value:u8)->f32{
    value as f32/256.0
}

pub(crate)const fn u8_f32(value:u8)->Option<f32>{
    if value==0{return Some(0.0)}
    Some(unsafe{u8_f32_unchecked(value)})
}

pub(crate)const fn u8_f32_soft(value:u8)->f32{
    if value==0{return 0.0}
    unsafe{u8_f32_unchecked(value)}
}

pub(crate)const unsafe fn f32_u8_unchecked(value:f32)->u8{
    (value*256.0)as u8
}

pub(crate)const fn f32_u8(value:f32)->Option<u8>{
    if is_valid_f32(value)||value>1.0{return None}
    else if value==1.0{return Some(u8::MAX)}
    Some(unsafe{f32_u8_unchecked(value)})
}

pub(crate)const fn f32_u8_soft(value:f32)->u8{
    if value.is_nan()||value<=0.0{return 0}
    else if value>=1.0{return u8::MAX}
    unsafe{f32_u8_unchecked(value)}
}

pub(crate)const unsafe fn u8_u16_unchecked(value:u8)->u16{
    (value as u16)<<8
}

pub(crate)const fn u8_u16(value:u8)->Option<u16>{
    Some(unsafe{u8_u16_unchecked(value)})
}

pub(crate)const fn u8_u16_soft(value:u8)->u16{
    unsafe{u8_u16_unchecked(value)}
}

pub(crate)const unsafe fn u16_u8_unchecked(value:u16)->u8{
    (value>>8)as u8
}

pub(crate)const fn u16_u8(value:u16)->Option<u8>{
    Some(unsafe{u16_u8_unchecked(value)})
}

pub(crate)const fn u16_u8_soft(value:u16)->u8{
    unsafe{u16_u8_unchecked(value)}
}

pub(crate)const unsafe fn u16_f32_unchecked(value:u16)->f32{
    (value as f32)/u16::MAX as f32
}

pub(crate)const fn u16_f32(value:u16)->Option<f32>{
    Some(unsafe{u16_f32_unchecked(value)})
}

pub(crate)const fn u16_f32_soft(value:u16)->f32{
    unsafe{u16_f32_unchecked(value)}
}

pub(crate)const unsafe fn f32_u16_unchecked(value:f32)->u16{
    (value*u16::MAX as f32)as u16
}

pub(crate)const fn f32_u16(value:f32)->Option<u16>{
    if is_valid_f32(value)||value>1.0{return None}
    else if value==1.0{return Some(u16::MAX)}
    Some(unsafe{f32_u16_unchecked(value)})
}

pub(crate)const fn f32_u16_soft(value:f32)->u16{
    if value.is_nan()||value<=0.0{return 0}
    else if value>=1.0{return u16::MAX}
    unsafe{f32_u16_unchecked(value)}
}

#[inline(always)]
const fn is_valid_f32(value:f32)->bool{
    !(value.is_nan()||value.is_infinite()||value.is_subnormal())
}