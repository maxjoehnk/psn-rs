pub mod chunk_ids {
    pub const PSN_PACKET_HEADER: u16 = 0x0000;

    pub const PSN_INFO_PACKET: u16 = 0x6576;
    pub const PSN_INFO_SYSTEM_NAME: u16 = 0x0001;
    pub const PSN_INFO_TRACKER_LIST: u16 = 0x0002;
    pub const PSN_INFO_TRACKER_NAME: u16 = 0x0000;

    pub const PSN_DATA_PACKET: u16 = 0x6755;
    pub const PSN_DATA_TRACKER_LIST: u16 = 0x0001;
    pub const PSN_DATA_TRACKER_POS: u16 = 0x0000;
    pub const PSN_DATA_TRACKER_SPEED: u16 = 0x0001;
    pub const PSN_DATA_TRACKER_ORI: u16 = 0x0002;
    pub const PSN_DATA_TRACKER_STATUS: u16 = 0x0003;
    pub const PSN_DATA_TRACKER_ACCEL: u16 = 0x0004;
    pub const PSN_DATA_TRACKER_TRGTPOS: u16 = 0x0005;
    pub const PSN_DATA_TRACKER_TIMESTAMP: u16 = 0x0006;
}

pub enum Packet {
    Info(InfoPacket),
    Data(DataPacket),
}

pub struct InfoPacket {
    pub header: PacketHeader,
    pub system_name: String,
    pub tracker_list: Vec<TrackerInfo>,
}

pub struct PacketHeader {
    /// This is the number of microseconds elapsed since the PSN server was started to the moment the packet was sent by the server
    pub timestamp: u64,
    /// An 8-bit unsigned integer defining the high version number of the Protocol. This parameter insures that all systems using the same high version number are compatible with the sent binary packet.
    pub version_high: u8,
    /// An 8-bit unsigned integer defining the low version number of the Protocol.
    pub version_low: u8,
    pub frame_id: u8,
    pub frame_packet_count: u8
}

pub struct TrackerInfo {
    pub tracker_id: u16,
    pub name: Option<String>,
}


pub struct DataPacket {
    pub header: PacketHeader,
    pub tracker_list: Vec<TrackerData>,
}

pub struct TrackerData {
    pub tracker_id: u16,
    pub position: TrackerPosition,
    pub speed: TrackerSpeed,
    pub orientation: TrackerOrientation,
    pub status: TrackerStatus,
    pub acceleration: TrackerAcceleration,
    pub target_position: TrackerTargetPosition,
    pub timestamp: u64,
}

pub struct TrackerPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct TrackerSpeed {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct TrackerOrientation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct TrackerStatus {
    pub validity: f32,
}

pub struct TrackerAcceleration {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct TrackerTargetPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
