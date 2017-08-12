/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use syn;

use bindgen::ir::Repr;

pub trait IterHelpers : Iterator {
    fn try_skip_map<F, T, E>(&mut self, f: F) -> Result<Vec<T>, E>
        where F: FnMut(&Self::Item) -> Result<Option<T>, E>;
}

impl<I> IterHelpers for I where I: Iterator {
    fn try_skip_map<F, T, E>(&mut self, mut f: F) -> Result<Vec<T>, E>
        where F: FnMut(&Self::Item) -> Result<Option<T>, E>
    {
        let mut out = Vec::new();
        while let Some(item) = self.next() {
            if let Some(x) = f(&item)? {
                out.push(x);
            }
        }
        Ok(out)
    }
}

pub fn find_first_some<T>(slice: &[Option<T>]) -> Option<&T> {
    for x in slice {
        if let &Some(ref x) = x {
            return Some(x);
        }
    }
    return None;
}

pub trait SynItemHelpers {
    fn has_attr(&self, target: syn::MetaItem) -> bool;

    fn get_doc_attr(&self) -> String;

    fn is_no_mangle(&self) -> bool {
        self.has_attr(syn::MetaItem::Word(syn::Ident::new("no_mangle")))
    }

    fn is_repr_c(&self) -> bool {
        let repr_args = vec![syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(syn::Ident::new("C")))];
        self.has_attr(syn::MetaItem::List(syn::Ident::new("repr"), repr_args))
    }

    fn is_repr_u32(&self) -> bool {
        let repr_args = vec![syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(syn::Ident::new("u32")))];
        self.has_attr(syn::MetaItem::List(syn::Ident::new("repr"), repr_args))
    }

    fn is_repr_u16(&self) -> bool {
        let repr_args = vec![syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(syn::Ident::new("u16")))];
        self.has_attr(syn::MetaItem::List(syn::Ident::new("repr"), repr_args))
    }

    fn is_repr_u8(&self) -> bool {
        let repr_args = vec![syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(syn::Ident::new("u8")))];
        self.has_attr(syn::MetaItem::List(syn::Ident::new("repr"), repr_args))
    }

    fn get_repr(&self) -> Repr {
        if self.is_repr_c() {
            Repr::C
        } else if self.is_repr_u32() {
            Repr::U32
        } else if self.is_repr_u16() {
            Repr::U16
        } else if self.is_repr_u8() {
            Repr::U8
        } else {
            Repr::None
        }
    }
}

impl SynItemHelpers for syn::Item {
    fn has_attr(&self, target: syn::MetaItem) -> bool {
        return self.attrs
                   .iter()
                   .any(|ref attr| attr.style == syn::AttrStyle::Outer && attr.value == target);
    }

    fn get_doc_attr(&self) -> String {
        let mut doc = String::new();
        for attr in &self.attrs {
            if attr.style == syn::AttrStyle::Outer &&
               attr.is_sugared_doc {
                if let syn::MetaItem::NameValue(_, syn::Lit::Str(ref comment, _)) = attr.value {
                    doc.push_str(&comment);
                    doc.push('\n');
                }
            }
        }
        doc
    }
}

impl SynItemHelpers for syn::ForeignItem {
    fn has_attr(&self, target: syn::MetaItem) -> bool {
        return self.attrs
                   .iter()
                   .any(|ref attr| attr.style == syn::AttrStyle::Outer && attr.value == target);
    }

    fn get_doc_attr(&self) -> String {
        let mut doc = String::new();
        for attr in &self.attrs {
            if attr.style == syn::AttrStyle::Outer &&
               attr.is_sugared_doc {
                if let syn::MetaItem::NameValue(_, syn::Lit::Str(ref comment, _)) = attr.value {
                    doc.push_str(&comment);
                    doc.push('\n');
                }
            }
        }
        doc
    }
}

impl SynItemHelpers for syn::Variant {
    fn has_attr(&self, target: syn::MetaItem) -> bool {
        return self.attrs
                   .iter()
                   .any(|ref attr| attr.style == syn::AttrStyle::Outer && attr.value == target);
    }

    fn get_doc_attr(&self) -> String {
        let mut doc = String::new();
        for attr in &self.attrs {
            if attr.style == syn::AttrStyle::Outer &&
               attr.is_sugared_doc {
                if let syn::MetaItem::NameValue(_, syn::Lit::Str(ref comment, _)) = attr.value {
                    doc.push_str(&comment);
                    doc.push('\n');
                }
            }
        }
        doc
    }
}

impl SynItemHelpers for syn::Field {
    fn has_attr(&self, target: syn::MetaItem) -> bool {
        return self.attrs
                   .iter()
                   .any(|ref attr| attr.style == syn::AttrStyle::Outer && attr.value == target);
    }

    fn get_doc_attr(&self) -> String {
        let mut doc = String::new();
        for attr in &self.attrs {
            if attr.style == syn::AttrStyle::Outer &&
               attr.is_sugared_doc {
                if let syn::MetaItem::NameValue(_, syn::Lit::Str(ref comment, _)) = attr.value {
                    doc.push_str(&comment);
                    doc.push('\n');
                }
            }
        }
        doc
    }
}

/// Helper function for accessing Abi information
pub trait SynAbiHelpers {
    fn is_c(&self) -> bool;
}

impl SynAbiHelpers for Option<syn::Abi> {
    fn is_c(&self) -> bool {
        self == &Some(syn::Abi::Named(String::from("C")))
    }
}

impl SynAbiHelpers for syn::Abi {
    fn is_c(&self) -> bool {
        self == &syn::Abi::Named(String::from("C"))
    }
}
