// This file is generated by rust-protobuf 2.2.2. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Vault {
    // message fields
    pub identities: ::protobuf::RepeatedField<::std::string::String>,
    pub index: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl Vault {
    pub fn new() -> Vault {
        ::std::default::Default::default()
    }

    // repeated string identities = 1;

    pub fn clear_identities(&mut self) {
        self.identities.clear();
    }

    // Param is passed by value, moved
    pub fn set_identities(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.identities = v;
    }

    // Mutable pointer to the field.
    pub fn mut_identities(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.identities
    }

    // Take field
    pub fn take_identities(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.identities, ::protobuf::RepeatedField::new())
    }

    pub fn get_identities(&self) -> &[::std::string::String] {
        &self.identities
    }

    // repeated .Vault.IndexEntry index = 2;

    pub fn clear_index(&mut self) {
        self.index.clear();
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.index = v;
    }

    // Mutable pointer to the field.
    pub fn mut_index(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.index
    }

    // Take field
    pub fn take_index(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.index, ::std::collections::HashMap::new())
    }

    pub fn get_index(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.index
    }
}

impl ::protobuf::Message for Vault {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.identities)?;
                },
                2 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.index)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.identities {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(2, &self.index);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.identities {
            os.write_string(1, &v)?;
        };
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(2, &self.index, os)?;
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Vault {
        Vault::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "identities",
                    |m: &Vault| { &m.identities },
                    |m: &mut Vault| { &mut m.identities },
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                    "index",
                    |m: &Vault| { &m.index },
                    |m: &mut Vault| { &mut m.index },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Vault>(
                    "Vault",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Vault {
        static mut instance: ::protobuf::lazy::Lazy<Vault> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Vault,
        };
        unsafe {
            instance.get(Vault::new)
        }
    }
}

impl ::protobuf::Clear for Vault {
    fn clear(&mut self) {
        self.clear_identities();
        self.clear_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Vault {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Vault {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Entry {
    // message fields
    pub attributes: ::std::collections::HashMap<::std::string::String, Attribute>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl Entry {
    pub fn new() -> Entry {
        ::std::default::Default::default()
    }

    // repeated .Entry.AttributesEntry attributes = 1;

    pub fn clear_attributes(&mut self) {
        self.attributes.clear();
    }

    // Param is passed by value, moved
    pub fn set_attributes(&mut self, v: ::std::collections::HashMap<::std::string::String, Attribute>) {
        self.attributes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attributes(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, Attribute> {
        &mut self.attributes
    }

    // Take field
    pub fn take_attributes(&mut self) -> ::std::collections::HashMap<::std::string::String, Attribute> {
        ::std::mem::replace(&mut self.attributes, ::std::collections::HashMap::new())
    }

    pub fn get_attributes(&self) -> &::std::collections::HashMap<::std::string::String, Attribute> {
        &self.attributes
    }
}

impl ::protobuf::Message for Entry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Attribute>>(wire_type, is, &mut self.attributes)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Attribute>>(1, &self.attributes);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Attribute>>(1, &self.attributes, os)?;
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Entry {
        Entry::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<Attribute>>(
                    "attributes",
                    |m: &Entry| { &m.attributes },
                    |m: &mut Entry| { &mut m.attributes },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Entry>(
                    "Entry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Entry {
        static mut instance: ::protobuf::lazy::Lazy<Entry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Entry,
        };
        unsafe {
            instance.get(Entry::new)
        }
    }
}

impl ::protobuf::Clear for Entry {
    fn clear(&mut self) {
        self.clear_attributes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Entry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Attribute {
    // message fields
    pub value: ::std::string::String,
    pub bytes_value: ::std::vec::Vec<u8>,
    pub confidential: bool,
    pub file: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl Attribute {
    pub fn new() -> Attribute {
        ::std::default::Default::default()
    }

    // string value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.value, ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    // bytes bytes_value = 2;

    pub fn clear_bytes_value(&mut self) {
        self.bytes_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_bytes_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.bytes_value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bytes_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.bytes_value
    }

    // Take field
    pub fn take_bytes_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.bytes_value, ::std::vec::Vec::new())
    }

    pub fn get_bytes_value(&self) -> &[u8] {
        &self.bytes_value
    }

    // bool confidential = 100;

    pub fn clear_confidential(&mut self) {
        self.confidential = false;
    }

    // Param is passed by value, moved
    pub fn set_confidential(&mut self, v: bool) {
        self.confidential = v;
    }

    pub fn get_confidential(&self) -> bool {
        self.confidential
    }

    // bool file = 101;

    pub fn clear_file(&mut self) {
        self.file = false;
    }

    // Param is passed by value, moved
    pub fn set_file(&mut self, v: bool) {
        self.file = v;
    }

    pub fn get_file(&self) -> bool {
        self.file
    }
}

impl ::protobuf::Message for Attribute {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.value)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.bytes_value)?;
                },
                100 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.confidential = tmp;
                },
                101 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.file = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.value);
        }
        if !self.bytes_value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.bytes_value);
        }
        if self.confidential != false {
            my_size += 3;
        }
        if self.file != false {
            my_size += 3;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.value.is_empty() {
            os.write_string(1, &self.value)?;
        }
        if !self.bytes_value.is_empty() {
            os.write_bytes(2, &self.bytes_value)?;
        }
        if self.confidential != false {
            os.write_bool(100, self.confidential)?;
        }
        if self.file != false {
            os.write_bool(101, self.file)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Attribute {
        Attribute::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    |m: &Attribute| { &m.value },
                    |m: &mut Attribute| { &mut m.value },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bytes_value",
                    |m: &Attribute| { &m.bytes_value },
                    |m: &mut Attribute| { &mut m.bytes_value },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "confidential",
                    |m: &Attribute| { &m.confidential },
                    |m: &mut Attribute| { &mut m.confidential },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "file",
                    |m: &Attribute| { &m.file },
                    |m: &mut Attribute| { &mut m.file },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Attribute>(
                    "Attribute",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Attribute {
        static mut instance: ::protobuf::lazy::Lazy<Attribute> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Attribute,
        };
        unsafe {
            instance.get(Attribute::new)
        }
    }
}

impl ::protobuf::Clear for Attribute {
    fn clear(&mut self) {
        self.clear_value();
        self.clear_bytes_value();
        self.clear_confidential();
        self.clear_file();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Attribute {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Attribute {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x08pb.proto\"\x8a\x01\n\x05Vault\x12\x1e\n\nidentities\x18\x01\x20\
    \x03(\tR\nidentities\x12'\n\x05index\x18\x02\x20\x03(\x0b2\x11.Vault.Ind\
    exEntryR\x05index\x1a8\n\nIndexEntry\x12\x10\n\x03key\x18\x01\x20\x01(\t\
    R\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\tR\x05value:\x028\x01\"\x8a\
    \x01\n\x05Entry\x126\n\nattributes\x18\x01\x20\x03(\x0b2\x16.Entry.Attri\
    butesEntryR\nattributes\x1aI\n\x0fAttributesEntry\x12\x10\n\x03key\x18\
    \x01\x20\x01(\tR\x03key\x12\x20\n\x05value\x18\x02\x20\x01(\x0b2\n.Attri\
    buteR\x05value:\x028\x01\"z\n\tAttribute\x12\x14\n\x05value\x18\x01\x20\
    \x01(\tR\x05value\x12\x1f\n\x0bbytes_value\x18\x02\x20\x01(\x0cR\nbytesV\
    alue\x12\"\n\x0cconfidential\x18d\x20\x01(\x08R\x0cconfidential\x12\x12\
    \n\x04file\x18e\x20\x01(\x08R\x04fileb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
