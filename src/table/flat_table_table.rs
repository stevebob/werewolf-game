use table::{
    TableId,
    TableTable,
    Table,
    ToType,
    TableRef,
    TableRefMut,
    IterTableRef,
    IdTableRef,
    TypeIdMap,
    EntryTypeTableRef,
};

use std::collections::{
    HashMap,
    HashSet,
    hash_set,
};

use std::hash::Hash;

#[derive(Hash, Clone, Copy, Eq, PartialEq, Debug)]
struct Key<EntryType: Hash + Copy + Eq> {
    id: TableId,
    entry_type: EntryType,
}

impl<EntryType: Hash + Copy + Eq> Key<EntryType> {
    fn new(id: TableId, entry_type: EntryType) -> Self {
        Key {
            id: id,
            entry_type: entry_type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FlatTableTable<EntryType, Entry>
where EntryType: Eq + Hash + Copy,
      Entry: ToType<EntryType>,
{
    tables: HashMap<Key<EntryType>, Entry>,
    entry_types: HashMap<TableId, HashSet<EntryType>>,
    entry_type_map: TypeIdMap<EntryType>,
}

impl<EntryType, Entry> FlatTableTable<EntryType, Entry>
where EntryType: Eq + Hash + Copy,
      Entry: ToType<EntryType>,
{
    pub fn new() -> Self {
        FlatTableTable {
            tables: HashMap::new(),
            entry_types: HashMap::new(),
            entry_type_map: TypeIdMap::new(),
        }
    }

    fn ensure_entry_type(&mut self, id: TableId) {
        if !self.entry_types.contains_key(&id) {
            self.entry_types.insert(id, HashSet::new());
        }
    }

    fn add_type(&mut self, id: TableId, entry_type: EntryType) {
        self.ensure_entry_type(id);
        self.entry_types.get_mut(&id).unwrap().insert(entry_type);
        self.entry_type_map.add(id, entry_type);
    }

    fn remove_type(&mut self, id: TableId, entry_type: EntryType) {
        self.ensure_entry_type(id);
        self.entry_types.get_mut(&id).unwrap().remove(&entry_type);
        self.entry_type_map.remove(id, entry_type);
    }
}

pub struct FlatTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    id: TableId,
    table_table: &'a FlatTableTable<EntryType, Entry>,
}

impl<'a, EntryType, Entry> FlatTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    fn new(id: TableId, table_table: &'a FlatTableTable<EntryType, Entry>) -> Self {
        FlatTableRef {
            id: id,
            table_table: table_table,
        }
    }
}

impl<'a, EntryType, Entry> Clone for FlatTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    fn clone(&self) -> Self {
        FlatTableRef::new(self.id, self.table_table)
    }
}

impl<'a, EntryType, Entry> Copy for FlatTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>
{}


pub struct FlatTableRefMut<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    id: TableId,
    table_table: &'a mut FlatTableTable<EntryType, Entry>,
}

impl<'a, EntryType, Entry> FlatTableRefMut<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    fn new(id: TableId, table_table: &'a mut FlatTableTable<EntryType, Entry>) -> Self {
        FlatTableRefMut {
            id: id,
            table_table: table_table,
        }
    }
}

impl<'a, EntryType, Entry> TableRef<'a, EntryType, Entry> for FlatTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    fn get(self, entry_type: EntryType) -> Option<&'a Entry> {
        self.table_table.tables.get(&Key::new(self.id, entry_type))
    }

    fn has(self, entry_type: EntryType) -> bool {
        self.table_table.tables.contains_key(&Key::new(self.id, entry_type))
    }
}

impl<'a, EntryType, Entry> TableRefMut<'a, EntryType, Entry> for FlatTableRefMut<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    fn add(&mut self, entry: Entry) -> Option<Entry> {
        let entry_type = entry.to_type();
        self.table_table.add_type(self.id, entry_type);
        self.table_table.tables.insert(Key::new(self.id, entry_type), entry)
    }

    fn remove(&mut self, t: EntryType) -> Option<Entry> {
        self.table_table.remove_type(self.id, t);
        self.table_table.tables.remove(&Key::new(self.id, t))
    }

    fn get_mut(&mut self, t: EntryType) -> Option<&mut Entry> {
        self.table_table.tables.get_mut(&Key::new(self.id, t))
    }
}

pub struct FlatTableIter<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    id: TableId,
    hash_set_iter: hash_set::Iter<'a, EntryType>,
    table_table: &'a FlatTableTable<EntryType, Entry>,
}

impl<'a, EntryType, Entry> Iterator for FlatTableIter<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    type Item = (&'a EntryType, &'a Entry);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry_type) = self.hash_set_iter.next() {
            Some((entry_type, self.table_table.tables.get(&Key::new(self.id, *entry_type)).unwrap()))
        } else {
            None
        }
    }
}

pub struct FlatTableEntryIter<'a, EntryType, Entry>(FlatTableIter<'a, EntryType, Entry>)
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>;

