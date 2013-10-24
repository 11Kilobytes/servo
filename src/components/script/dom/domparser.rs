/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::DOMParserBinding;
use dom::bindings::codegen::DOMParserBinding::SupportedTypeValues::{Text_html, Text_xml};
use dom::bindings::utils::{DOMString, Fallible, Reflector, Reflectable, FailureUnknown};
use dom::document::{AbstractDocument, Document, XML};
use dom::htmldocument::HTMLDocument;
use dom::window::Window;

pub struct DOMParser {
    owner: @mut Window, //XXXjdm Document instead?
    reflector_: Reflector
}

impl DOMParser {
    pub fn new(owner: @mut Window) -> @mut DOMParser {
        let parser = @mut DOMParser {
            owner: owner,
            reflector_: Reflector::new()
        };

        // TODO(tkuehn): This just handles the top-level page. Need to handle subframes.
        let cx = owner.get_cx();
        let scope = owner.reflector().get_jsobject();
        parser.wrap_object_shared(cx, scope);
        parser
    }

    pub fn Constructor(owner: @mut Window) -> Fallible<@mut DOMParser> {
        Ok(DOMParser::new(owner))
    }

    pub fn ParseFromString(&self,
                           _s: &DOMString,
                           ty: DOMParserBinding::SupportedType)
                           -> Fallible<AbstractDocument> {
        let cx = self.owner.get_cx();
        match ty {
            Text_html => {
                Ok(HTMLDocument::new(self.owner))
            }
            Text_xml => {
                Ok(AbstractDocument::as_abstract(cx, @mut Document::new(self.owner, XML)))
            }
            _ => {
                Err(FailureUnknown)
            }
        }
    }
}

