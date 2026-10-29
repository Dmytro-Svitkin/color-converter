use crate::Color;
use crate::utils::*;

/*
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Rgb<T>{
    red:T,
    blue:T,
    green:T
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Rgba<T>{
    red:T,
    blue:T,
    green:T,
    alpha:T
}
*/

impl Color<u8>{
    pub const fn from_rgba(red:u8,green:u8,blue:u8,alpha:u8)->Self{
        Self::from_u8_rgba(red,green,blue,alpha)
    }

    pub const fn from_rgb(red:u8,green:u8,blue:u8)->Self{
        Self::from_u8_rgb(red,green,blue)
    }

    pub const fn as_raw_rgba(&self)->[u8;4]{
        self.as_raw_u8_rgba()
    }

    pub const fn as_raw_rgb(&self)->[u8;3]{
        self.as_raw_u8_rgb()
    }

    pub const fn from_u8_rgba(red:u8,green:u8,blue:u8,alpha:u8)->Self{
        Self{
            red,
            green,
            blue,
            alpha
        }
    }

    pub const fn from_u8_rgb(red:u8,green:u8,blue:u8)->Self{
        Self::from_u8_rgba(red,green,blue,u8::MAX)
    }

    pub const fn from_u16_rgba(red:u16,green:u16,blue:u16,alpha:u16)->Self{
        Self{
            red:  u16_u8_soft(red),
            green:u16_u8_soft(green),
            blue: u16_u8_soft(blue),
            alpha:u16_u8_soft(alpha)
        }
    }

    pub const fn from_u16_rgb(red:u16,green:u16,blue:u16)->Self{
        Self::from_u16_rgba(red,green,blue,u16::MAX)
    }

    pub const fn from_f32_rgba(red:f32,green:f32,blue:f32,alpha:f32)->Self{
        Self{
            red:  f32_u8_soft(red),
            green:f32_u8_soft(green),
            blue: f32_u8_soft(blue),
            alpha:f32_u8_soft(alpha)
        }
    }

    pub const fn from_f32_rgb(red:f32,green:f32,blue:f32)->Self{
        Self::from_f32_rgba(red,green,blue,1.0)
    }

    pub const fn as_raw_u8_rgba(&self)->[u8;4]{
        [
            self.red,
            self.green,
            self.blue,
            self.alpha
        ]
    }

    pub const fn as_raw_u8_rgb(&self)->[u8;3]{
        [
            self.red,
            self.green,
            self.blue
        ]
    }

    pub const fn as_raw_u16_rgba(&self)->[u16;4]{
        [
            u8_u16_soft(self.red),
            u8_u16_soft(self.green),
            u8_u16_soft(self.blue),
            u8_u16_soft(self.alpha)
        ]
    }

    pub const fn as_raw_u16_rgb(&self)->[u16;3]{
        [
            u8_u16_soft(self.red),
            u8_u16_soft(self.green),
            u8_u16_soft(self.blue)
        ]
    }

    pub const fn as_raw_f32_rgba(&self)->[f32;4]{
        [
            u8_f32_soft(self.red),
            u8_f32_soft(self.green),
            u8_f32_soft(self.blue),
            u8_f32_soft(self.alpha)
        ]
    }

    pub const fn as_raw_f32_rgb(&self)->[f32;3]{
        [
            u8_f32_soft(self.red),
            u8_f32_soft(self.green),
            u8_f32_soft(self.blue)
        ]
    }
}

impl Color<u16>{
    pub const fn from_rgba(red:u16,green:u16,blue:u16,alpha:u16)->Self{
        Self::from_u16_rgba(red,green,blue,alpha)
    }

    pub const fn from_rgb(red:u16,green:u16,blue:u16)->Self{
        Self::from_u16_rgb(red,green,blue)
    }

    pub const fn as_raw_rgba(&self)->[u16;4]{
        self.as_raw_u16_rgba()
    }

    pub const fn as_raw_rgb(&self)->[u16;3]{
        self.as_raw_u16_rgb()
    }

    pub const fn from_u8_rgba(red:u8,green:u8,blue:u8,alpha:u8)->Self{
        Self{
            red:  u8_u16_soft(red),
            green:u8_u16_soft(green),
            blue: u8_u16_soft(blue),
            alpha:u8_u16_soft(alpha)
        }
    }

    pub const fn from_u8_rgb(red:u8,green:u8,blue:u8)->Self{
        Self::from_u8_rgba(red,green,blue,u8::MAX)
    }

    pub const fn from_u16_rgba(red:u16,green:u16,blue:u16,alpha:u16)->Self{
        Self{
            red:  u16_u8_soft(red),
            green:u16_u8_soft(green),
            blue: u16_u8_soft(blue),
            alpha:u16_u8_soft(alpha)
        }
    }

    pub const fn from_u16_rgb(red:u16,green:u16,blue:u16)->Self{
        Self::from_u16_rgba(red,green,blue,u16::MAX)
    }

    pub const fn from_f32_rgba(red:f32,green:f32,blue:f32,alpha:f32)->Self{
        Self{
            red:  f32_u8_soft(red),
            green:f32_u8_soft(green),
            blue: f32_u8_soft(blue),
            alpha:f32_u8_soft(alpha)
        }
    }

    pub const fn from_f32_rgb(red:f32,green:f32,blue:f32)->Self{
        Self::from_f32_rgba(red,green,blue,1.0)
    }

    pub const fn as_raw_u8_rgba(&self)->[u8;4]{
        [
            self.red,
            self.green,
            self.blue,
            self.alpha
        ]
    }

    pub const fn as_raw_u8_rgb(&self)->[u8;3]{
        [
            self.red,
            self.green,
            self.blue
        ]
    }

    pub const fn as_raw_u16_rgba(&self)->[u16;4]{
        [
            u8_u16_soft(self.red),
            u8_u16_soft(self.green),
            u8_u16_soft(self.blue),
            u8_u16_soft(self.alpha)
        ]
    }

    pub const fn as_raw_u16_rgb(&self)->[u16;3]{
        [
            u8_u16_soft(self.red),
            u8_u16_soft(self.green),
            u8_u16_soft(self.blue)
        ]
    }

    pub const fn as_raw_f32_rgba(&self)->[f32;4]{
        [
            u8_f32_soft(self.red),
            u8_f32_soft(self.green),
            u8_f32_soft(self.blue),
            u8_f32_soft(self.alpha)
        ]
    }

    pub const fn as_raw_f32_rgb(&self)->[f32;3]{
        [
            u8_f32_soft(self.red),
            u8_f32_soft(self.green),
            u8_f32_soft(self.blue)
        ]
    }
}