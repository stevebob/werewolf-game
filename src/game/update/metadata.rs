use table::{
    Table,
    ToType,
};

pub type Metadata = Table<MetadatumType, Metadatum>;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum MetadatumType {
    TurnTime,
    ActionTime,
}

pub enum Metadatum {
    TurnTime(u64),
    ActionTime(u64),
}

impl ToType<MetadatumType> for Metadatum {
    fn to_type(&self) -> MetadatumType {
        match *self {
            Metadatum::TurnTime(_) => MetadatumType::TurnTime,
            Metadatum::ActionTime(_) => MetadatumType::ActionTime,
        }
    }
}