// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum MessageOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Message<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Message<'a> {
  type Inner = Message<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc ) }
  }
}

impl<'a> Message<'a> {
  pub const VT_SEQ: flatbuffers::VOffsetT = 4;
  pub const VT_PACKETS: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Message { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args MessageArgs<'args>
  ) -> flatbuffers::WIPOffset<Message<'bldr>> {
    let mut builder = MessageBuilder::new(_fbb);
    builder.add_seq(args.seq);
    if let Some(x) = args.packets { builder.add_packets(x); }
    builder.finish()
  }


  #[inline]
  pub unsafe fn seq(&self) -> u64 {
    self._tab.get::<u64>(Message::VT_SEQ, Some(0)).unwrap()
  }
  #[inline]
  pub unsafe fn packets(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Packet<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Packet>>>>(Message::VT_PACKETS, None)
  }
}

impl flatbuffers::Verifiable for Message<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u64>("seq", Self::VT_SEQ, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Packet>>>>("packets", Self::VT_PACKETS, false)?
     .finish();
    Ok(())
  }
}
pub struct MessageArgs<'a> {
    pub seq: u64,
    pub packets: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Packet<'a>>>>>,
}
impl<'a> Default for MessageArgs<'a> {
  #[inline]
  fn default() -> Self {
    MessageArgs {
      seq: 0,
      packets: None,
    }
  }
}

pub struct MessageBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MessageBuilder<'a, 'b> {
  #[inline]
  pub fn add_seq(&mut self, seq: u64) {
    self.fbb_.push_slot::<u64>(Message::VT_SEQ, seq, 0);
  }
  #[inline]
  pub fn add_packets(&mut self, packets: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Packet<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_PACKETS, packets);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MessageBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MessageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Message<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Message<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Message");
    unsafe {
      ds.field("seq", &self.seq());
      ds.field("packets", &self.packets());
      ds.finish()
    }
  }
}
pub enum PacketOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Packet<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Packet<'a> {
  type Inner = Packet<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc)}
  }
}

impl<'a> Packet<'a> {
  pub const VT_SRC: flatbuffers::VOffsetT = 4;
  pub const VT_DST: flatbuffers::VOffsetT = 6;
  pub const VT_PROTO: flatbuffers::VOffsetT = 8;
  pub const VT_LEN: flatbuffers::VOffsetT = 10;
  pub const VT_ID: flatbuffers::VOffsetT = 12;
  pub const VT_FLAGS: flatbuffers::VOffsetT = 14;
  pub const VT_OPTS: flatbuffers::VOffsetT = 16;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Packet { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args PacketArgs<'args>
  ) -> flatbuffers::WIPOffset<Packet<'bldr>> {
    let mut builder = PacketBuilder::new(_fbb);
    if let Some(x) = args.opts { builder.add_opts(x); }
    if let Some(x) = args.proto { builder.add_proto(x); }
    builder.add_dst(args.dst);
    builder.add_src(args.src);
    builder.add_flags(args.flags);
    builder.add_id(args.id);
    builder.add_len(args.len);
    builder.finish()
  }


  #[inline]
  pub unsafe fn src(&self) -> u32 {
    self._tab.get::<u32>(Packet::VT_SRC, Some(0)).unwrap()
  }
  #[inline]
  pub unsafe fn dst(&self) -> u32 {
    self._tab.get::<u32>(Packet::VT_DST, Some(0)).unwrap()
  }
  #[inline]
  pub unsafe fn proto(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Packet::VT_PROTO, None)
  }
  #[inline]
  pub unsafe fn len(&self) -> u16 {
    self._tab.get::<u16>(Packet::VT_LEN, Some(0)).unwrap()
  }
  #[inline]
  pub unsafe fn id(&self) -> u16 {
    self._tab.get::<u16>(Packet::VT_ID, Some(0)).unwrap()
  }
  #[inline]
  pub unsafe fn flags(&self) -> u16 {
    self._tab.get::<u16>(Packet::VT_FLAGS, Some(0)).unwrap()
  }
  #[inline]
  pub unsafe fn opts(&self) -> Option<&'a [u8]> {
    //self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Packet::VT_OPTS, None).map(|v| v.safe_slice())
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Packet::VT_OPTS, None).map(|v| v.bytes())
  }
}

