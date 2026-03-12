use std::{ffi::CString, fmt::Write};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    gen_attrs::shorthand_name,
    gen_iterable::iterable_name,
    gen_utils::{kebab_to_type, kebab_to_upper},
    parse_spec::{OperationSpec, Spec},
    Context, WARNING,
};

pub fn notif_group_type(name: &str) -> Ident {
    format_ident!("{}", kebab_to_upper(name))
}

pub fn notif_cgroup_type(name: &str) -> Ident {
    format_ident!("{}_CSTR", kebab_to_upper(name))
}

pub fn notif_type(name: &str) -> Ident {
    format_ident!("{}", kebab_to_type(name))
}

pub fn notif_kebab_name(type_name: &str) -> String {
    let type_name = type_name.strip_suffix("-ntf").unwrap_or(type_name);
    format!("op-{type_name}-notif")
}

pub fn gen_notifs(tokens: &mut TokenStream, spec: &Spec, ctx: &mut Context) {
    if spec
        .protocol
        .as_ref()
        .is_none_or(|p| !p.starts_with("genetlink"))
    {
        return;
    }

    for ops in &spec.operations.list {
        if ops.mcgrp.as_ref().is_none() {
            continue;
        }

        let name = notif_kebab_name(&ops.name);
        gen_notif(tokens, spec, ctx, ops, &name);
    }

    if spec.mcast_groups.is_some() {
        gen_notif_groups(tokens, spec, ctx);
    };
}

pub fn gen_notif(
    tokens: &mut TokenStream,
    spec: &Spec,
    ctx: &mut Context,
    ops: &OperationSpec,
    name: &str,
) {
    let notif_ops = ops.notify.as_ref().map(|op_name| spec.find_op(op_name));
    let notif_op = notif_ops.and_then(|ops| None.or(ops.r#do.as_ref()).or(ops.dump.as_ref()));
    let attrs = None
        .or(notif_op.and_then(|op| op.reply.attribute_set.as_ref()))
        .or(notif_ops.and_then(|ops| ops.attribute_set.as_ref()))
        .or(ops.attribute_set.as_ref())
        .unwrap();
    let iter_attrs = iterable_name(attrs);

    let name = notif_type(name);
    let cmd = ops.value.unwrap() as u8;

    let mut doc_str = String::new();

    if let Some(event) = &ops.event {
        if !event.attributes.is_empty() {
            for attr in event.attributes.iter() {
                let func = shorthand_name(attr);
                writeln!(doc_str, "- [`.{func}()`]({iter_attrs}::{func})").unwrap();
                ctx.test_exprs
                    .insert(format!("let _ = {iter_attrs}::{func};"));
            }
        }
    }

    if let Some(notif_op) = notif_op {
        for attr in notif_op.reply.attributes.iter() {
            let func = shorthand_name(attr);
            writeln!(doc_str, "- [`.{func}()`]({iter_attrs}::{func})").unwrap();
            ctx.test_exprs
                .insert(format!("let _ = {iter_attrs}::{func};"));
        }
    }

    let mut doc = quote!();
    if !doc_str.is_empty() {
        doc_str = format!("Notify attributes:\n{doc_str}");
        doc = quote!(#[doc = #doc_str]);
    }

    tokens.extend(quote! {
        #doc
        #[derive(Debug)]
        pub struct #name;
        impl #name {
            pub const CMD: u8 = #cmd;
            pub fn decode_notif<'a>(buf: &'a [u8]) -> #iter_attrs<'a> {
                let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
                #iter_attrs::with_loc(attrs, buf.as_ptr() as usize)
            }
        }
    });
}

pub fn gen_notif_groups(tokens: &mut TokenStream, spec: &Spec, ctx: &mut Context) {
    let mcast_groups = spec.mcast_groups.as_ref().unwrap();

    if mcast_groups.list.is_empty() {
        return;
    }

    if !spec.operations.transparent {
        for op in &spec.operations.list {
            if op.mcgrp.is_some() && op.r#do.is_none() && op.dump.is_none() {
                println!("{WARNING} Multicast notifications are only implemented for operations marked transparent: true");
            }
        }
        return;
    }

    let mut groups = TokenStream::new();

    for group in &mcast_groups.list {
        let group_name = &group.name;

        let mut doc_str = String::new();
        for op in &spec.operations.list {
            if op.mcgrp.as_ref().is_none_or(|g| g != group_name) {
                continue;
            }

            let notif = notif_type(&notif_kebab_name(&op.name));
            writeln!(doc_str, "- [`{notif}`]").unwrap();
            ctx.test_exprs.insert(format!("let _ = {notif};"));
        }

        if !doc_str.is_empty() {
            doc_str = format!("Notifications:\n{doc_str}");
        }
        if !group.flags.is_empty() {
            writeln!(doc_str, "Flags: {}", group.flags.join(", ")).unwrap();
        }

        let mut doc = quote!();
        if !doc_str.is_empty() {
            doc = quote!(#[doc = #doc_str]);
        }

        let group_type = notif_group_type(group_name);
        let cgroup_type = notif_cgroup_type(group_name);
        let cgroup_name = CString::new(group_name.as_str()).unwrap();
        groups.extend(quote! {
            #doc
            pub const #group_type: &str = #group_name;
            #doc
            pub const #cgroup_type: &CStr = #cgroup_name;
        });
    }

    tokens.extend(quote! {
        pub struct NotifGroup;
        impl NotifGroup {
            #groups
        }
    });
}
