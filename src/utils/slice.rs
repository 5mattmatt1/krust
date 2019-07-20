///
pub unsafe fn convert_slice<'a, T, D>(src: &'a [T]) -> &'a [D]
{
    return core::slice::from_raw_parts(&src[0] as *const _ as *const D,
                                        src.len() * (core::mem::size_of::<T>() / 
                                                     core::mem::size_of::<D>()));
}

/// 
pub unsafe fn convert_slice_mut<'a, T, D>(src: &'a mut [T]) -> &'a [D]
{
    return core::slice::from_raw_parts_mut(&mut src[0] as *mut _ as *mut D,
                                            src.len() * (core::mem::size_of::<T>() / 
                                                        core::mem::size_of::<D>()));
}