impl flatbuffers::Verifiable for Packet<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u32>("src", Self::VT_SRC, false)?
     .visit_field::<u32>("dst", Self::VT_DST, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("proto", Self::VT_PROTO, false)?
     .visit_field::<u16>("len", Self::VT_LEN, false)?
     .visit_field::<u16>("id", Self::VT_ID, false)?
     .visit_field::<u16>("flags", Self::VT_FLAGS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>("opts", Self::VT_OPTS, false)?
     .finish();
    Ok(())
  }
}
pub struct PacketArgs<'a> {
    pub src: u32,
    pub dst: u32,
    pub proto: Option<flatbuffers::WIPOffset<&'a str>>,
    pub len: u16,
    pub id: u16,
    pub flags: u16,
    pub opts: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
}
impl<'a> Default for PacketArgs<'a> {
  #[inline]
  fn default() -> Self {
    PacketArgs {
      src: 0,
      dst: 0,
      proto: None,
      len: 0,
      id: 0,
      flags: 0,
      opts: None,
    }
  }
}

pub struct PacketBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> PacketBuilder<'a, 'b> {
  #[inline]
  pub fn add_src(&mut self, src: u32) {
    self.fbb_.push_slot::<u32>(Packet::VT_SRC, src, 0);
  }
  #[inline]
  pub fn add_dst(&mut self, dst: u32) {
    self.fbb_.push_slot::<u32>(Packet::VT_DST, dst, 0);
  }
  #[inline]
  pub fn add_proto(&mut self, proto: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Packet::VT_PROTO, proto);
  }
  #[inline]
  pub fn add_len(&mut self, len: u16) {
    self.fbb_.push_slot::<u16>(Packet::VT_LEN, len, 0);
  }
  #[inline]
  pub fn add_id(&mut self, id: u16) {
    self.fbb_.push_slot::<u16>(Packet::VT_ID, id, 0);
  }
  #[inline]
  pub fn add_flags(&mut self, flags: u16) {
    self.fbb_.push_slot::<u16>(Packet::VT_FLAGS, flags, 0);
  }
  #[inline]
  pub fn add_opts(&mut self, opts: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Packet::VT_OPTS, opts);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> PacketBuilder<'a, 'b> {
    let start = _fbb.start_table();
    PacketBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Packet<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Packet<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Packet");
    unsafe {
      ds.field("src", &self.src());
      ds.field("dst", &self.dst());
      ds.field("proto", &self.proto());
      ds.field("len", &self.len());
      ds.field("id", &self.id());
      ds.field("flags", &self.flags());
      ds.field("opts", &self.opts());
    }
    ds.finish()
  }
}
#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_message<'a>(buf: &'a [u8]) -> Message<'a> {
  unsafe { flatbuffers::root_unchecked::<Message<'a>>(buf) }
}

#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_message<'a>(buf: &'a [u8]) -> Message<'a> {
  unsafe { flatbuffers::size_prefixed_root_unchecked::<Message<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `Message`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_message_unchecked`.
pub fn root_as_message(buf: &[u8]) -> Result<Message, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<Message>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Message` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_message_unchecked`.
pub fn size_prefixed_root_as_message(buf: &[u8]) -> Result<Message, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<Message>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Message` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_message_unchecked`.
pub fn root_as_message_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Message<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<Message<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Message` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_message_unchecked`.
pub fn size_prefixed_root_as_message_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Message<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<Message<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Message and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Message`.
pub unsafe fn root_as_message_unchecked(buf: &[u8]) -> Message {
  flatbuffers::root_unchecked::<Message>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Message and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Message`.
pub unsafe fn size_prefixed_root_as_message_unchecked(buf: &[u8]) -> Message {
  flatbuffers::size_prefixed_root_unchecked::<Message>(buf)
}
#[inline]
pub fn finish_message_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Message<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_message_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Message<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
