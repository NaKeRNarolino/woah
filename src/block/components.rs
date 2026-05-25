use eo::sjson::HasSJsonIdent;

pub trait Component {
    const ID: &'static str;
}

include!(concat!(env!("OUT_DIR"), "/block/components.rs"));
