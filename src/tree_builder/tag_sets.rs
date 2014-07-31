// Copyright 2014 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Various sets of HTML tag names, and macros for declaring them.

#![macro_escape]

use util::atom::Atom;
use util::namespace::{Namespace, HTML};

//§ the-stack-of-open-elements
pub type TagSet<'a> = |(Namespace, Atom)|: 'a -> bool;

macro_rules! declare_tag_set_impl ( ($param:ident $b:expr $supr:ident $($tag:ident)+) => (
    match $param {
        $( (HTML, atom!($tag)) => $b, )+
        p => $supr(p),
    }
))

macro_rules! declare_tag_set_body (
    ($param:ident $supr:ident - $($tag:ident)+)
        => ( declare_tag_set_impl!($param false $supr $($tag)+) );

    ($param:ident $supr:ident + $($tag:ident)+)
        => ( declare_tag_set_impl!($param true $supr $($tag)+) );

    ($param:ident $($tag:ident)+)
        => ( declare_tag_set_impl!($param true empty_set $($tag)+) );
)

macro_rules! declare_tag_set (
    (pub $name:ident = $($toks:tt)+) => (
        pub fn $name(p: (Namespace, Atom)) -> bool {
            declare_tag_set_body!(p $($toks)+)
        }
    );

    ($name:ident = $($toks:tt)+) => (
        fn $name(p: (Namespace, Atom)) -> bool {
            declare_tag_set_body!(p $($toks)+)
        }
    );
)

#[inline(always)] pub fn empty_set(_: (Namespace, Atom)) -> bool { false }
#[inline(always)] pub fn full_set(_: (Namespace, Atom)) -> bool { true }

// FIXME: MathML, SVG
declare_tag_set!(pub default_scope = applet caption html table td th marquee object template)

declare_tag_set!(pub list_item_scope = default_scope + ol ul)
declare_tag_set!(pub button_scope = default_scope + button)
declare_tag_set!(pub table_scope = html table template)
declare_tag_set!(pub select_scope = full_set - optgroup option)

declare_tag_set!(pub table_body_context = tbody tfoot thead template html)
declare_tag_set!(pub table_row_context = tr template html)
declare_tag_set!(pub td_th = td th)

declare_tag_set!(pub cursory_implied_end = dd dt li option optgroup p rp rt)

declare_tag_set!(pub thorough_implied_end = cursory_implied_end
    + caption colgroup tbody td tfoot th thead tr)

declare_tag_set!(pub heading_tag = h1 h2 h3 h4 h5 h6)

declare_tag_set!(pub special_tag =
    address applet area article aside base basefont bgsound blockquote body br button caption
    center col colgroup dd details dir div dl dt embed fieldset figcaption figure footer form
    frame frameset h1 h2 h3 h4 h5 h6 head header hgroup hr html iframe img input isindex li
    link listing main marquee menu menuitem meta nav noembed noframes noscript object ol p
    param plaintext pre script section select source style summary table tbody td template
    textarea tfoot th thead title tr track ul wbr xmp)

#[allow(dead_code)]
fn unused_tag_sets() {
    // FIXME: Some tag sets are unused until we implement <template> or other stuff.
    // Suppress the warning here.
    thorough_implied_end((HTML, atom!(p)));
}
//§ END
