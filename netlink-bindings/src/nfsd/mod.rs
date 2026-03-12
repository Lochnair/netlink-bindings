#![doc = "NFSD configuration over generic netlink\\."]
#![allow(clippy::all)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]
#![allow(unreachable_code)]
#![allow(unreachable_patterns)]
use crate::builtin::{BuiltinBitfield32, BuiltinNfgenmsg, Nlmsghdr, PushDummy};
use crate::{
    consts,
    traits::{NetlinkRequest, Protocol},
    utils::*,
};
pub const PROTONAME: &str = "nfsd";
pub const PROTONAME_CSTR: &CStr = c"nfsd";
#[derive(Clone)]
pub enum RpcStatus<'a> {
    Xid(u32),
    Flags(u32),
    Prog(u32),
    Version(u8),
    Proc(u32),
    ServiceTime(i64),
    Pad(&'a [u8]),
    Saddr4(std::net::Ipv4Addr),
    Daddr4(std::net::Ipv4Addr),
    Saddr6(&'a [u8]),
    Daddr6(&'a [u8]),
    Sport(u16),
    Dport(u16),
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    CompoundOps(u32),
}
impl<'a> IterableRpcStatus<'a> {
    pub fn get_xid(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RpcStatus::Xid(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Xid",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_flags(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RpcStatus::Flags(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Flags",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_prog(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RpcStatus::Prog(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Prog",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_version(&self) -> Result<u8, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RpcStatus::Version(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Version",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_proc(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RpcStatus::Proc(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Proc",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_service_time(&self) -> Result<i64, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RpcStatus::ServiceTime(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "ServiceTime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_pad(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RpcStatus::Pad(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Pad",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_saddr4(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RpcStatus::Saddr4(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Saddr4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_daddr4(&self) -> Result<std::net::Ipv4Addr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RpcStatus::Daddr4(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Daddr4",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_saddr6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RpcStatus::Saddr6(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Saddr6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_daddr6(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RpcStatus::Daddr6(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Daddr6",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_sport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RpcStatus::Sport(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Sport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_dport(&self) -> Result<u16, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let RpcStatus::Dport(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "RpcStatus",
            "Dport",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_compound_ops(&self) -> MultiAttrIterable<Self, RpcStatus<'a>, u32> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let RpcStatus::CompoundOps(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl RpcStatus<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableRpcStatus<'a> {
        IterableRpcStatus::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Xid",
            2u16 => "Flags",
            3u16 => "Prog",
            4u16 => "Version",
            5u16 => "Proc",
            6u16 => "ServiceTime",
            7u16 => "Pad",
            8u16 => "Saddr4",
            9u16 => "Daddr4",
            10u16 => "Saddr6",
            11u16 => "Daddr6",
            12u16 => "Sport",
            13u16 => "Dport",
            14u16 => "CompoundOps",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableRpcStatus<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableRpcStatus<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterableRpcStatus<'a> {
    type Item = Result<RpcStatus<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        let mut r#type;
        loop {
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                break;
            };
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => RpcStatus::Xid({
                    let res = parse_be_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => RpcStatus::Flags({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => RpcStatus::Prog({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => RpcStatus::Version({
                    let res = parse_u8(next);
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => RpcStatus::Proc({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                6u16 => RpcStatus::ServiceTime({
                    let res = parse_i64(next);
                    let Some(val) = res else { break };
                    val
                }),
                7u16 => RpcStatus::Pad({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                8u16 => RpcStatus::Saddr4({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                9u16 => RpcStatus::Daddr4({
                    let res = parse_be_u32(next).map(Ipv4Addr::from_bits);
                    let Some(val) = res else { break };
                    val
                }),
                10u16 => RpcStatus::Saddr6({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                11u16 => RpcStatus::Daddr6({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                12u16 => RpcStatus::Sport({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                13u16 => RpcStatus::Dport({
                    let res = parse_be_u16(next);
                    let Some(val) = res else { break };
                    val
                }),
                14u16 => RpcStatus::CompoundOps({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "RpcStatus",
            r#type.and_then(|t| RpcStatus::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableRpcStatus<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("RpcStatus");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {
                RpcStatus::Xid(val) => fmt.field("Xid", &val),
                RpcStatus::Flags(val) => fmt.field("Flags", &val),
                RpcStatus::Prog(val) => fmt.field("Prog", &val),
                RpcStatus::Version(val) => fmt.field("Version", &val),
                RpcStatus::Proc(val) => fmt.field("Proc", &val),
                RpcStatus::ServiceTime(val) => fmt.field("ServiceTime", &val),
                RpcStatus::Pad(val) => fmt.field("Pad", &val),
                RpcStatus::Saddr4(val) => fmt.field("Saddr4", &val),
                RpcStatus::Daddr4(val) => fmt.field("Daddr4", &val),
                RpcStatus::Saddr6(val) => fmt.field("Saddr6", &val),
                RpcStatus::Daddr6(val) => fmt.field("Daddr6", &val),
                RpcStatus::Sport(val) => fmt.field("Sport", &val),
                RpcStatus::Dport(val) => fmt.field("Dport", &val),
                RpcStatus::CompoundOps(val) => fmt.field("CompoundOps", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableRpcStatus<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("RpcStatus", offset));
            return (
                stack,
                missing_type.and_then(|t| RpcStatus::attr_from_type(t)),
            );
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                RpcStatus::Xid(val) => {
                    if last_off == offset {
                        stack.push(("Xid", last_off));
                        break;
                    }
                }
                RpcStatus::Flags(val) => {
                    if last_off == offset {
                        stack.push(("Flags", last_off));
                        break;
                    }
                }
                RpcStatus::Prog(val) => {
                    if last_off == offset {
                        stack.push(("Prog", last_off));
                        break;
                    }
                }
                RpcStatus::Version(val) => {
                    if last_off == offset {
                        stack.push(("Version", last_off));
                        break;
                    }
                }
                RpcStatus::Proc(val) => {
                    if last_off == offset {
                        stack.push(("Proc", last_off));
                        break;
                    }
                }
                RpcStatus::ServiceTime(val) => {
                    if last_off == offset {
                        stack.push(("ServiceTime", last_off));
                        break;
                    }
                }
                RpcStatus::Pad(val) => {
                    if last_off == offset {
                        stack.push(("Pad", last_off));
                        break;
                    }
                }
                RpcStatus::Saddr4(val) => {
                    if last_off == offset {
                        stack.push(("Saddr4", last_off));
                        break;
                    }
                }
                RpcStatus::Daddr4(val) => {
                    if last_off == offset {
                        stack.push(("Daddr4", last_off));
                        break;
                    }
                }
                RpcStatus::Saddr6(val) => {
                    if last_off == offset {
                        stack.push(("Saddr6", last_off));
                        break;
                    }
                }
                RpcStatus::Daddr6(val) => {
                    if last_off == offset {
                        stack.push(("Daddr6", last_off));
                        break;
                    }
                }
                RpcStatus::Sport(val) => {
                    if last_off == offset {
                        stack.push(("Sport", last_off));
                        break;
                    }
                }
                RpcStatus::Dport(val) => {
                    if last_off == offset {
                        stack.push(("Dport", last_off));
                        break;
                    }
                }
                RpcStatus::CompoundOps(val) => {
                    if last_off == offset {
                        stack.push(("CompoundOps", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("RpcStatus", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Server<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Threads(u32),
    Gracetime(u32),
    Leasetime(u32),
    Scope(&'a CStr),
    MinThreads(u32),
}
impl<'a> IterableServer<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_threads(&self) -> MultiAttrIterable<Self, Server<'a>, u32> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let Server::Threads(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
    pub fn get_gracetime(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Server::Gracetime(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Server",
            "Gracetime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_leasetime(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Server::Leasetime(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Server",
            "Leasetime",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_scope(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Server::Scope(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Server",
            "Scope",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_min_threads(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Server::MinThreads(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Server",
            "MinThreads",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Server<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableServer<'a> {
        IterableServer::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Threads",
            2u16 => "Gracetime",
            3u16 => "Leasetime",
            4u16 => "Scope",
            5u16 => "MinThreads",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableServer<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableServer<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterableServer<'a> {
    type Item = Result<Server<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        let mut r#type;
        loop {
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                break;
            };
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => Server::Threads({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Server::Gracetime({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Server::Leasetime({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                4u16 => Server::Scope({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                5u16 => Server::MinThreads({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Server",
            r#type.and_then(|t| Server::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableServer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Server");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {
                Server::Threads(val) => fmt.field("Threads", &val),
                Server::Gracetime(val) => fmt.field("Gracetime", &val),
                Server::Leasetime(val) => fmt.field("Leasetime", &val),
                Server::Scope(val) => fmt.field("Scope", &val),
                Server::MinThreads(val) => fmt.field("MinThreads", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableServer<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Server", offset));
            return (stack, missing_type.and_then(|t| Server::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Server::Threads(val) => {
                    if last_off == offset {
                        stack.push(("Threads", last_off));
                        break;
                    }
                }
                Server::Gracetime(val) => {
                    if last_off == offset {
                        stack.push(("Gracetime", last_off));
                        break;
                    }
                }
                Server::Leasetime(val) => {
                    if last_off == offset {
                        stack.push(("Leasetime", last_off));
                        break;
                    }
                }
                Server::Scope(val) => {
                    if last_off == offset {
                        stack.push(("Scope", last_off));
                        break;
                    }
                }
                Server::MinThreads(val) => {
                    if last_off == offset {
                        stack.push(("MinThreads", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Server", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum Version {
    Major(u32),
    Minor(u32),
    Enabled(()),
}
impl<'a> IterableVersion<'a> {
    pub fn get_major(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Version::Major(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Version",
            "Major",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_minor(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Version::Minor(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Version",
            "Minor",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_enabled(&self) -> Result<(), ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Version::Enabled(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Version",
            "Enabled",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Version {
    pub fn new<'a>(buf: &'a [u8]) -> IterableVersion<'a> {
        IterableVersion::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Major",
            2u16 => "Minor",
            3u16 => "Enabled",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableVersion<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableVersion<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterableVersion<'a> {
    type Item = Result<Version, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        let mut r#type;
        loop {
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                break;
            };
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => Version::Major({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Version::Minor({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                3u16 => Version::Enabled(()),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Version",
            r#type.and_then(|t| Version::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl std::fmt::Debug for IterableVersion<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Version");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {
                Version::Major(val) => fmt.field("Major", &val),
                Version::Minor(val) => fmt.field("Minor", &val),
                Version::Enabled(val) => fmt.field("Enabled", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableVersion<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Version", offset));
            return (stack, missing_type.and_then(|t| Version::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Version::Major(val) => {
                    if last_off == offset {
                        stack.push(("Major", last_off));
                        break;
                    }
                }
                Version::Minor(val) => {
                    if last_off == offset {
                        stack.push(("Minor", last_off));
                        break;
                    }
                }
                Version::Enabled(val) => {
                    if last_off == offset {
                        stack.push(("Enabled", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Version", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ServerProto<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Version(IterableVersion<'a>),
}
impl<'a> IterableServerProto<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_version(&self) -> MultiAttrIterable<Self, ServerProto<'a>, IterableVersion<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let ServerProto::Version(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl ServerProto<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableServerProto<'a> {
        IterableServerProto::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Version",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableServerProto<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableServerProto<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterableServerProto<'a> {
    type Item = Result<ServerProto<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        let mut r#type;
        loop {
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                break;
            };
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => ServerProto::Version({
                    let res = Some(IterableVersion::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "ServerProto",
            r#type.and_then(|t| ServerProto::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableServerProto<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ServerProto");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {
                ServerProto::Version(val) => fmt.field("Version", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableServerProto<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ServerProto", offset));
            return (
                stack,
                missing_type.and_then(|t| ServerProto::attr_from_type(t)),
            );
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        let mut missing = None;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                ServerProto::Version(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ServerProto", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum Sock<'a> {
    Addr(&'a [u8]),
    TransportName(&'a CStr),
}
impl<'a> IterableSock<'a> {
    pub fn get_addr(&self) -> Result<&'a [u8], ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Sock::Addr(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Sock",
            "Addr",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_transport_name(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let Sock::TransportName(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "Sock",
            "TransportName",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl Sock<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableSock<'a> {
        IterableSock::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Addr",
            2u16 => "TransportName",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableSock<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableSock<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterableSock<'a> {
    type Item = Result<Sock<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        let mut r#type;
        loop {
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                break;
            };
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => Sock::Addr({
                    let res = Some(next);
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => Sock::TransportName({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "Sock",
            r#type.and_then(|t| Sock::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableSock<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Sock");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {
                Sock::Addr(val) => fmt.field("Addr", &val),
                Sock::TransportName(val) => fmt.field("TransportName", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableSock<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("Sock", offset));
            return (stack, missing_type.and_then(|t| Sock::attr_from_type(t)));
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                Sock::Addr(val) => {
                    if last_off == offset {
                        stack.push(("Addr", last_off));
                        break;
                    }
                }
                Sock::TransportName(val) => {
                    if last_off == offset {
                        stack.push(("TransportName", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("Sock", cur));
        }
        (stack, None)
    }
}
#[derive(Clone)]
pub enum ServerSock<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    Addr(IterableSock<'a>),
}
impl<'a> IterableServerSock<'a> {
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn get_addr(&self) -> MultiAttrIterable<Self, ServerSock<'a>, IterableSock<'a>> {
        MultiAttrIterable::new(self.clone(), |variant| {
            if let ServerSock::Addr(val) = variant {
                Some(val)
            } else {
                None
            }
        })
    }
}
impl ServerSock<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterableServerSock<'a> {
        IterableServerSock::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Addr",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterableServerSock<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterableServerSock<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterableServerSock<'a> {
    type Item = Result<ServerSock<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        let mut r#type;
        loop {
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                break;
            };
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => ServerSock::Addr({
                    let res = Some(IterableSock::with_loc(next, self.orig_loc));
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "ServerSock",
            r#type.and_then(|t| ServerSock::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterableServerSock<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("ServerSock");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {
                ServerSock::Addr(val) => fmt.field("Addr", &val),
            };
        }
        fmt.finish()
    }
}
impl IterableServerSock<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("ServerSock", offset));
            return (
                stack,
                missing_type.and_then(|t| ServerSock::attr_from_type(t)),
            );
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        let mut missing = None;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                ServerSock::Addr(val) => {
                    (stack, missing) = val.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("ServerSock", cur));
        }
        (stack, missing)
    }
}
#[derive(Clone)]
pub enum PoolMode<'a> {
    Mode(&'a CStr),
    Npools(u32),
}
impl<'a> IterablePoolMode<'a> {
    pub fn get_mode(&self) -> Result<&'a CStr, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PoolMode::Mode(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PoolMode",
            "Mode",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
    pub fn get_npools(&self) -> Result<u32, ErrorContext> {
        let mut iter = self.clone();
        iter.pos = 0;
        for attr in iter {
            if let PoolMode::Npools(val) = attr? {
                return Ok(val);
            }
        }
        Err(ErrorContext::new_missing(
            "PoolMode",
            "Npools",
            self.orig_loc,
            self.buf.as_ptr() as usize,
        ))
    }
}
impl PoolMode<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> IterablePoolMode<'a> {
        IterablePoolMode::with_loc(buf, buf.as_ptr() as usize)
    }
    fn attr_from_type(r#type: u16) -> Option<&'static str> {
        let res = match r#type {
            1u16 => "Mode",
            2u16 => "Npools",
            _ => return None,
        };
        Some(res)
    }
}
#[derive(Clone, Copy, Default)]
pub struct IterablePoolMode<'a> {
    buf: &'a [u8],
    pos: usize,
    orig_loc: usize,
}
impl<'a> IterablePoolMode<'a> {
    fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
        Self {
            buf,
            pos: 0,
            orig_loc,
        }
    }
    pub fn get_buf(&self) -> &'a [u8] {
        self.buf
    }
}
impl<'a> Iterator for IterablePoolMode<'a> {
    type Item = Result<PoolMode<'a>, ErrorContext>;
    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        let mut r#type;
        loop {
            r#type = None;
            if self.buf.len() == self.pos {
                return None;
            }
            let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                break;
            };
            r#type = Some(header.r#type);
            let res = match header.r#type {
                1u16 => PoolMode::Mode({
                    let res = CStr::from_bytes_with_nul(next).ok();
                    let Some(val) = res else { break };
                    val
                }),
                2u16 => PoolMode::Npools({
                    let res = parse_u32(next);
                    let Some(val) = res else { break };
                    val
                }),
                n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                n => continue,
            };
            return Some(Ok(res));
        }
        Some(Err(ErrorContext::new(
            "PoolMode",
            r#type.and_then(|t| PoolMode::attr_from_type(t)),
            self.orig_loc,
            self.buf.as_ptr().wrapping_add(pos) as usize,
        )))
    }
}
impl<'a> std::fmt::Debug for IterablePoolMode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("PoolMode");
        for attr in self.clone() {
            let attr = match attr {
                Ok(a) => a,
                Err(err) => {
                    fmt.finish()?;
                    f.write_str("Err(")?;
                    err.fmt(f)?;
                    return f.write_str(")");
                }
            };
            match attr {
                PoolMode::Mode(val) => fmt.field("Mode", &val),
                PoolMode::Npools(val) => fmt.field("Npools", &val),
            };
        }
        fmt.finish()
    }
}
impl IterablePoolMode<'_> {
    pub fn lookup_attr(
        &self,
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        let mut stack = Vec::new();
        let cur = ErrorContext::calc_offset(self.orig_loc, self.buf.as_ptr() as usize);
        if missing_type.is_some() && cur == offset {
            stack.push(("PoolMode", offset));
            return (
                stack,
                missing_type.and_then(|t| PoolMode::attr_from_type(t)),
            );
        }
        if cur > offset || cur + self.buf.len() < offset {
            return (stack, None);
        }
        let mut attrs = self.clone();
        let mut last_off = cur + attrs.pos;
        while let Some(attr) = attrs.next() {
            let Ok(attr) = attr else { break };
            match attr {
                PoolMode::Mode(val) => {
                    if last_off == offset {
                        stack.push(("Mode", last_off));
                        break;
                    }
                }
                PoolMode::Npools(val) => {
                    if last_off == offset {
                        stack.push(("Npools", last_off));
                        break;
                    }
                }
                _ => {}
            };
            last_off = cur + attrs.pos;
        }
        if !stack.is_empty() {
            stack.push(("PoolMode", cur));
        }
        (stack, None)
    }
}
pub struct PushRpcStatus<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushRpcStatus<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
    fn as_rec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_rec()
    }
}
impl<Prev: Rec> PushRpcStatus<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_xid(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_flags(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_prog(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_version(mut self, value: u8) -> Self {
        push_header(self.as_rec_mut(), 4u16, 1 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_proc(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_service_time(mut self, value: i64) -> Self {
        push_header(self.as_rec_mut(), 6u16, 8 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_pad(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 7u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_saddr4(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_rec_mut(), 8u16, 4 as u16);
        self.as_rec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    pub fn push_daddr4(mut self, value: std::net::Ipv4Addr) -> Self {
        push_header(self.as_rec_mut(), 9u16, 4 as u16);
        self.as_rec_mut().extend(&value.to_bits().to_be_bytes());
        self
    }
    pub fn push_saddr6(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 10u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_daddr6(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 11u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_sport(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 12u16, 2 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    pub fn push_dport(mut self, value: u16) -> Self {
        push_header(self.as_rec_mut(), 13u16, 2 as u16);
        self.as_rec_mut().extend(value.to_be_bytes());
        self
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn push_compound_ops(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 14u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushRpcStatus<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushServer<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushServer<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
    fn as_rec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_rec()
    }
}
impl<Prev: Rec> PushServer<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn push_threads(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_gracetime(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_leasetime(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 3u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_scope(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            4u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_scope_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 4u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_min_threads(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 5u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushServer<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushVersion<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushVersion<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
    fn as_rec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_rec()
    }
}
impl<Prev: Rec> PushVersion<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_major(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 1u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_minor(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
    pub fn push_enabled(mut self, value: ()) -> Self {
        push_header(self.as_rec_mut(), 3u16, 0 as u16);
        self
    }
}
impl<Prev: Rec> Drop for PushVersion<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushServerProto<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushServerProto<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
    fn as_rec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_rec()
    }
}
impl<Prev: Rec> PushServerProto<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_version(mut self) -> PushVersion<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 1u16);
        PushVersion {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushServerProto<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushSock<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushSock<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
    fn as_rec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_rec()
    }
}
impl<Prev: Rec> PushSock<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_addr(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, value.len() as u16);
        self.as_rec_mut().extend(value);
        self
    }
    pub fn push_transport_name(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            2u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_transport_name_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 2u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
}
impl<Prev: Rec> Drop for PushSock<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushServerSock<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushServerSock<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
    fn as_rec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_rec()
    }
}
impl<Prev: Rec> PushServerSock<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    #[doc = "Attribute may repeat multiple times (treat it as array)"]
    pub fn nested_addr(mut self) -> PushSock<Self> {
        let header_offset = push_nested_header(self.as_rec_mut(), 1u16);
        PushSock {
            prev: Some(self),
            header_offset: Some(header_offset),
        }
    }
}
impl<Prev: Rec> Drop for PushServerSock<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
pub struct PushPoolMode<Prev: Rec> {
    pub(crate) prev: Option<Prev>,
    pub(crate) header_offset: Option<usize>,
}
impl<Prev: Rec> Rec for PushPoolMode<Prev> {
    fn as_rec_mut(&mut self) -> &mut Vec<u8> {
        self.prev.as_mut().unwrap().as_rec_mut()
    }
    fn as_rec(&self) -> &Vec<u8> {
        self.prev.as_ref().unwrap().as_rec()
    }
}
impl<Prev: Rec> PushPoolMode<Prev> {
    pub fn new(prev: Prev) -> Self {
        Self {
            prev: Some(prev),
            header_offset: None,
        }
    }
    pub fn end_nested(mut self) -> Prev {
        let mut prev = self.prev.take().unwrap();
        if let Some(header_offset) = &self.header_offset {
            finalize_nested_header(prev.as_rec_mut(), *header_offset);
        }
        prev
    }
    pub fn push_mode(mut self, value: &CStr) -> Self {
        push_header(
            self.as_rec_mut(),
            1u16,
            value.to_bytes_with_nul().len() as u16,
        );
        self.as_rec_mut().extend(value.to_bytes_with_nul());
        self
    }
    pub fn push_mode_bytes(mut self, value: &[u8]) -> Self {
        push_header(self.as_rec_mut(), 1u16, (value.len() + 1) as u16);
        self.as_rec_mut().extend(value);
        self.as_rec_mut().push(0);
        self
    }
    pub fn push_npools(mut self, value: u32) -> Self {
        push_header(self.as_rec_mut(), 2u16, 4 as u16);
        self.as_rec_mut().extend(value.to_ne_bytes());
        self
    }
}
impl<Prev: Rec> Drop for PushPoolMode<Prev> {
    fn drop(&mut self) {
        if let Some(prev) = &mut self.prev {
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
        }
    }
}
#[doc = "dump pending nfsd rpc\n\nReply attributes:\n- [.get_xid()](IterableRpcStatus::get_xid)\n- [.get_flags()](IterableRpcStatus::get_flags)\n- [.get_prog()](IterableRpcStatus::get_prog)\n- [.get_version()](IterableRpcStatus::get_version)\n- [.get_proc()](IterableRpcStatus::get_proc)\n- [.get_service_time()](IterableRpcStatus::get_service_time)\n- [.get_saddr4()](IterableRpcStatus::get_saddr4)\n- [.get_daddr4()](IterableRpcStatus::get_daddr4)\n- [.get_saddr6()](IterableRpcStatus::get_saddr6)\n- [.get_daddr6()](IterableRpcStatus::get_daddr6)\n- [.get_sport()](IterableRpcStatus::get_sport)\n- [.get_dport()](IterableRpcStatus::get_dport)\n- [.get_compound_ops()](IterableRpcStatus::get_compound_ops)\n"]
#[derive(Debug)]
pub struct OpRpcStatusGetDump<'r> {
    request: Request<'r>,
}
impl<'r> OpRpcStatusGetDump<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self {
            request: request.set_dump(),
        }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushRpcStatus<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushRpcStatus::new(buf)
    }
    pub fn encode(&mut self) -> PushRpcStatus<&mut Vec<u8>> {
        PushRpcStatus::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushRpcStatus<RequestBuf<'r>> {
        PushRpcStatus::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableRpcStatus<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableRpcStatus::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Rec>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 1u8;
        header.version = 1u8;
        prev.as_rec_mut().extend(header.as_slice());
    }
}
impl NetlinkRequest for OpRpcStatusGetDump<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableRpcStatus<'buf>;
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "set the maximum number of running threads\nFlags: admin-perm\nRequest attributes:\n- [.push_threads()](PushServer::push_threads)\n- [.push_gracetime()](PushServer::push_gracetime)\n- [.push_leasetime()](PushServer::push_leasetime)\n- [.push_scope()](PushServer::push_scope)\n- [.push_min_threads()](PushServer::push_min_threads)\n"]
#[derive(Debug)]
pub struct OpThreadsSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpThreadsSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushServer<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushServer::new(buf)
    }
    pub fn encode(&mut self) -> PushServer<&mut Vec<u8>> {
        PushServer::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushServer<RequestBuf<'r>> {
        PushServer::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableServer<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableServer::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Rec>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 2u8;
        header.version = 1u8;
        prev.as_rec_mut().extend(header.as_slice());
    }
}
impl NetlinkRequest for OpThreadsSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableServer<'buf>;
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "get the maximum number of running threads\n\nReply attributes:\n- [.get_threads()](IterableServer::get_threads)\n- [.get_gracetime()](IterableServer::get_gracetime)\n- [.get_leasetime()](IterableServer::get_leasetime)\n- [.get_scope()](IterableServer::get_scope)\n- [.get_min_threads()](IterableServer::get_min_threads)\n"]
#[derive(Debug)]
pub struct OpThreadsGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpThreadsGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushServer<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushServer::new(buf)
    }
    pub fn encode(&mut self) -> PushServer<&mut Vec<u8>> {
        PushServer::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushServer<RequestBuf<'r>> {
        PushServer::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableServer<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableServer::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Rec>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 3u8;
        header.version = 1u8;
        prev.as_rec_mut().extend(header.as_slice());
    }
}
impl NetlinkRequest for OpThreadsGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableServer<'buf>;
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "set nfs enabled versions\nFlags: admin-perm\nRequest attributes:\n- [.nested_version()](PushServerProto::nested_version)\n"]
#[derive(Debug)]
pub struct OpVersionSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpVersionSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushServerProto<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushServerProto::new(buf)
    }
    pub fn encode(&mut self) -> PushServerProto<&mut Vec<u8>> {
        PushServerProto::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushServerProto<RequestBuf<'r>> {
        PushServerProto::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableServerProto<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableServerProto::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Rec>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 4u8;
        header.version = 1u8;
        prev.as_rec_mut().extend(header.as_slice());
    }
}
impl NetlinkRequest for OpVersionSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableServerProto<'buf>;
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "get nfs enabled versions\n\nReply attributes:\n- [.get_version()](IterableServerProto::get_version)\n"]
#[derive(Debug)]
pub struct OpVersionGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpVersionGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushServerProto<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushServerProto::new(buf)
    }
    pub fn encode(&mut self) -> PushServerProto<&mut Vec<u8>> {
        PushServerProto::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushServerProto<RequestBuf<'r>> {
        PushServerProto::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableServerProto<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableServerProto::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Rec>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 5u8;
        header.version = 1u8;
        prev.as_rec_mut().extend(header.as_slice());
    }
}
impl NetlinkRequest for OpVersionGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableServerProto<'buf>;
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "set nfs running sockets\nFlags: admin-perm\nRequest attributes:\n- [.nested_addr()](PushServerSock::nested_addr)\n"]
#[derive(Debug)]
pub struct OpListenerSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpListenerSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushServerSock<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushServerSock::new(buf)
    }
    pub fn encode(&mut self) -> PushServerSock<&mut Vec<u8>> {
        PushServerSock::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushServerSock<RequestBuf<'r>> {
        PushServerSock::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableServerSock<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableServerSock::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Rec>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 6u8;
        header.version = 1u8;
        prev.as_rec_mut().extend(header.as_slice());
    }
}
impl NetlinkRequest for OpListenerSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableServerSock<'buf>;
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "get nfs running listeners\n\nReply attributes:\n- [.get_addr()](IterableServerSock::get_addr)\n"]
#[derive(Debug)]
pub struct OpListenerGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpListenerGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushServerSock<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushServerSock::new(buf)
    }
    pub fn encode(&mut self) -> PushServerSock<&mut Vec<u8>> {
        PushServerSock::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushServerSock<RequestBuf<'r>> {
        PushServerSock::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterableServerSock<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterableServerSock::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Rec>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 7u8;
        header.version = 1u8;
        prev.as_rec_mut().extend(header.as_slice());
    }
}
impl NetlinkRequest for OpListenerGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterableServerSock<'buf>;
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "set the current server pool\\-mode\nFlags: admin-perm\nRequest attributes:\n- [.push_mode()](PushPoolMode::push_mode)\n"]
#[derive(Debug)]
pub struct OpPoolModeSetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPoolModeSetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushPoolMode<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushPoolMode::new(buf)
    }
    pub fn encode(&mut self) -> PushPoolMode<&mut Vec<u8>> {
        PushPoolMode::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushPoolMode<RequestBuf<'r>> {
        PushPoolMode::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterablePoolMode<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePoolMode::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Rec>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 8u8;
        header.version = 1u8;
        prev.as_rec_mut().extend(header.as_slice());
    }
}
impl NetlinkRequest for OpPoolModeSetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterablePoolMode<'buf>;
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf).lookup_attr(offset, missing_type)
    }
}
#[doc = "get info about server pool\\-mode\n\nReply attributes:\n- [.get_mode()](IterablePoolMode::get_mode)\n- [.get_npools()](IterablePoolMode::get_npools)\n"]
#[derive(Debug)]
pub struct OpPoolModeGetDo<'r> {
    request: Request<'r>,
}
impl<'r> OpPoolModeGetDo<'r> {
    pub fn new(mut request: Request<'r>) -> Self {
        Self::write_header(request.buf_mut());
        Self { request: request }
    }
    pub fn encode_request<'buf>(buf: &'buf mut Vec<u8>) -> PushPoolMode<&'buf mut Vec<u8>> {
        Self::write_header(buf);
        PushPoolMode::new(buf)
    }
    pub fn encode(&mut self) -> PushPoolMode<&mut Vec<u8>> {
        PushPoolMode::new(self.request.buf_mut())
    }
    pub fn into_encoder(self) -> PushPoolMode<RequestBuf<'r>> {
        PushPoolMode::new(self.request.buf)
    }
    pub fn decode_request<'a>(buf: &'a [u8]) -> IterablePoolMode<'a> {
        let (_header, attrs) = buf.split_at(buf.len().min(BuiltinNfgenmsg::len()));
        IterablePoolMode::with_loc(attrs, buf.as_ptr() as usize)
    }
    fn write_header<Prev: Rec>(prev: &mut Prev) {
        let mut header = BuiltinNfgenmsg::new();
        header.cmd = 9u8;
        header.version = 1u8;
        prev.as_rec_mut().extend(header.as_slice());
    }
}
impl NetlinkRequest for OpPoolModeGetDo<'_> {
    fn protocol(&self) -> Protocol {
        Protocol::Generic("nfsd".as_bytes())
    }
    fn flags(&self) -> u16 {
        self.request.flags
    }
    fn payload(&self) -> &[u8] {
        self.request.buf()
    }
    type ReplyType<'buf> = IterablePoolMode<'buf>;
    fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
        Self::decode_request(buf)
    }
    fn lookup(
        buf: &[u8],
        offset: usize,
        missing_type: Option<u16>,
    ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
        Self::decode_request(buf).lookup_attr(offset, missing_type)
    }
}
use crate::traits::LookupFn;
use crate::utils::RequestBuf;
#[derive(Debug)]
pub struct Request<'buf> {
    buf: RequestBuf<'buf>,
    flags: u16,
    writeback: Option<&'buf mut Option<RequestInfo>>,
}
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct RequestInfo {
    protocol: Protocol,
    flags: u16,
    name: &'static str,
    lookup: LookupFn,
}
impl Request<'static> {
    pub fn new() -> Self {
        Self::new_from_buf(Vec::new())
    }
    pub fn new_from_buf(buf: Vec<u8>) -> Self {
        Self {
            flags: 0,
            buf: RequestBuf::Own(buf),
            writeback: None,
        }
    }
    pub fn into_buf(self) -> Vec<u8> {
        match self.buf {
            RequestBuf::Own(buf) => buf,
            _ => unreachable!(),
        }
    }
}
impl<'buf> Request<'buf> {
    pub fn new_with_buf(buf: &'buf mut Vec<u8>) -> Self {
        buf.clear();
        Self::new_extend(buf)
    }
    pub fn new_extend(buf: &'buf mut Vec<u8>) -> Self {
        Self {
            flags: 0,
            buf: RequestBuf::Ref(buf),
            writeback: None,
        }
    }
    fn do_writeback(&mut self, protocol: Protocol, name: &'static str, lookup: LookupFn) {
        let Some(writeback) = &mut self.writeback else {
            return;
        };
        **writeback = Some(RequestInfo {
            protocol,
            flags: self.flags,
            name,
            lookup,
        })
    }
    pub fn buf(&self) -> &Vec<u8> {
        self.buf.buf()
    }
    pub fn buf_mut(&mut self) -> &mut Vec<u8> {
        self.buf.buf_mut()
    }
    #[doc = "Set `NLM_F_CREATE` flag"]
    pub fn set_create(mut self) -> Self {
        self.flags |= consts::NLM_F_CREATE as u16;
        self
    }
    #[doc = "Set `NLM_F_EXCL` flag"]
    pub fn set_excl(mut self) -> Self {
        self.flags |= consts::NLM_F_EXCL as u16;
        self
    }
    #[doc = "Set `NLM_F_REPLACE` flag"]
    pub fn set_replace(mut self) -> Self {
        self.flags |= consts::NLM_F_REPLACE as u16;
        self
    }
    #[doc = "Set `NLM_F_CREATE` and `NLM_F_REPLACE` flag"]
    pub fn set_change(self) -> Self {
        self.set_create().set_replace()
    }
    #[doc = "Set `NLM_F_APPEND` flag"]
    pub fn set_append(mut self) -> Self {
        self.flags |= consts::NLM_F_APPEND as u16;
        self
    }
    #[doc = "Set `self.flags |= flags`"]
    pub fn set_flags(mut self, flags: u16) -> Self {
        self.flags |= flags;
        self
    }
    #[doc = "Set `self.flags ^= self.flags & flags`"]
    pub fn unset_flags(mut self, flags: u16) -> Self {
        self.flags ^= self.flags & flags;
        self
    }
    #[doc = "Set `NLM_F_DUMP` flag"]
    fn set_dump(mut self) -> Self {
        self.flags |= consts::NLM_F_DUMP as u16;
        self
    }
    #[doc = "dump pending nfsd rpc\n\nReply attributes:\n- [.get_xid()](IterableRpcStatus::get_xid)\n- [.get_flags()](IterableRpcStatus::get_flags)\n- [.get_prog()](IterableRpcStatus::get_prog)\n- [.get_version()](IterableRpcStatus::get_version)\n- [.get_proc()](IterableRpcStatus::get_proc)\n- [.get_service_time()](IterableRpcStatus::get_service_time)\n- [.get_saddr4()](IterableRpcStatus::get_saddr4)\n- [.get_daddr4()](IterableRpcStatus::get_daddr4)\n- [.get_saddr6()](IterableRpcStatus::get_saddr6)\n- [.get_daddr6()](IterableRpcStatus::get_daddr6)\n- [.get_sport()](IterableRpcStatus::get_sport)\n- [.get_dport()](IterableRpcStatus::get_dport)\n- [.get_compound_ops()](IterableRpcStatus::get_compound_ops)\n"]
    pub fn op_rpc_status_get_dump(self) -> OpRpcStatusGetDump<'buf> {
        let mut res = OpRpcStatusGetDump::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-rpc-status-get-dump",
            OpRpcStatusGetDump::lookup,
        );
        res
    }
    #[doc = "set the maximum number of running threads\nFlags: admin-perm\nRequest attributes:\n- [.push_threads()](PushServer::push_threads)\n- [.push_gracetime()](PushServer::push_gracetime)\n- [.push_leasetime()](PushServer::push_leasetime)\n- [.push_scope()](PushServer::push_scope)\n- [.push_min_threads()](PushServer::push_min_threads)\n"]
    pub fn op_threads_set_do(self) -> OpThreadsSetDo<'buf> {
        let mut res = OpThreadsSetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-threads-set-do", OpThreadsSetDo::lookup);
        res
    }
    #[doc = "get the maximum number of running threads\n\nReply attributes:\n- [.get_threads()](IterableServer::get_threads)\n- [.get_gracetime()](IterableServer::get_gracetime)\n- [.get_leasetime()](IterableServer::get_leasetime)\n- [.get_scope()](IterableServer::get_scope)\n- [.get_min_threads()](IterableServer::get_min_threads)\n"]
    pub fn op_threads_get_do(self) -> OpThreadsGetDo<'buf> {
        let mut res = OpThreadsGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-threads-get-do", OpThreadsGetDo::lookup);
        res
    }
    #[doc = "set nfs enabled versions\nFlags: admin-perm\nRequest attributes:\n- [.nested_version()](PushServerProto::nested_version)\n"]
    pub fn op_version_set_do(self) -> OpVersionSetDo<'buf> {
        let mut res = OpVersionSetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-version-set-do", OpVersionSetDo::lookup);
        res
    }
    #[doc = "get nfs enabled versions\n\nReply attributes:\n- [.get_version()](IterableServerProto::get_version)\n"]
    pub fn op_version_get_do(self) -> OpVersionGetDo<'buf> {
        let mut res = OpVersionGetDo::new(self);
        res.request
            .do_writeback(res.protocol(), "op-version-get-do", OpVersionGetDo::lookup);
        res
    }
    #[doc = "set nfs running sockets\nFlags: admin-perm\nRequest attributes:\n- [.nested_addr()](PushServerSock::nested_addr)\n"]
    pub fn op_listener_set_do(self) -> OpListenerSetDo<'buf> {
        let mut res = OpListenerSetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-listener-set-do",
            OpListenerSetDo::lookup,
        );
        res
    }
    #[doc = "get nfs running listeners\n\nReply attributes:\n- [.get_addr()](IterableServerSock::get_addr)\n"]
    pub fn op_listener_get_do(self) -> OpListenerGetDo<'buf> {
        let mut res = OpListenerGetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-listener-get-do",
            OpListenerGetDo::lookup,
        );
        res
    }
    #[doc = "set the current server pool\\-mode\nFlags: admin-perm\nRequest attributes:\n- [.push_mode()](PushPoolMode::push_mode)\n"]
    pub fn op_pool_mode_set_do(self) -> OpPoolModeSetDo<'buf> {
        let mut res = OpPoolModeSetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-pool-mode-set-do",
            OpPoolModeSetDo::lookup,
        );
        res
    }
    #[doc = "get info about server pool\\-mode\n\nReply attributes:\n- [.get_mode()](IterablePoolMode::get_mode)\n- [.get_npools()](IterablePoolMode::get_npools)\n"]
    pub fn op_pool_mode_get_do(self) -> OpPoolModeGetDo<'buf> {
        let mut res = OpPoolModeGetDo::new(self);
        res.request.do_writeback(
            res.protocol(),
            "op-pool-mode-get-do",
            OpPoolModeGetDo::lookup,
        );
        res
    }
}
#[cfg(test)]
mod generated_tests {
    use super::*;
    #[test]
    fn tests() {
        let _ = IterablePoolMode::get_mode;
        let _ = IterablePoolMode::get_npools;
        let _ = IterableRpcStatus::get_compound_ops;
        let _ = IterableRpcStatus::get_daddr4;
        let _ = IterableRpcStatus::get_daddr6;
        let _ = IterableRpcStatus::get_dport;
        let _ = IterableRpcStatus::get_flags;
        let _ = IterableRpcStatus::get_proc;
        let _ = IterableRpcStatus::get_prog;
        let _ = IterableRpcStatus::get_saddr4;
        let _ = IterableRpcStatus::get_saddr6;
        let _ = IterableRpcStatus::get_service_time;
        let _ = IterableRpcStatus::get_sport;
        let _ = IterableRpcStatus::get_version;
        let _ = IterableRpcStatus::get_xid;
        let _ = IterableServer::get_gracetime;
        let _ = IterableServer::get_leasetime;
        let _ = IterableServer::get_min_threads;
        let _ = IterableServer::get_scope;
        let _ = IterableServer::get_threads;
        let _ = IterableServerProto::get_version;
        let _ = IterableServerSock::get_addr;
        let _ = PushPoolMode::<&mut Vec<u8>>::push_mode;
        let _ = PushServer::<&mut Vec<u8>>::push_gracetime;
        let _ = PushServer::<&mut Vec<u8>>::push_leasetime;
        let _ = PushServer::<&mut Vec<u8>>::push_min_threads;
        let _ = PushServer::<&mut Vec<u8>>::push_scope;
        let _ = PushServer::<&mut Vec<u8>>::push_threads;
        let _ = PushServerProto::<&mut Vec<u8>>::nested_version;
        let _ = PushServerSock::<&mut Vec<u8>>::nested_addr;
    }
}
