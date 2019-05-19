use std::marker;
use std::any::TypeId;
use std::any::Any;

use crate::reflect::runtime_type_box::RuntimeTypeBox;
use crate::reflect::runtime_types::RuntimeType;
use crate::reflect::EnumDescriptor;
use crate::reflect::MessageDescriptor;
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectValueRef;


/// Dynamic version of `RuntimeType`
pub trait RuntimeTypeDynamic: Send + Sync + 'static {
    fn to_box(&self) -> RuntimeTypeBox;

    fn value_to_ref<'a>(&self, value: &'a ProtobufValue) -> ReflectValueRef<'a>;

    fn default_value_ref(&self) -> ReflectValueRef;

    fn enum_descriptor(&self) -> &'static EnumDescriptor;

    fn message_descriptor(&self) -> &'static MessageDescriptor;
}

pub(crate) struct RuntimeTypeDynamicImpl<T: RuntimeType>(pub marker::PhantomData<T>);

impl<T: RuntimeType> RuntimeTypeDynamic for RuntimeTypeDynamicImpl<T> {
    fn to_box(&self) -> RuntimeTypeBox {
        T::runtime_type_box()
    }

    fn value_to_ref<'a>(&self, value: &'a ProtobufValue) -> ReflectValueRef<'a> {
        if Any::type_id(value) == TypeId::of::<T::Value>() {
            unsafe {
                T::as_ref(&*(value as *const dyn ProtobufValue as *const T::Value))
            }
        } else {
            panic!("wrong type")
        }
    }

    fn default_value_ref(&self) -> ReflectValueRef {
        T::default_value_ref()
    }

    fn enum_descriptor(&self) -> &'static EnumDescriptor {
        T::enum_descriptor()
    }
    fn message_descriptor(&self) -> &'static MessageDescriptor {
        T::message_descriptor()
    }
}
