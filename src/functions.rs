use core::any::TypeId;

#[inline]
pub(crate) fn is_number<T: 'static>() -> bool {
    let type_id = TypeId::of::<T>();

    type_id == TypeId::of::<u8>()
        || type_id == TypeId::of::<u16>()
        || type_id == TypeId::of::<u32>()
        || type_id == TypeId::of::<u64>()
        || type_id == TypeId::of::<u128>()
        || type_id == TypeId::of::<usize>()
        || type_id == TypeId::of::<i8>()
        || type_id == TypeId::of::<i16>()
        || type_id == TypeId::of::<i32>()
        || type_id == TypeId::of::<i64>()
        || type_id == TypeId::of::<i128>()
        || type_id == TypeId::of::<isize>()
        || type_id == TypeId::of::<f32>()
        || type_id == TypeId::of::<f64>()
}
