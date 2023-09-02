#![feature(const_precise_live_drops)]

pub enum COption<T: Sized>{
    Some(T),
    None
}

impl<T: Sized> COption<T>{

    pub const fn into_std(self) -> Option<T>{
        match self{
            COption::Some(t) => Option::Some(t),
            COption::None => Option::None,
        }
    }
}
