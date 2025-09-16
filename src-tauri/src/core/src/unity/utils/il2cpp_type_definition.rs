use std::ops::Range;

use crate::unity::{
    generated::il2cpp_2022333f1::root::Il2CppTypeDefinition,
    utils::{complex_type::ComplexType, il2cpp::Il2Cpp},
};

impl Il2CppTypeDefinition {
    #[inline(always)]
    pub fn field_indices(&self) -> Range<usize> {
        let start = self.fieldStart as usize;
        start..(start + self.field_count as usize)
    }

    pub fn has_field<'a>(&'a self, il2cpp: &'a Il2Cpp<'a>, name: &str, ty_name: &str) -> bool {
        self.field_indices().any(|idx| {
            let field = &il2cpp.metadata.fields[idx];

            let field_name = il2cpp.metadata.get_string_by_index(field.nameIndex);
            if field_name != name {
                return false;
            }

            Self::field_type_matches(il2cpp, field.typeIndex as usize, ty_name)
        })
    }

    fn field_type_matches(il2cpp: &Il2Cpp, type_index: usize, ty_name: &str) -> bool {
        match il2cpp.types[type_index].get_complex_type(il2cpp) {
            Ok(ComplexType::Simple { ref name, .. }) => name == ty_name,
            Ok(ComplexType::Generic { ref base, .. }) => base.to_string() == ty_name,
            _ => false,
        }
    }

    #[inline(always)]
    fn has_flag(&self, bit: u8) -> bool {
        (self.bitfield & (1 << bit)) != 0
    }

    pub fn is_value_type(&self) -> bool {
        self.has_flag(0)
    }
    pub fn is_enum_type(&self) -> bool {
        self.has_flag(1)
    }
    pub fn has_finalize(&self) -> bool {
        self.has_flag(2)
    }
    pub fn has_cctor(&self) -> bool {
        self.has_flag(3)
    }
    pub fn is_blittable(&self) -> bool {
        self.has_flag(4)
    }
    pub fn is_import(&self) -> bool {
        self.has_flag(5)
    }

    pub fn packing_size(&self) -> u8 {
        ((self.bitfield >> 6) & 0xF) as u8
    }
}
