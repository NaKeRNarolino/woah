use eo::sjson::HasSJsonIdent;

pub trait Component {
    const ID: &'static str;
}

include!(concat!(env!("OUT_DIR"), "/item/components.rs"));
