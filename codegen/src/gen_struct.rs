use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    gen_defs::GenImplStruct,
    gen_utils::{align_up, doc_attr, kebab_to_rust, kebab_to_type},
    parse_spec::{AttrProp, AttrType, ByteOrder, CBitFieldType, DefType, Spec},
};

pub fn struct_type(spec: &Spec, name: &str) -> Ident {
    if spec.experimental.struct_prefix.is_some_and(|e| e == false) {
        format_ident!("{}", kebab_to_type(name))
    } else {
        format_ident!("Push{}", kebab_to_type(name))
    }
}

pub fn gen_struct_len(spec: &Spec, r#struct: &str) -> (usize, usize) {
    match r#struct {
        "builtin-bitfield32" => return (8, 4),
        _ => {}
    }

    let DefType::Struct { members, .. } = &spec.find_def(r#struct).def else {
        unreachable!("{:?}", r#struct);
    };

    let mut len = 0;
    let mut alignment = 1;
    let mut sub_bit_len = 0;
    for member in members {
        if let AttrType::CBitField { bits, .. } = member.r#type {
            sub_bit_len += bits;
            continue;
        }

        if sub_bit_len != 0 {
            len += (sub_bit_len + 7) / 8;
            sub_bit_len = 0;
        }

        let (member_len, member_alignment) = match &member.r#type {
            AttrType::U8 => (1, 1),
            AttrType::U16 => (2, 2),
            AttrType::U32 => (4, 4),
            AttrType::U64 => (8, 8),
            AttrType::S8 => (1, 1),
            AttrType::S16 => (2, 2),
            AttrType::S32 => (4, 4),
            AttrType::S64 => (8, 8),
            AttrType::Pad { len: Some(len) } => (*len, 1),
            AttrType::Binary {
                r#struct: Some(r#struct),
                ..
            } => gen_struct_len(spec, r#struct),
            AttrType::Binary {
                r#struct: None,
                len: Some(len),
            } => (*len, 1),
            r#type => unreachable!("{:?}", r#type),
        };

        len = align_up(len, member_alignment) + member_len;
        alignment = alignment.max(member_alignment);
    }

    if sub_bit_len != 0 {
        let bytes = (sub_bit_len + 7) / 8;
        len += bytes;
    }

    (align_up(len, alignment), alignment)
}

