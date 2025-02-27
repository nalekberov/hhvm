// @generated by Thrift for thrift/compiler/test/fixtures/rust-noserver/src/module.thrift
// This file is probably not the place you want to edit!

//! Thrift type definitions for `module`.

#![allow(clippy::redundant_closure)]


#[derive(Clone, PartialEq)]
pub struct MyStruct {
    pub MyIntField: ::std::primitive::i64,
    pub MyStringField: ::std::string::String,
    pub MyDataField: crate::types::MyDataItem,
    pub myEnum: crate::types::MyEnum,
    // This field forces `..Default::default()` when instantiating this
    // struct, to make code future-proof against new fields added later to
    // the definition in Thrift. If you don't want this, add the annotation
    // `(rust.exhaustive)` to the Thrift struct to eliminate this field.
    #[doc(hidden)]
    pub _dot_dot_Default_default: self::dot_dot::OtherFields,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MyDataItem {
    // This field forces `..Default::default()` when instantiating this
    // struct, to make code future-proof against new fields added later to
    // the definition in Thrift. If you don't want this, add the annotation
    // `(rust.exhaustive)` to the Thrift struct to eliminate this field.
    #[doc(hidden)]
    pub _dot_dot_Default_default: self::dot_dot::OtherFields,
}

#[derive(Clone, PartialEq, Debug)]
pub enum MyUnion {
    myEnum(crate::types::MyEnum),
    myStruct(crate::types::MyStruct),
    myDataItem(crate::types::MyDataItem),
    UnknownField(::std::primitive::i32),
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MyEnum(pub ::std::primitive::i32);

impl MyEnum {
    pub const MyValue1: Self = MyEnum(0i32);
    pub const MyValue2: Self = MyEnum(1i32);
}

impl ::fbthrift::ThriftEnum for MyEnum {
    fn enumerate() -> &'static [(Self, &'static str)] {
        &[
            (Self::MyValue1, "MyValue1"),
            (Self::MyValue2, "MyValue2"),
        ]
    }

    fn variants() -> &'static [&'static str] {
        &[
            "MyValue1",
            "MyValue2",
        ]
    }

    fn variant_values() -> &'static [Self] {
        &[
            Self::MyValue1,
            Self::MyValue2,
        ]
    }
}

impl ::std::default::Default for MyEnum {
    fn default() -> Self {
        Self(::fbthrift::__UNKNOWN_ID)
    }
}

impl<'a> ::std::convert::From<&'a MyEnum> for ::std::primitive::i32 {
    #[inline]
    fn from(x: &'a MyEnum) -> Self {
        x.0
    }
}

impl ::std::convert::From<MyEnum> for ::std::primitive::i32 {
    #[inline]
    fn from(x: MyEnum) -> Self {
        x.0
    }
}

impl ::std::convert::From<::std::primitive::i32> for MyEnum {
    #[inline]
    fn from(x: ::std::primitive::i32) -> Self {
        Self(x)
    }
}

impl ::std::fmt::Display for MyEnum {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        static VARIANTS_BY_NUMBER: &[(&::std::primitive::str, ::std::primitive::i32)] = &[
            ("MyValue1", 0),
            ("MyValue2", 1),
        ];
        ::fbthrift::help::enum_display(VARIANTS_BY_NUMBER, fmt, self.0)
    }
}

impl ::std::fmt::Debug for MyEnum {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(fmt, "MyEnum::{}", self)
    }
}

impl ::std::str::FromStr for MyEnum {
    type Err = ::anyhow::Error;

    fn from_str(string: &::std::primitive::str) -> ::std::result::Result<Self, Self::Err> {
        static VARIANTS_BY_NAME: &[(&::std::primitive::str, ::std::primitive::i32)] = &[
            ("MyValue1", 0),
            ("MyValue2", 1),
        ];
        ::fbthrift::help::enum_from_str(VARIANTS_BY_NAME, string, "MyEnum").map(Self)
    }
}

impl ::fbthrift::GetTType for MyEnum {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::I32;
}

impl<P> ::fbthrift::Serialize<P> for MyEnum
where
    P: ::fbthrift::ProtocolWriter,
{
    #[inline]
    fn write(&self, p: &mut P) {
        p.write_i32(self.into())
    }
}

impl<P> ::fbthrift::Deserialize<P> for MyEnum
where
    P: ::fbthrift::ProtocolReader,
{
    #[inline]
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        ::std::result::Result::Ok(Self::from(p.read_i32()?))
    }
}

#[allow(clippy::derivable_impls)]
impl ::std::default::Default for self::MyStruct {
    fn default() -> Self {
        Self {
            MyIntField: ::std::default::Default::default(),
            MyStringField: ::std::default::Default::default(),
            MyDataField: ::std::default::Default::default(),
            myEnum: ::std::default::Default::default(),
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        }
    }
}

