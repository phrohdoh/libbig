#![cfg(feature = "neon")]

use neon;
use neon::vm::{Call, JsResult};
use neon::mem::Handle;
use neon::js::{JsString, JsNumber, JsObject};

use ::BigArchive;

impl From<::errors::ReadError> for neon::vm::Throw {
    fn from(e: ::errors::ReadError) -> Self {
        neon::vm::Throw
    }
}

pub fn libbig_bigarchive_new_from_path(call: Call) -> JsResult<JsObject> {
    let path = call.args.require(call.scope, 0)?.check::<JsString>()?.value();
    Ok(try!(BigArchive::new_from_path(&path)))
}

register_module!(m, {
    m.export("libbig_bigarchive_new_from_path",
                libbig_bigarchive_new_from_path)?;
    Ok(())
});

declare_types! {
    pub class JsBigArchive for BigArchive<T> {
        init(call) {
            let scope = call.scope;
            let path = call.args.require(call.scope, 0)?.check::<JsString>()?.value();
            Ok(try!(BigArchive::new_from_path(&path)))
        }
    }
}