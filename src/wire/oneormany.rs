use std::fmt;

use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Serialize,
};

use crate::OneOrMany;

impl<'de, T: Deserialize<'de>> Deserialize<'de> for OneOrMany<T> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct OneOrManyVisitor<T> {
            _marker: std::marker::PhantomData<T>,
        };

        impl<'de, T: Deserialize<'de>> Visitor<'de> for OneOrManyVisitor<T> {
            type Value = OneOrMany<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "an array or a single entry")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<OneOrMany<T>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut res = Vec::new();
                while let Some(element) = seq.next_element::<T>()? {
                    res.push(element);
                }
                Ok(OneOrMany::Many(res))
            }

            fn visit_map<V>(self, map: V) -> Result<OneOrMany<T>, V::Error>
            where
                V: MapAccess<'de>,
            {
                Ok(OneOrMany::One(Deserialize::deserialize(
                    de::value::MapAccessDeserializer::new(map),
                )?))
            }
        }

        deserializer.deserialize_any(OneOrManyVisitor::<T> {
            _marker: std::marker::PhantomData,
        })
    }
}
