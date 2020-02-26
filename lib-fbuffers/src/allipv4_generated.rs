// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

pub enum MessageOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Message<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Message<'a> {
    type Inner = Message<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Message<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Message {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args MessageArgs<'args>) -> flatbuffers::WIPOffset<Message<'bldr>> {
      let mut builder = MessageBuilder::new(_fbb);
      builder.add_seq(args.seq);
      if let Some(x) = args.packets { builder.add_packets(x); }
      builder.finish()
    }

    pub const VT_SEQ: flatbuffers::VOffsetT = 4;
    pub const VT_PACKETS: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn seq(&self) -> u64 {
    self._tab.get::<u64>(Message::VT_SEQ, Some(0)).unwrap()
  }
  #[inline]
  pub fn packets(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Packet<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Packet<'a>>>>>(Message::VT_PACKETS, None)
  }
}

pub struct MessageArgs<'a> {
    pub seq: u64,
    pub packets: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<Packet<'a >>>>>,
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

pub enum PacketOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Packet<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Packet<'a> {
    type Inner = Packet<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Packet<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Packet {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args PacketArgs<'args>) -> flatbuffers::WIPOffset<Packet<'bldr>> {
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

    pub const VT_SRC: flatbuffers::VOffsetT = 4;
    pub const VT_DST: flatbuffers::VOffsetT = 6;
    pub const VT_PROTO: flatbuffers::VOffsetT = 8;
    pub const VT_LEN: flatbuffers::VOffsetT = 10;
    pub const VT_ID: flatbuffers::VOffsetT = 12;
    pub const VT_FLAGS: flatbuffers::VOffsetT = 14;
    pub const VT_OPTS: flatbuffers::VOffsetT = 16;

  #[inline]
  pub fn src(&self) -> u32 {
    self._tab.get::<u32>(Packet::VT_SRC, Some(0)).unwrap()
  }
  #[inline]
  pub fn dst(&self) -> u32 {
    self._tab.get::<u32>(Packet::VT_DST, Some(0)).unwrap()
  }
  #[inline]
  pub fn proto(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Packet::VT_PROTO, None)
  }
  #[inline]
  pub fn len(&self) -> u16 {
    self._tab.get::<u16>(Packet::VT_LEN, Some(0)).unwrap()
  }
  #[inline]
  pub fn id(&self) -> u16 {
    self._tab.get::<u16>(Packet::VT_ID, Some(0)).unwrap()
  }
  #[inline]
  pub fn flags(&self) -> u16 {
    self._tab.get::<u16>(Packet::VT_FLAGS, Some(0)).unwrap()
  }
  #[inline]
  pub fn opts(&self) -> Option<&'a [u8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Packet::VT_OPTS, None).map(|v| v.safe_slice())
  }
}

pub struct PacketArgs<'a> {
    pub src: u32,
    pub dst: u32,
    pub proto: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub len: u16,
    pub id: u16,
    pub flags: u16,
    pub opts: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a ,  u8>>>,
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

#[inline]
pub fn get_root_as_message<'a>(buf: &'a [u8]) -> Message<'a> {
  flatbuffers::get_root::<Message<'a>>(buf)
}

#[inline]
pub fn get_size_prefixed_root_as_message<'a>(buf: &'a [u8]) -> Message<'a> {
  flatbuffers::get_size_prefixed_root::<Message<'a>>(buf)
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
