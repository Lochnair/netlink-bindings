use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    gen_defs::GenImplStruct,
    gen_struct::{gen_struct_len, struct_type},
    gen_utils::{align_up, doc_attr, kebab_to_rust, kebab_to_type},
    parse_spec::{AttrProp, AttrSet, AttrType, ByteOrder, CBitFieldType, DefType, Spec},
    WARNING,
};

#[allow(unused)]
pub fn gen_push_align(spec: &Spec, set: &AttrSet, attr: AttrProp, alignment: usize) -> TokenStream {
    if alignment <= 4 {
        return quote!();
    }

    let mut pad_id = None;

    let superset = set
        .subset_of
        .as_ref()
        .map(|s| spec.find_attr(s))
        .unwrap_or(set);

    let mut id: u16 = 0;
    for a in &superset.attributes {
        id += 1;
        if let Some(val) = &a.value {
            id = *val;
        }
        if matches!(a.r#type, AttrType::Pad { .. }) {
            pad_id = Some(id);
            break;
        }
    }

    if pad_id.is_none() {
        println!(
            "{WARNING} Attrset {} has {}-byte-aligned attr {} but no pading attribute",
            set.name, alignment, attr.name
        );
    }

    let pad_id: u16 = pad_id.unwrap_or(0x3ff);
    quote! {
        push_pad(self.as_rec_mut(), #pad_id, #alignment);
    }
}

// Binary structures are aligned according to C conventions
// Link: https://www.gnu.org/software/c-intro-and-ref/manual/html_node/Structure-Layout.html
pub fn gen_cstruct(tokens: &mut TokenStream, spec: &Spec, name: &str, members: &[AttrProp]) {
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

    let mut cinner = TokenStream::new();
    let mut cimpl = TokenStream::new();
    let mut debug = TokenStream::new();
    let mut default_debug = 0;
    for attr in members {
        gen_cstruct_field(
            spec,
            &mut cinner,
            &mut cimpl,
            &mut debug,
            &mut default_debug,
            &mut m,
            attr,
        );
    }

    if m.bit_off != 0 {
        m.bit_off = 0;
        m.last_bit_type = None;
    }

    let fmt_name = format_ident!("fmt");

    let alignment = m.alignment;
    insert_padding(spec, &mut cinner, &mut m, 0, alignment);
    let len = m.off;

    if default_debug == members.len() {
        tokens.extend(quote! {
            #[derive(Debug)]
        });
    }

    tokens.extend(quote! {
        // Generally, we can't rely on struct being aligned >4 bytes (default netlink alignment)
        // Kernel usually does consistently insert padding, but other programs may not.
        // This has the downside of being unable to take &u64 directly (compiler will complain).
        #[repr(C, packed(4))]
        pub struct #type_name {
            #cinner
        }

        impl Clone for #type_name {
            fn clone(&self) -> Self {
                Self::new_from_array(*self.as_array())
            }
        }

        #[doc = "Create zero-initialized struct"]
        impl Default for #type_name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl #type_name {
            #[doc = "Create zero-initialized struct"]
            pub fn new() -> Self {
                Self::new_from_array([0u8; Self::len()])
            }
            #[doc = "Copy from contents from slice"]
            pub fn new_from_slice(other: &[u8]) -> Option<Self> {
                if other.len() != Self::len() {
                    return None;
                }
                let mut buf = [0u8; Self::len()];
                buf.clone_from_slice(other);
                Some(Self::new_from_array(buf))
            }
            #[doc = "Copy from contents from another slice, padding with zeros or truncating when needed"]
            pub fn new_from_zeroed(other: &[u8]) -> Self {
                let mut buf = [0u8; Self::len()];
                let len = buf.len().min(other.len());
                buf[..len].clone_from_slice(&other[..len]);
                Self::new_from_array(buf)
            }
            pub fn new_from_array(buf: [u8; #len]) -> Self {
                unsafe { std::mem::transmute(buf) }
            }
            pub fn as_slice(&self) -> &[u8] {
                unsafe {
                    let ptr: *const u8 = std::mem::transmute(self as *const Self);
                    std::slice::from_raw_parts(ptr, Self::len())
                }
            }
            pub fn from_slice(buf: &[u8]) -> &Self {
                assert!(buf.len() >= Self::len());
                assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
                unsafe { std::mem::transmute(buf.as_ptr()) }
            }
            pub fn as_array(&self) -> &[u8; #len] {
                unsafe { std::mem::transmute(self) }
            }
            pub fn from_array(buf: &[u8; #len]) -> &Self {
                assert!(buf.as_ptr() as usize % std::mem::align_of::<Self>() == 0);
                unsafe { std::mem::transmute(buf) }
            }
            pub fn into_array(self) -> [u8; #len] {
                unsafe { std::mem::transmute(self) }
            }
            // pub fn as_mut_slice(&mut self) -> &mut [u8] {
            //     &mut self.buf
            // }
            pub const fn len() -> usize {
                const _: () = assert!(std::mem::size_of::<#type_name>() == #len);
                #len
            }
            // pub const fn alignment() -> usize {
            //     #alignment
            // }
            #cimpl
        }
    });

    if default_debug != members.len() {
        tokens.extend(quote! {
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
}

pub fn gen_cstruct_field(
    spec: &Spec,
    members: &mut TokenStream,
    cimpl: &mut TokenStream,
    debug: &mut TokenStream,
    default_debug: &mut usize,
    m: &mut GenImplStruct,
    attr: &AttrProp,
) {
    let value_name = format_ident!("value");
    let prefix = match attr.name.as_str() {
        "type" | "if" => "r#",
        _ => "",
    };
    let getter_prefix = match attr.name.as_str() {
        "type" | "len" | "if" => "get_",
        _ => "",
    };

    let ord = match attr.byte_order {
        ByteOrder::Host => "",
        ByteOrder::Little => "_le",
        ByteOrder::Big => "_be",
    };

    let cname = format_ident!("{prefix}{}", kebab_to_rust(&attr.name));
    let cname_buf = format_ident!("_{}{ord}", kebab_to_rust(&attr.name));

    let getter_name = format_ident!("{getter_prefix}{}", kebab_to_rust(&attr.name));
    let setter_name = format_ident!("set_{}", kebab_to_rust(&attr.name));

    let mut getter = quote!(#cname);
    if (attr.is_num() && !attr.byte_order.is_host())
        || matches!(attr.r#type, AttrType::CBitField { .. })
    {
        getter = quote!(#getter_name())
    }

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

    let decode_num = format_ident!("from_{encode_ord}");
    let encode_num = format_ident!("to_{encode_ord}");

    let name = kebab_to_rust(&attr.name);
    // Taken from ./gen_debug_impl.rs
    let debug_format = match attr.display_hint.as_ref().map(|s| s.as_str()) {
        _ if attr.r#enum.is_some() => {
            let next = attr;
            let val_name = quote!(self.#getter);

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
        Some("hex") if attr.is_num() => quote!(&FormatHex(self.#getter.#encode())),
        Some("hex") => quote!(&FormatHex(self.#getter)),
        Some("string") => quote!(&FormatBinStr(self.#getter)),
        _ if matches!(attr.r#type, AttrType::U64 | AttrType::S64) => {
            quote!(&{self.#getter})
        }
        _ if matches!(
            attr.r#type,
            AttrType::Binary {
                r#struct: Some(_),
                ..
            }
        ) =>
        {
            quote!(&self.#getter)
        }
        _ => {
            *default_debug += 1;
            quote!(&self.#getter)
        }
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
        let rust_type = format_ident!("{}", rust_type);

        if m.last_bit_type
            .as_ref()
            .is_none_or(|(s, _, _)| s != sub_type)
            || m.bit_off + bits > max_bits
        {
            let cname_buf = format_ident!("_bits_{}{ord}", kebab_to_rust(&attr.name));

            m.bit_off = 0;
            m.last_bit_type = Some((sub_type.clone(), 0, cname_buf.clone()));
            m.off = align_up(m.off, alignment);
            m.off += len;
            m.alignment = m.alignment.max(alignment);

            members.extend(quote! {
                pub #cname_buf: #rust_type,
            });
        }

        let (_, _, cname_buf) = m.last_bit_type.as_ref().unwrap();

        let bit_off = m.bit_off;
        m.bit_off += bits;

        let shl = u32::BITS - bit_off as u32 - *bits as u32;
        let shr = u32::BITS - *bits as u32;
        cimpl.extend(quote! {
            #docs
            pub fn #getter_name(&self) -> #rust_type {
                (((self.#cname_buf as u32) << #shl) >> #shr) as #rust_type
            }
            #docs
            pub fn #setter_name(&mut self, #value_name: #rust_type) {
                let mask = (1 << #bits) - 1;
                self.#cname_buf = (self.#cname_buf & (!(mask << #bit_off))) | ((#value_name & mask) << #bit_off);
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
        AttrType::Binary {
            r#struct: Some(r#struct),
            ..
        } => {
            let rust_type = struct_type(spec, r#struct);

            let (len, alignment) = gen_struct_len(spec, r#struct);

            insert_padding(spec, members, m, len, alignment);

            members.extend(quote! {
                #docs
                pub #cname: #rust_type,
            });

            return;
        }
        AttrType::Binary {
            r#struct: None,
            len: Some(len),
        } => {
            m.off += len;

            members.extend(quote! {
                #docs
                pub #cname: [u8; #len],
            });

            return;
        }
        other => todo!("{:?}", other),
    };

    let rust_type = format_ident!("{}", rust_type);

    let alignment = len;
    insert_padding(spec, members, m, len, alignment);

    if matches!(attr.byte_order, ByteOrder::Host) {
        members.extend(quote! {
            #docs
            pub #cname: #rust_type,
        });
    } else {
        members.extend(quote! {
            #docs
            pub #cname_buf: #rust_type,
        });
        cimpl.extend(quote! {
            #docs
            pub fn #getter_name(&self) -> #rust_type {
                #rust_type::#decode_num(self.#cname_buf)
            }
            #docs
            pub fn #setter_name(&mut self, #value_name: #rust_type) {
                self.#cname_buf = #value_name.#encode_num();
            }
        });
    }
}

fn insert_padding(
    spec: &Spec,
    members: &mut TokenStream,
    m: &mut GenImplStruct,
    len: usize,
    alignment: usize,
) {
    let pad = align_up(m.off, alignment) - m.off;
    if pad != 0 && (alignment > 4 || spec.experimental.struct_explicit_padding) {
        assert!(alignment <= 8);
        let pad_name = format_ident!("_pad_{}", m.off);
        members.extend(quote! {
            pub #pad_name: [u8; #pad],
        });
    }

    m.off = align_up(m.off, alignment);
    m.alignment = m.alignment.max(alignment);
    m.off += len;
}
