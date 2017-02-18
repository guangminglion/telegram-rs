use serde::ser::{self, Serialize};
use byteorder::{LittleEndian, WriteBytesExt};
use errors::*;

pub struct Serializer<W>
    where W: WriteBytesExt
{
    writer: W,
}

impl<W> Serializer<W>
    where W: WriteBytesExt
{
    pub fn new(writer: W) -> Serializer<W> {
        Serializer { writer: writer }
    }
}

pub struct Compound<'a, W: 'a>(&'a mut Serializer<W>) where W: WriteBytesExt;

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
    where W: WriteBytesExt
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Compound<'a, W>;
    type SerializeTuple = Compound<'a, W>;
    type SerializeTupleStruct = Compound<'a, W>;
    type SerializeTupleVariant = Compound<'a, W>;
    type SerializeMap = Compound<'a, W>;
    type SerializeStruct = Compound<'a, W>;
    type SerializeStructVariant = Compound<'a, W>;

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<()> {
        // NOTE: Telegram has no representation for this.
        Err(ser::Error::custom("Telegram does not support Serializer::serialize_bool"))
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<()> {
        Ok(self.writer.write_i8(value)?)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<()> {
        Ok(self.writer.write_i16::<LittleEndian>(value)?)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<()> {
        Ok(self.writer.write_i32::<LittleEndian>(value)?)
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<()> {
        Ok(self.writer.write_i64::<LittleEndian>(value)?)
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> Result<()> {
        Ok(self.writer.write_u8(value)?)
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> Result<()> {
        Ok(self.writer.write_u16::<LittleEndian>(value)?)
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> Result<()> {
        Ok(self.writer.write_u32::<LittleEndian>(value)?)
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> Result<()> {
        Ok(self.writer.write_u64::<LittleEndian>(value)?)
    }

    #[inline]
    fn serialize_f32(self, value: f32) -> Result<()> {
        Ok(self.writer.write_f32::<LittleEndian>(value)?)
    }

    #[inline]
    fn serialize_f64(self, value: f64) -> Result<()> {
        Ok(self.writer.write_f64::<LittleEndian>(value)?)
    }

    #[inline]
    fn serialize_char(self, value: char) -> Result<()> {
        // NOTE: Telegram has no representation for this.
        Err(ser::Error::custom("Telegram does not support Serializer::serialize_char"))
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<()> {
        let len = value.len();

        if len <= 253 {
            // If L <= 253, the serialization contains one byte with the value of L,
            // then L bytes of the string followed by 0 to 3 characters containing 0,
            // such that the overall length of the value be divisible by 4,
            // whereupon all of this is interpreted as a sequence
            // of int(L/4)+1 32-bit little-endian integers.

            self.writer.write_u8(len as u8)?;
        } else {
            // If L >= 254, the serialization contains byte 254, followed by 3
            // bytes with the string length L in little-endian order, followed by L
            // bytes of the string, further followed by 0 to 3 null padding bytes.

            self.writer.write_u8(254)?;
            self.writer.write_uint::<LittleEndian>(len as u64, 3)?;
        }

        for byte in value.as_bytes() {
            self.writer.write_u8(*byte)?;
        }

        let rem = len % 4;
        if rem > 0 {
            for _ in 0..(4 - rem) {
                self.writer.write_u8(0)?;
            }
        }

        Ok(())
    }

    #[inline]
    fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        // NOTE: Telegram has no representation for this.
        Err(ser::Error::custom("Telegram does not support Serializer::serialize_bytes"))
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        // NOTE: Telegram has no representation for this.
        Err(ser::Error::custom("Telegram does not support Serializer::serialize_none"))
    }

    #[inline]
    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<()> {
        // NOTE: Telegram has no representation for this.
        Err(ser::Error::custom("Telegram does not support Serializer::serialize_some"))
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        // NOTE: Telegram has no representation for this.
        Err(ser::Error::custom("Telegram does not support Serializer::serialize_unit"))
    }

    #[inline]
    fn serialize_unit_struct(self, name: &'static str) -> Result<()> {
        // NOTE: Telegram has no representation for this.
        unimplemented!();
    }

    #[inline]
    fn serialize_unit_variant(self,
                              name: &'static str,
                              variant_index: usize,
                              variant: &'static str)
                              -> Result<()> {
        // NOTE: Telegram has no representation for this.
        unimplemented!();
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized + Serialize>(self,
                                                       name: &'static str,
                                                       value: &T)
                                                       -> Result<()> {
        // NOTE: Telegram has no representation for this.
        unimplemented!();
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized + Serialize>(self,
                                                        name: &'static str,
                                                        variant_index: usize,
                                                        variant: &'static str,
                                                        value: &T)
                                                        -> Result<()> {
        // NOTE: Telegram has no representation for this.
        unimplemented!();
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        return Ok(Compound(self));
    }

    #[inline]
    fn serialize_seq_fixed_size(self, len: usize) -> Result<Self::SerializeSeq> {
        return Ok(Compound(self));
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        return Ok(Compound(self));
    }

    #[inline]
    fn serialize_tuple_struct(self,
                              name: &'static str,
                              len: usize)
                              -> Result<Self::SerializeTupleStruct> {
        return Ok(Compound(self));
    }

    #[inline]
    fn serialize_tuple_variant(self,
                               name: &'static str,
                               variant_index: usize,
                               variant: &'static str,
                               len: usize)
                               -> Result<Self::SerializeTupleVariant> {
        return Ok(Compound(self));
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        return Ok(Compound(self));
    }

    #[inline]
    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        return Ok(Compound(self));
    }

    #[inline]
    fn serialize_struct_variant(self,
                                name: &'static str,
                                variant_index: usize,
                                variant: &'static str,
                                len: usize)
                                -> Result<Self::SerializeStructVariant> {
        return Ok(Compound(self));
    }
}

impl<'a, W> ser::SerializeSeq for Compound<'a, W>
    where W: WriteBytesExt
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        unimplemented!();
    }

    #[inline]
    fn end(self) -> Result<()> {
        unimplemented!();
    }
}

impl<'a, W> ser::SerializeMap for Compound<'a, W>
    where W: WriteBytesExt
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<()> {
        unimplemented!();
    }

    #[inline]
    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        unimplemented!();
    }

    #[inline]
    fn end(self) -> Result<()> {
        unimplemented!();
    }
}

impl<'a, W> ser::SerializeTuple for Compound<'a, W>
    where W: WriteBytesExt
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        unimplemented!();
    }

    #[inline]
    fn end(self) -> Result<()> {
        unimplemented!();
    }
}

impl<'a, W> ser::SerializeTupleStruct for Compound<'a, W>
    where W: WriteBytesExt
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        unimplemented!();
    }

    #[inline]
    fn end(self) -> Result<()> {
        unimplemented!();
    }
}

impl<'a, W> ser::SerializeTupleVariant for Compound<'a, W>
    where W: WriteBytesExt
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        unimplemented!();
    }

    #[inline]
    fn end(self) -> Result<()> {
        unimplemented!();
    }
}

impl<'a, W> ser::SerializeStruct for Compound<'a, W>
    where W: WriteBytesExt
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized + Serialize>(&mut self, _: &'static str, value: &T) -> Result<()> {
        value.serialize(&mut *self.0)
    }

    #[inline]
    fn end(self) -> Result<()> {
        // Do nothing; there is no state
        Ok(())
    }
}

impl<'a, W> ser::SerializeStructVariant for Compound<'a, W>
    where W: WriteBytesExt
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized + Serialize>(&mut self,
                                              key: &'static str,
                                              value: &T)
                                              -> Result<()> {
        unimplemented!();
    }

    #[inline]
    fn end(self) -> Result<()> {
        unimplemented!();
    }
}
