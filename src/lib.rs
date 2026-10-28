#![no_std]

mod utils;

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