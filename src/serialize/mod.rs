use crate::packets::*;
use crate::packets::chunk_ids::*;

struct Chunk {
    id: u16,
    data: Vec<u8>,
    sub_chunks: Vec<Chunk>,
}

impl Chunk {
    fn to_buffer(self) -> Vec<u8> {
        let header = self.header().to_le_bytes();
        let data = self.data;
        let chunks = self.sub_chunks.into_iter().map(|chunk| chunk.to_buffer()).flatten().collect::<Vec<u8>>();

        vec![header, data, chunks].into_iter().flatten().collect::<Vec<u8>>()
    }

    fn header(&self) -> u32 {
        let mut header = self.id as u32;
        header |= (self.data.len() as u32) << 16;
        header |= (self.sub_chunks.is_empty() as u32) << 31;
        header
    }

    fn from_string(id: u16, string: String) -> Self {
        Self {
            id,
            data: string.into_bytes(),
            sub_chunks: vec![],
        }
    }

    fn from_u64(id: u16, data: u64) -> Self {
        Self {
            id,
            data: data.to_le_bytes().to_vec(),
            sub_chunks: vec![],
        }
    }
}

trait PsnChunk {
    fn to_chunk(self) -> Chunk;
}

impl Packet {
    pub fn to_buffer(self) -> Vec<u8> {
        match self {
            Self::Info(packet) => packet.to_chunk(),
            Self::Data(packet) => packet.to_chunk(),
        }.to_buffer()
    }
}

impl PsnChunk for InfoPacket {
    fn to_chunk(self) -> Chunk {
        Chunk {
            id: PSN_INFO_PACKET,
            data: vec![],
            sub_chunks: vec![
                self.header.to_chunk(),
                PsnChunk::from_string(PSN_INFO_SYSTEM_NAME, self.system_name),
                self.tracker_list.to_chunk(),
            ],
        }
    }
}

impl PsnChunk for PacketHeader {
    fn to_chunk(self) -> Chunk {
        Chunk {
            id: PSN_PACKET_HEADER,
            data: vec![
                self.timestamp.to_le_bytes(),
                self.version_high.to_le_bytes(),
                self.version_low.to_le_bytes(),
                self.frame_id.to_le_bytes(),
                self.frame_packet_count.to_le_bytes(),
            ].into_iter().flatten().collect(),
            sub_chunks: vec![],
        }
    }
}

impl PsnChunk for Vec<TrackerInfo> {
    fn to_chunk(self) -> Chunk {
        Chunk {
            id: PSN_INFO_TRACKER_LIST,
            data: vec![],
            sub_chunks: self.into_iter().map(|tracker| tracker.to_chunk()).collect(),
        }
    }
}

impl PsnChunk for TrackerInfo {
    fn to_chunk(self) -> Chunk {
        Chunk {
            id: self.tracker_id,
            data: vec![],
            sub_chunks: if let Some(name) = self.name {
                vec![PsnChunk::from_string(PSN_INFO_TRACKER_NAME, name)]
            } else {
                vec![]
            },
        }
    }
}

impl PsnChunk for DataPacket {
    fn to_chunk(self) -> Chunk {
        Chunk {
            id: PSN_DATA_PACKET,
            data: vec![],
            sub_chunks: vec![
                self.header.to_chunk(),
                self.tracker_list.to_chunk(),
            ],
        }
    }
}

impl PsnChunk for Vec<TrackerData> {
    fn to_chunk(self) -> Chunk {
        Chunk {
            id: PSN_DATA_TRACKER_LIST,
            data: vec![],
            sub_chunks: self.into_iter().map(|tracker| tracker.to_chunk()).collect(),
        }
    }
}

impl PsnChunk for TrackerData {
    fn to_chunk(self) -> Chunk {
        Chunk {
            id: self.tracker_id,
            data: vec![],
            sub_chunks: vec![
                self.position.to_chunk(),
                self.speed.to_chunk(),
                self.orientation.to_chunk(),
                self.status.to_chunk(),
                self.acceleration.to_chunk(),
                self.target_position.to_chunk(),
                Chunk::from_u64(PSN_DATA_TRACKER_TIMESTAMP, self.timestamp),
            ],
        }
    }
}

impl PsnChunk for TrackerPosition {
    fn to_chunk(self) -> Chunk {
        Chunk {
            id: PSN_DATA_TRACKER_POSITION,
            data: vec![
                self.x.to_le_bytes(),
                self.y.to_le_bytes(),
                self.z.to_le_bytes(),
            ].into_iter().flatten().collect(),
            sub_chunks: vec![],
        }
    }
}

impl PsnChunk for TrackerSpeed {
    fn to_chunk(self) -> Chunk {
        Chunk {
            id: PSN_DATA_TRACKER_SPEED,
            data: vec![
                self.x.to_le_bytes(),
                self.y.to_le_bytes(),
                self.z.to_le_bytes(),
            ].into_iter().flatten().collect(),
            sub_chunks: vec![],
        }
    }
}

impl PsnChunk for TrackerOrientation {
    fn to_chunk(self) -> Chunk {
        Chunk {
            id: PSN_DATA_TRACKER_ORIENTATION,
            data: vec![
                self.x.to_le_bytes(),
                self.y.to_le_bytes(),
                self.z.to_le_bytes(),
            ].into_iter().flatten().collect(),
            sub_chunks: vec![],
        }
    }
}

impl PsnChunk for TrackerStatus {
    fn to_chunk(self) -> Chunk {
        Chunk {
            id: PSN_DATA_TRACKER_STATUS,
            data: self.validity.to_le_bytes().to_vec(),
            sub_chunks: vec![],
        }
    }
}

impl PsnChunk for TrackerAcceleration {
    fn to_chunk(self) -> Chunk {
        Chunk {
            id: PSN_DATA_TRACKER_ACCELERATION,
            data: vec![
                self.x.to_le_bytes(),
                self.y.to_le_bytes(),
                self.z.to_le_bytes(),
            ].into_iter().flatten().collect(),
            sub_chunks: vec![],
        }
    }
}

impl PsnChunk for TrackerTargetPosition {
    fn to_chunk(self) -> Chunk {
        Chunk {
            id: PSN_DATA_TRACKER_TARGET_POSITION,
            data: vec![
                self.x.to_le_bytes(),
                self.y.to_le_bytes(),
                self.z.to_le_bytes(),
            ].into_iter().flatten().collect(),
            sub_chunks: vec![],
        }
    }
}
