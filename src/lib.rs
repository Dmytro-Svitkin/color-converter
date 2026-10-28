#![no_std]

const F32_N0U8IX:[f32;255]={// Might remove that later, but unlikely.
    let mut array:[f32;255]=[0.0;255];
    let mut index_counter:usize=0;
    while index_counter<255{
        index_counter+=1;
        array[index_counter-1]=index_counter as f32/256.0;
    }
    array
};

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Color<T>{
    pub(crate)red:T,
    pub(crate)green:T,
    pub(crate)blue:T,
    pub(crate)alpha:T
}

impl Color<u8>{
    pub const fn from_u8_rgba(red:u8,green:u8,blue:u8,alpha:u8)->Self{
        Self{red,blue,green,alpha}
    }

    pub const fn from_u8_rgb(red:u8,green:u8,blue:u8)->Self{
        Self::from_u8_rgba(red,green,blue,255)
    }

    pub const fn transparent_black()->Self{
        Self::from_u8_rgba(0,0,0,0)
    }
    
    pub const fn transparent_white()->Self{
        Self::from_u8_rgba(255,255,255,0)
    }

    pub const fn black()->Self{
        Self::from_u8_rgb(0,0,0)
    }

    pub const fn white()->Self{
        Self::from_u8_rgb(255,255,255)
    }

    pub const fn red()->Self{
        Self::from_u8_rgb(255,0,0)
    }

    pub const fn green()->Self{
        Self::from_u8_rgb(0,255,0)
    }

    pub const fn blue()->Self{
        Self::from_u8_rgb(0,0,255)
    }
    
    pub const fn cyan()->Self{
        Self::from_u8_rgb(0,255,255)
    }
    
    pub const fn yellow()->Self{
        Self::from_u8_rgb(255,255,0)
    }
    
    pub const fn magenta()->Self{
        Self::from_u8_rgb(255,0,255)
    }
}

const unsafe fn u8_f32_unchecked(value:u8)->f32{
    F32_N0U8IX[value as usize-1]
}

const fn u8_f32(value:u8)->Option<f32>{
    if value==0{return Some(0.0)}
    Some(unsafe{u8_f32_unchecked(value)})
}

const fn u8_f32_soft(value:u8)->f32{
    if value==0{return 0.0}
    unsafe{u8_f32_unchecked(value)}
}

const unsafe fn f32_u8_unchecked(value:f32)->u8{
    (value*256.0)as u8
}

const fn f32_u8(value:f32)->Option<u8>{
    if value.is_nan()||value.is_infinite(){return None}
    Some(unsafe{f32_u8_unchecked(value)})
}

const fn f32_u8_soft(value:f32)->u8{
    if value.is_nan()||value.is_sign_negative(){return 0}
    unsafe{f32_u8_unchecked(value)}
}

const unsafe fn u8_u16_unchecked(value:u8)->u16{
    (value as u16)<<8
}

const fn u8_u16(value:u8)->Option<u16>{
    Some(unsafe{u8_u16_unchecked(value)})
}

const fn u8_u16_soft(value:u8)->u16{
    unsafe{u8_u16_unchecked(value)}
}

const unsafe fn u16_u8_unchecked(value:u16)->u8{
    (value>>8)as u8
}

const fn u16_u8(value:u16)->Option<u8>{
    Some(unsafe{u16_u8_unchecked(value)})
}

const fn u16_u8_soft(value:u16)->u8{
    unsafe{u16_u8_unchecked(value)}
}