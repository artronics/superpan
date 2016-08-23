pub enum Status {
    SUCCESS,
    LIMIT_REACHED,
    NO_BEACON,
    SCAN_IN_PROGRESS,
    COUNTER_ERROR,
    FRAME_TOO_LONG,
    UNAVAILABLE_KEY,
    UNSUPPORTED_SECURITY,
    INVALID_PARAMETER
}

struct PIB {
    macExtendedAddress: u64,
    macAssociatedPANCoord: bool,
}

impl PIB {
    pub fn new_with_default(ext_address: u64) -> PIB {
        PIB {
            macExtendedAddress: ext_address,
            macAssociatedPANCoord: false,
        }
    }
}

pub mod MLME {
    use super::Status;

    pub enum ScanType {
        ED,
        ACTIVE,
        PASSIVE,
        ORPHAN,
    }

    pub struct ScanReq {
        scan_type: ScanType,
        channels: Vec<usize>,
        duration: usize,
        channel_page: usize,
    }

    pub struct ScanRes {
        status: Status,
        scan_type: ScanType,
        channel_page: usize,
        energy_detected_list: Vec<u8>,
    }

    //    pub fn scan_req(req: ScanReq) -> ScanRes {}
}

/// Mac constants
pub mod MAC {
    pub const aBaseSlotDuration: usize = 60;
}