impl ::std::fmt::Debug for self::MyStruct {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter
            .debug_struct("MyStruct")
            .field("MyIntField", &self.MyIntField)
            .field("MyStringField", &self.MyStringField)
            .field("MyDataField", &self.MyDataField)
            .field("myEnum", &self.myEnum)
            .finish()
    }
}

unsafe impl ::std::marker::Send for self::MyStruct {}
unsafe impl ::std::marker::Sync for self::MyStruct {}
impl ::std::marker::Unpin for self::MyStruct {}

impl ::fbthrift::GetTType for self::MyStruct {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
}

impl<P> ::fbthrift::Serialize<P> for self::MyStruct
where
    P: ::fbthrift::ProtocolWriter,
{
    fn write(&self, p: &mut P) {
        p.write_struct_begin("MyStruct");
        p.write_field_begin("MyIntField", ::fbthrift::TType::I64, 1);
        ::fbthrift::Serialize::write(&self.MyIntField, p);
        p.write_field_end();
        p.write_field_begin("MyStringField", ::fbthrift::TType::String, 2);
        ::fbthrift::Serialize::write(&self.MyStringField, p);
        p.write_field_end();
        p.write_field_begin("MyDataField", ::fbthrift::TType::Struct, 3);
        ::fbthrift::Serialize::write(&self.MyDataField, p);
        p.write_field_end();
        p.write_field_begin("myEnum", ::fbthrift::TType::I32, 4);
        ::fbthrift::Serialize::write(&self.myEnum, p);
        p.write_field_end();
        p.write_field_stop();
        p.write_struct_end();
    }
}

impl<P> ::fbthrift::Deserialize<P> for self::MyStruct
where
    P: ::fbthrift::ProtocolReader,
{
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        static FIELDS: &[::fbthrift::Field] = &[
            ::fbthrift::Field::new("MyDataField", ::fbthrift::TType::Struct, 3),
            ::fbthrift::Field::new("MyIntField", ::fbthrift::TType::I64, 1),
            ::fbthrift::Field::new("MyStringField", ::fbthrift::TType::String, 2),
            ::fbthrift::Field::new("myEnum", ::fbthrift::TType::I32, 4),
        ];
        let mut field_MyIntField = ::std::option::Option::None;
        let mut field_MyStringField = ::std::option::Option::None;
        let mut field_MyDataField = ::std::option::Option::None;
        let mut field_myEnum = ::std::option::Option::None;
        let _ = p.read_struct_begin(|_| ())?;
        loop {
            let (_, fty, fid) = p.read_field_begin(|_| (), FIELDS)?;
            match (fty, fid as ::std::primitive::i32) {
                (::fbthrift::TType::Stop, _) => break,
                (::fbthrift::TType::I64, 1) => field_MyIntField = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::String, 2) => field_MyStringField = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::Struct, 3) => field_MyDataField = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (::fbthrift::TType::I32, 4) => field_myEnum = ::std::option::Option::Some(::fbthrift::Deserialize::read(p)?),
                (fty, _) => p.skip(fty)?,
            }
            p.read_field_end()?;
        }
        p.read_struct_end()?;
        ::std::result::Result::Ok(Self {
            MyIntField: field_MyIntField.unwrap_or_default(),
            MyStringField: field_MyStringField.unwrap_or_default(),
            MyDataField: field_MyDataField.unwrap_or_default(),
            myEnum: field_myEnum.unwrap_or_default(),
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        })
    }
}


impl ::fbthrift::metadata::ThriftAnnotations for MyStruct {
    fn get_structured_annotation<T: Sized + 'static>() -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        None
    }

    fn get_field_structured_annotation<T: Sized + 'static>(field_id: i16) -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        match field_id {
            1 => {
            },
            2 => {
            },
            3 => {
            },
            4 => {
            },
            _ => {}
        }

        None
    }
}


#[allow(clippy::derivable_impls)]
impl ::std::default::Default for self::MyDataItem {
    fn default() -> Self {
        Self {
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        }
    }
}

impl ::std::fmt::Debug for self::MyDataItem {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter
            .debug_struct("MyDataItem")
            .finish()
    }
}

unsafe impl ::std::marker::Send for self::MyDataItem {}
unsafe impl ::std::marker::Sync for self::MyDataItem {}
impl ::std::marker::Unpin for self::MyDataItem {}

impl ::fbthrift::GetTType for self::MyDataItem {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
}

impl<P> ::fbthrift::Serialize<P> for self::MyDataItem
where
    P: ::fbthrift::ProtocolWriter,
{
    fn write(&self, p: &mut P) {
        p.write_struct_begin("MyDataItem");
        p.write_field_stop();
        p.write_struct_end();
    }
}

