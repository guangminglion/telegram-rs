use serde::de::{self, Deserialize};
use byteorder::{LittleEndian, ReadBytesExt};
use errors::*;

pub struct Deserializer<R>
    where R: ReadBytesExt
{
    reader: R,
}

impl<R> Deserializer<R>
    where R: ReadBytesExt
{
    pub fn new(reader: R) -> Self {
        Deserializer { reader: reader }
    }
}

impl<'a, R> de::Deserializer for &'a mut Deserializer<R>
    where R: ReadBytesExt
{
    type Error = Error;

    #[inline]
    fn deserialize<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        Err(de::Error::custom("Telegram does not support Deserializer::deserialize"))
    }

    #[inline]
    fn deserialize_bool<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        // NOTE: Telegram has no representation for this.
        Err(de::Error::custom("Telegram does not support Deserializer::deserialize_bool"))
    }

    #[inline]
    fn deserialize_u8<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        let value = self.reader.read_u8()?;
        visitor.visit_u8(value)
    }

    #[inline]
    fn deserialize_u16<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        let value = self.reader.read_u16::<LittleEndian>()?;
        visitor.visit_u16(value)
    }

    #[inline]
    fn deserialize_u32<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        let value = self.reader.read_u32::<LittleEndian>()?;
        visitor.visit_u32(value)
    }

    #[inline]
    fn deserialize_u64<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        let value = self.reader.read_u64::<LittleEndian>()?;
        visitor.visit_u64(value)
    }

    #[inline]
    fn deserialize_i8<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        let value = self.reader.read_i8()?;
        visitor.visit_i8(value)
    }

    #[inline]
    fn deserialize_i16<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        let value = self.reader.read_i16::<LittleEndian>()?;
        visitor.visit_i16(value)
    }

    #[inline]
    fn deserialize_i32<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        let value = self.reader.read_i32::<LittleEndian>()?;
        visitor.visit_i32(value)
    }

    #[inline]
    fn deserialize_i64<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        let value = self.reader.read_i64::<LittleEndian>()?;
        visitor.visit_i64(value)
    }

    #[inline]
    fn deserialize_f32<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        let value = self.reader.read_f32::<LittleEndian>()?;
        visitor.visit_f32(value)
    }

    #[inline]
    fn deserialize_f64<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        let value = self.reader.read_f64::<LittleEndian>()?;
        visitor.visit_f64(value)
    }

    #[inline]
    fn deserialize_char<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        // NOTE: Telegram has no representation for this.
        Err(de::Error::custom("Telegram does not support Deserializer::deserialize_char"))
    }

    #[inline]
    fn deserialize_str<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_string<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_bytes<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        // NOTE: Telegram has no representation for this.
        Err(de::Error::custom("Telegram does not support Deserializer::deserialize_bytes"))
    }

    #[inline]
    fn deserialize_byte_buf<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        // NOTE: Telegram has no representation for this.
        Err(de::Error::custom("Telegram does not support Deserializer::deserialize_byte_buf"))
    }

    #[inline]
    fn deserialize_option<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        // NOTE: Telegram has no representation for this.
        Err(de::Error::custom("Telegram does not support Deserializer::deserialize_option"))
    }

    #[inline]
    fn deserialize_unit<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        // NOTE: Telegram has no representation for this.
        Err(de::Error::custom("Telegram does not support Deserializer::deserialize_unit"))
    }

    #[inline]
    fn deserialize_unit_struct<V: de::Visitor>(self,
                                               name: &'static str,
                                               visitor: V)
                                               -> Result<V::Value> {
        // NOTE: Telegram has no representation for this.
        Err(de::Error::custom("Telegram does not support Deserializer::deserialize_unit_struct"))
    }

    #[inline]
    fn deserialize_newtype_struct<V: de::Visitor>(self,
                                                  name: &'static str,
                                                  visitor: V)
                                                  -> Result<V::Value> {
        // NOTE: Telegram has no representation for this.
        Err(de::Error::custom("Telegram does not support Deserializer::deserialize_newtype_struct"))
    }

    #[inline]
    fn deserialize_seq<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_seq_fixed_size<V: de::Visitor>(self,
                                                  len: usize,
                                                  visitor: V)
                                                  -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_tuple<V: de::Visitor>(self, len: usize, visitor: V) -> Result<V::Value> {
        // NOTE: Telegram has no representation for this.
        Err(de::Error::custom("Telegram does not support Deserializer::deserialize_tuple"))
    }

    #[inline]
    fn deserialize_tuple_struct<V: de::Visitor>(self,
                                                name: &'static str,
                                                len: usize,
                                                visitor: V)
                                                -> Result<V::Value> {
        // NOTE: Telegram has no representation for this.
        Err(de::Error::custom("Telegram does not support Deserializer::deserialize_tuple_struct"))
    }

    #[inline]
    fn deserialize_map<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        // NOTE: Telegram has no representation for this.
        Err(de::Error::custom("Telegram does not support Deserializer::deserialize_map"))
    }

    #[inline]
    fn deserialize_struct<V: de::Visitor>(self,
                                          name: &'static str,
                                          fields: &'static [&'static str],
                                          visitor: V)
                                          -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_struct_field<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_enum<V: de::Visitor>(self,
                                        name: &'static str,
                                        variants: &'static [&'static str],
                                        visitor: V)
                                        -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_ignored_any<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        // NOTE: Telegram has no representation for this.
        Err(de::Error::custom("Telegram does not support Deserializer::deserialize_ignored_any"))
    }
}