impl<'a, EntryType, Entry> Iterator for FlatTableEntryIter<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    type Item = &'a Entry;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((_, entry)) = self.0.next() {
            Some(entry)
        } else {
            None
        }
    }
}

impl<'a, EntryType, Entry> IterTableRef<'a, EntryType, Entry> for FlatTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    type Iter = FlatTableIter<'a, EntryType, Entry>;
    type TypeIter = hash_set::Iter<'a, EntryType>;
    type EntryIter = FlatTableEntryIter<'a, EntryType, Entry>;

    fn slots(self) -> Self::Iter {
        FlatTableIter {
            id: self.id,
            hash_set_iter: self.types(),
            table_table: self.table_table,
        }
    }

    fn entries(self) -> Self::EntryIter {
        FlatTableEntryIter(self.slots())
    }

    fn types(self) -> Self::TypeIter {
        self.table_table.entry_types.get(&self.id).unwrap().iter()
    }
}

impl<'a, EntryType, Entry> IdTableRef<'a, EntryType, Entry> for FlatTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    fn id(self) -> TableId {
        self.id
    }
}

impl<'a, EntryType, Entry> TableTable<'a, EntryType, Entry>
for FlatTableTable<EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    type Ref = FlatTableRef<'a, EntryType, Entry>;
    type RefMut = FlatTableRefMut<'a, EntryType, Entry>;
    type EntryTypeRef = FlatEntryTypeTableRef<'a, EntryType, Entry>;

    fn add(&mut self, id: TableId, mut table: Table<EntryType, Entry>)
        -> Option<Table<EntryType, Entry>>
    {
        // clean up existing table under the given id
        let ret = self.remove(id);

        // add new table
        for (entry_type, entry) in table.slots.drain() {
            let key = Key::new(id, entry_type);
            self.tables.insert(key, entry);
            self.add_type(id, entry_type);
        }

        ret
    }

    fn remove(&mut self, id: TableId) -> Option<Table<EntryType, Entry>> {
        if let Some(mut entry_types) = self.entry_types.remove(&id) {
            let mut table = Table::<EntryType, Entry>::new();
            for entry_type in entry_types.drain() {
                let key = Key::new(id, entry_type);
                let entry = self.tables.remove(&key).unwrap();
                table.add(entry);
            }
            Some(table)
        } else {
            None
        }
    }

    fn get(&'a self, id: TableId) -> Option<Self::Ref> {
        if self.entry_types.contains_key(&id) {
            Some(FlatTableRef::new(id, self))
        } else {
            None
        }
    }

    fn get_mut(&'a mut self, id: TableId) -> Option<Self::RefMut> {
        if self.entry_types.contains_key(&id) {
            Some(FlatTableRefMut::new(id, self))
        } else {
            None
        }
    }

    fn entry_type(&'a self, entry_type: EntryType) -> Option<Self::EntryTypeRef> {
        if let Some(ids) = self.entry_type_map.get(entry_type) {
            Some(FlatEntryTypeTableRef::new(entry_type, &self, ids))
        } else {
            None
        }
    }
}

pub struct FlatEntryTypeTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    entry_type: EntryType,
    table_table: &'a FlatTableTable<EntryType, Entry>,
    ids: &'a HashSet<TableId>,
}

impl<'a, EntryType, Entry> FlatEntryTypeTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    fn new(entry_type: EntryType,
           table_table: &'a FlatTableTable<EntryType, Entry>,
           ids: &'a HashSet<TableId>) -> Self
    {
        FlatEntryTypeTableRef {
            entry_type: entry_type,
            table_table: table_table,
            ids: ids,
        }
    }
}

impl<'a, EntryType, Entry> Clone for FlatEntryTypeTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    fn clone(&self) -> Self {
        FlatEntryTypeTableRef::new(self.entry_type, self.table_table, self.ids)
    }
}

impl<'a, EntryType, Entry> Copy for FlatEntryTypeTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>
{}

pub struct FlatEntryIter<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    entry_type_table_ref: FlatEntryTypeTableRef<'a, EntryType, Entry>,
    iter: hash_set::Iter<'a, TableId>,
}

impl<'a, EntryType, Entry> Iterator for FlatEntryIter<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    type Item = FlatTableRef<'a, EntryType, Entry>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(id) = self.iter.next() {
            Some(self.entry_type_table_ref.table_table.get(*id).unwrap())
        } else {
            None
        }
    }
}

impl<'a, EntryType, Entry> EntryTypeTableRef <'a, EntryType, Entry>
for FlatEntryTypeTableRef<'a, EntryType, Entry>
where EntryType: 'a + Eq + Hash + Copy,
      Entry: 'a + ToType<EntryType>,
{
    type Ref = FlatTableRef<'a, EntryType, Entry>;
    type IdIter = hash_set::Iter<'a, TableId>;
    type Iter = FlatEntryIter<'a, EntryType, Entry>;

    fn iter(self) -> Self::Iter {
        FlatEntryIter {
            entry_type_table_ref: self,
            iter: self.id_iter(),
        }
    }

    fn id_iter(self) -> Self::IdIter {
        self.ids.iter()
    }
}