impl<P> ::fbthrift::Deserialize<P> for self::MyDataItem
where
    P: ::fbthrift::ProtocolReader,
{
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        static FIELDS: &[::fbthrift::Field] = &[
        ];
        let _ = p.read_struct_begin(|_| ())?;
        loop {
            let (_, fty, fid) = p.read_field_begin(|_| (), FIELDS)?;
            match (fty, fid as ::std::primitive::i32) {
                (::fbthrift::TType::Stop, _) => break,
                (fty, _) => p.skip(fty)?,
            }
            p.read_field_end()?;
        }
        p.read_struct_end()?;
        ::std::result::Result::Ok(Self {
            _dot_dot_Default_default: self::dot_dot::OtherFields(()),
        })
    }
}


impl ::fbthrift::metadata::ThriftAnnotations for MyDataItem {
    fn get_structured_annotation<T: Sized + 'static>() -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        None
    }

    fn get_field_structured_annotation<T: Sized + 'static>(field_id: i16) -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        match field_id {
            _ => {}
        }

        None
    }
}



impl ::std::default::Default for MyUnion {
    fn default() -> Self {
        Self::UnknownField(-1)
    }
}

impl ::fbthrift::GetTType for MyUnion {
    const TTYPE: ::fbthrift::TType = ::fbthrift::TType::Struct;
}

impl<P> ::fbthrift::Serialize<P> for MyUnion
where
    P: ::fbthrift::ProtocolWriter,
{
    fn write(&self, p: &mut P) {
        p.write_struct_begin("MyUnion");
        match self {
            Self::myEnum(inner) => {
                p.write_field_begin("myEnum", ::fbthrift::TType::I32, 1);
                ::fbthrift::Serialize::write(inner, p);
                p.write_field_end();
            }
            Self::myStruct(inner) => {
                p.write_field_begin("myStruct", ::fbthrift::TType::Struct, 2);
                ::fbthrift::Serialize::write(inner, p);
                p.write_field_end();
            }
            Self::myDataItem(inner) => {
                p.write_field_begin("myDataItem", ::fbthrift::TType::Struct, 3);
                ::fbthrift::Serialize::write(inner, p);
                p.write_field_end();
            }
            Self::UnknownField(_) => {}
        }
        p.write_field_stop();
        p.write_struct_end();
    }
}

impl<P> ::fbthrift::Deserialize<P> for MyUnion
where
    P: ::fbthrift::ProtocolReader,
{
    fn read(p: &mut P) -> ::anyhow::Result<Self> {
        static FIELDS: &[::fbthrift::Field] = &[
            ::fbthrift::Field::new("myDataItem", ::fbthrift::TType::Struct, 3),
            ::fbthrift::Field::new("myEnum", ::fbthrift::TType::I32, 1),
            ::fbthrift::Field::new("myStruct", ::fbthrift::TType::Struct, 2),
        ];
        let _ = p.read_struct_begin(|_| ())?;
        let mut once = false;
        let mut alt = ::std::option::Option::None;
        loop {
            let (_, fty, fid) = p.read_field_begin(|_| (), FIELDS)?;
            match (fty, fid as ::std::primitive::i32, once) {
                (::fbthrift::TType::Stop, _, _) => break,
                (::fbthrift::TType::I32, 1, false) => {
                    once = true;
                    alt = ::std::option::Option::Some(Self::myEnum(::fbthrift::Deserialize::read(p)?));
                }
                (::fbthrift::TType::Struct, 2, false) => {
                    once = true;
                    alt = ::std::option::Option::Some(Self::myStruct(::fbthrift::Deserialize::read(p)?));
                }
                (::fbthrift::TType::Struct, 3, false) => {
                    once = true;
                    alt = ::std::option::Option::Some(Self::myDataItem(::fbthrift::Deserialize::read(p)?));
                }
                (fty, _, false) => p.skip(fty)?,
                (badty, badid, true) => return ::std::result::Result::Err(::std::convert::From::from(::fbthrift::ApplicationException::new(
                    ::fbthrift::ApplicationExceptionErrorCode::ProtocolError,
                    format!(
                        "unwanted extra union {} field ty {:?} id {}",
                        "MyUnion",
                        badty,
                        badid,
                    ),
                ))),
            }
            p.read_field_end()?;
        }
        p.read_struct_end()?;
        ::std::result::Result::Ok(alt.unwrap_or_default())
    }
}


impl ::fbthrift::metadata::ThriftAnnotations for MyUnion {
    fn get_structured_annotation<T: Sized + 'static>() -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        None
    }

    fn get_field_structured_annotation<T: Sized + 'static>(field_id: i16) -> ::std::option::Option<T> {
        #[allow(unused_variables)]
        let type_id = ::std::any::TypeId::of::<T>();

        match field_id {
            1 => {
            },
            2 => {
            },
            3 => {
            },
            _ => {}
        }

        None
    }
}

mod dot_dot {
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OtherFields(pub(crate) ());

    #[allow(dead_code)] // if serde isn't being used
    pub(super) fn default_for_serde_deserialize() -> OtherFields {
        OtherFields(())
    }
}