pub fn gen_struct_field(
    spec: &Spec,
    members: &mut TokenStream,
    debug: &mut TokenStream,
    m: &mut GenImplStruct,
    attr: &AttrProp,
) {
    let value_name = format_ident!("value");
    let getter_prefix = match attr.name.as_str() {
        "type" | "len" | "if" => "get_",
        _ => "",
    };
    let getter_name = format_ident!("{getter_prefix}{}", kebab_to_rust(&attr.name));
    let setter_name = format_ident!("set_{}", kebab_to_rust(&attr.name));

    if let AttrType::Pad { len: Some(len) } = &attr.r#type {
        m.off += len;
        return;
    }

    let encode_ord = match attr.byte_order {
        ByteOrder::Host => "ne",
        ByteOrder::Little => "le",
        ByteOrder::Big => "be",
    };
    let encode = format_ident!("to_{encode_ord}_bytes");

    let name = kebab_to_rust(&attr.name);
    // Taken from ./gen_debug_impl.rs
    let debug_format = match attr.display_hint.as_ref().map(|s| s.as_str()) {
        _ if attr.r#enum.is_some() => {
            let next = attr;
            let val_name = quote!(self.#getter_name());

            let r#enum = next.r#enum.as_ref().unwrap();
            let enum_def = spec.find_def(r#enum);
            let enum_type = format_ident!("{}", kebab_to_type(r#enum));

            let as_flags = next.enum_as_flags.is_some_and(|val| val);
            let def_flags = matches!(enum_def.def, DefType::Flags { .. });
            let (formatter, from_val);
            if def_flags {
                formatter = quote!(FormatFlags);
                from_val = quote!(#enum_type::from_value);
            } else if as_flags {
                formatter = quote!(FormatFlags);
                from_val = quote!(|val| #enum_type::from_value(val.trailing_zeros().into()));
            } else {
                formatter = quote!(FormatEnum);
                from_val = quote!(#enum_type::from_value);
            }

            match &next.r#type {
                AttrType::Binary {
                    r#struct: Some(s), ..
                } if s == "builtin-bitfield32" => {
                    quote! { &#formatter(#val_name.value.into(), #from_val) }
                }
                _ => quote! { &#formatter(#val_name.into(), #from_val) },
            }
        }
        Some("hex") if attr.is_num() => quote!(&FormatHex(self.#getter_name().#encode())),
        Some("hex") => quote!(&FormatHex(self.#getter_name())),
        Some("string") => quote!(&FormatBinStr(self.#getter_name())),
        _ => quote!(&self.#getter_name()),
    };
    debug.extend(quote!(.field(#name, #debug_format)));

    let mut docs = TokenStream::new();
    doc_attr(attr, |doc| docs.extend(quote!(#[doc = #doc])));

    if let AttrType::CBitField { bits, sub_type } = &attr.r#type {
        let (rust_type, alignment) = match sub_type {
            CBitFieldType::U8 => ("u8", 1),
            CBitFieldType::U16 => ("u16", 2),
            CBitFieldType::U32 => ("u32", 4),
        };
        let len = alignment;
        let max_bits = len * 8;

        if m.last_bit_type
            .as_ref()
            .is_none_or(|(s, _, _)| s != sub_type)
            || m.bit_off + bits > max_bits
        {
            m.bit_off = 0;
            m.alignment = m.alignment.max(alignment);
            m.off = align_up(m.off, alignment);
            m.last_bit_type = Some((sub_type.clone(), m.off, format_ident!("unused")));
            m.off += len;
        }

        let (_, off, _) = m.last_bit_type.as_ref().unwrap();

        let byte_off = off + m.bit_off / 8;
        let bit_off = m.bit_off % 8;
        m.bit_off += bits;
        let rust_type = format_ident!("{}", rust_type);

        members.extend(quote! {
            #docs
            pub fn #getter_name(&self) -> #rust_type {
                get_bits(&self.buf[..], #byte_off, #bit_off, #bits) as #rust_type
            }
            #docs
            pub fn #setter_name(&mut self, #value_name: #rust_type) {
                set_bits(&mut self.buf[..], #byte_off, #bit_off, #bits, #value_name as u32);
            }
        });

        return;
    }

    if m.bit_off != 0 {
        m.bit_off = 0;
        m.last_bit_type = None;
    }

    let (rust_type, len) = match &attr.r#type {
        AttrType::U8 => ("u8", 1),
        AttrType::U16 => ("u16", 2),
        AttrType::U32 => ("u32", 4),
        AttrType::U64 => ("u64", 8),
        AttrType::S8 => ("i8", 1),
        AttrType::S16 => ("i16", 2),
        AttrType::S32 => ("i32", 4),
        AttrType::S64 => ("i64", 8),
        // AttrType::Binary { len: Some(len), .. } if attr.is_ip() => {
        //     let first = m.off;
        //     let last = m.off + len;
        //     m.off += len;
        //
        //     members.extend(quote! {
        //         #docs
        //         pub fn #getter_name(&self) -> Option<std::net::IpAddr> {
        //             parse_ip(&self.buf[#first..#last])
        //         }
        //         #docs
        //         pub fn #setter_name(&mut self, #value_name: Option<std::net::IpAddr>) {
        //             if let Some(#value_name) = #value_name {
        //                 encode_ip(&mut std::io::Cursor::new(&mut self.buf[#first..#last]), #value_name);
        //             } else {
        //                 self.buf[#first..#last].fill(0)
        //             }
        //         }
        //     });
        //
        //     return;
        // }
        AttrType::Binary {
            r#struct: Some(r#struct),
            ..
        } => {
            let rust_type = struct_type(spec, r#struct);

            let (len, alignment) = gen_struct_len(spec, r#struct);

            m.off = align_up(m.off, alignment);
            m.alignment = m.alignment.max(alignment);

            let first = m.off;
            let last = m.off + len;
            m.off += len;

            members.extend(quote! {
                #docs
                pub fn #getter_name(&self) -> #rust_type {
                    #rust_type::new_from_slice(&self.buf[#first..#last]).unwrap()
                }
                #docs
                pub fn #setter_name(&mut self, #value_name: #rust_type) {
                    self.buf[#first..#last].copy_from_slice(&#value_name.as_slice())
                }
            });

            return;
        }
        AttrType::Binary {
            r#struct: None,
            len: Some(len),
        } => {
            let rust_type = quote!([u8; #len]);

            let first = m.off;
            let last = m.off + len;
            m.off += len;

            members.extend(quote! {
                #docs
                pub fn #getter_name(&self) -> #rust_type {
                    self.buf[#first..#last].try_into().unwrap()
                }
                #docs
                pub fn #setter_name(&mut self, #value_name: #rust_type) {
                    self.buf[#first..#last].copy_from_slice(&#value_name)
                }
            });

            return;
        }
        other => todo!("{:?}", other),
    };

    let rust_type = format_ident!("{}", rust_type);

    m.alignment = m.alignment.max(len);
    m.off = align_up(m.off, len);

    let first = m.off;
    let last = m.off + len;
    m.off += len;

    let ord = match attr.byte_order {
        ByteOrder::Host => "",
        ByteOrder::Little => "_le",
        ByteOrder::Big => "_be",
    };
    let parse = format_ident!("parse{ord}_{rust_type}");

    members.extend(quote! {
        #docs
        pub fn #getter_name(&self) -> #rust_type {
            #parse(&self.buf[#first..#last]).unwrap()
        }
        #docs
        pub fn #setter_name(&mut self, #value_name: #rust_type) {
            self.buf[#first..#last].copy_from_slice(&#value_name.#encode())
        }
    });
}

// Binary structures are aligned according to C conventions
// Link: https://www.gnu.org/software/c-intro-and-ref/manual/html_node/Structure-Layout.html
pub fn gen_struct(tokens: &mut TokenStream, spec: &Spec, name: &str, members: &[AttrProp]) {
    let type_name = struct_type(spec, name);
    let name_str = kebab_to_type(name);

    let mut m = GenImplStruct {
        off: 0,
        bit_off: 0,
        last_bit_type: None,
        alignment: 1,
        lifetime_needed: false,
        type_name: type_name.clone(),
    };

    let mut inner = TokenStream::new();
    let mut debug = TokenStream::new();
    for attr in members {
        gen_struct_field(spec, &mut inner, &mut debug, &mut m, attr);
    }

    if m.bit_off != 0 {
        m.bit_off = 0;
        m.last_bit_type = None;
    }

    let fmt_name = format_ident!("fmt");

    let len = align_up(m.off, m.alignment);
    tokens.extend(quote! {
        #[derive(Clone)]
        pub struct #type_name {
            pub(crate) buf: [u8; #len],
        }

        #[doc = "Create zero-initialized struct"]
        impl Default for #type_name {
            fn default() -> Self {
                Self { buf: [0u8; #len] }
            }
        }

        impl #type_name {
            #[doc = "Create zero-initialized struct"]
            pub fn new() -> Self {
                Default::default()
            }
            #[doc = "Copy from contents from other slice"]
            pub fn new_from_slice(other: &[u8]) -> Option<Self> {
                if other.len() != Self::len() {
                    return None;
                }
                let mut buf = [0u8; Self::len()];
                buf.clone_from_slice(other);
                Some(Self { buf })
            }
            #[doc = "Copy from contents from another slice, padding with zeros or truncating when needed"]
            pub fn new_from_zeroed(other: &[u8]) -> Self {
                let mut buf = [0u8; Self::len()];
                let len = buf.len().min(other.len());
                buf[..len].clone_from_slice(&other[..len]);
                Self { buf }
            }
            pub fn as_slice(&self) -> &[u8] {
                &self.buf
            }
            pub fn as_mut_slice(&mut self) -> &mut [u8] {
                &mut self.buf
            }
            pub const fn len() -> usize {
                #len
            }
            // pub const fn alignment() -> usize {
            //     #alignment
            // }
            #inner
        }

        impl std::fmt::Debug for #type_name {
            fn fmt(&self, #fmt_name: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #fmt_name
                    .debug_struct(#name_str)
                    #debug
                    .finish()
            }
        }
    });
}
