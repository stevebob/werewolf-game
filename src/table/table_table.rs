use table::{
    TableId,
    ToType,
    Table,
    TableRef,
    TableRefMut,
    EntryTypeTableRef,
};

use std::hash::Hash;

pub trait TableTable<'a, EntryType, Entry>
    where EntryType: 'a + Eq + Hash,
          Entry: 'a + ToType<EntryType>,
{

    type Ref: TableRef<'a, EntryType, Entry>;
    type RefMut: TableRefMut<'a, EntryType, Entry>;
    type EntryTypeRef: EntryTypeTableRef<'a, EntryType, Entry>;

    fn add(&mut self, id: TableId, table: Table<EntryType, Entry>)
        -> Option<Table<EntryType, Entry>>;

    fn remove(&mut self, id: TableId) -> Option<Table<EntryType, Entry>>;

    fn get(&'a self, id: TableId) -> Option<Self::Ref>;
    fn get_mut(&'a mut self, id: TableId) -> Option<Self::RefMut>;

    fn entry_type(&'a self, entry_type: EntryType) -> Option<Self::EntryTypeRef>;
}
