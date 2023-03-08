trait Map {
    type Key<'a>: 'a;
    type Value<'a>: 'a;

    fn get<'a>(&'a self, key: Self::Key<'a>) -> Option<Self::Value<'a>>;
    fn set<'a>(&'a mut self, key: Self::Key<'a>, value: Self::Value<'a>)
        -> Option<Self::Value<'a>>;
}

impl<T> Map for Vec<(T, T)> {
    type Key<'a> = &'a T;
    type Value<'a> = &'a mut T;

    fn get<'a>(&'a self, key: Self::Key<'a>) -> Option<Self::Value<'a>> {
        self.iter_mut().find(|&(k, _)| *k == key).map(|(_, v)| v)
    }

    fn set<'a>(
        &'a mut self,
        key: Self::Key<'a>,
        value: Self::Value<'a>,
    ) -> Option<Self::Value<'a>> {
        match self.iter_mut().find(|&(k, _)| *k == key) {
            Some((_, v)) => {
                let old_value = std::mem::replace(v, value);
                Some(old_value)
            }
            None => {
                self.push((key.clone(), value));
                None
            }
        }
    }
}

fn main() {}
