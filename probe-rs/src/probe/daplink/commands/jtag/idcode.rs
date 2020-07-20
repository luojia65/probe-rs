use super::super::{Category, Request, Response, Result, Status};

#[derive(Debug)]
pub struct IdcodeRequest {
    jtag_index: u8
}

impl Request for IdcodeRequest {
    const CATEGORY: Category = Category(0x16);

    fn to_bytes(&self, buffer: &mut [u8], offset: usize) -> Result<usize> {
        buffer[offset] = self.jtag_index;
        Ok(1)
    }
}

impl IdcodeRequest {
    pub(crate) fn new(jtag_index: u8) -> IdcodeRequest {
        IdcodeRequest { jtag_index }
    }
}

#[derive(Debug)]
pub(crate) struct IdcodeResponse {
    pub status: Status,
    pub id_code: u32,
}

/*
    Ref: https://github.com/wegam/CMSIS-DAP-master_WOW/blob/c6938e304ed392a17972adcfee14a13d41f6c116/Firmware/STM32/CMSIS-DAP/DAP.c#L673

	*(response+0) =  DAP_OK;
	*(response+1) = (uint8_t)(data >>  0);
	*(response+2) = (uint8_t)(data >>  8);
	*(response+3) = (uint8_t)(data >> 16);
	*(response+4) = (uint8_t)(data >> 24);
*/

impl Response for IdcodeResponse {
    fn from_bytes(buffer: &[u8], offset: usize) -> Result<Self> {
        Ok(IdcodeResponse {
            status: Status::from_byte(buffer[offset])?,
            id_code: u32::from_le_bytes([
                buffer[offset + 1],
                buffer[offset + 2],
                buffer[offset + 3],
                buffer[offset + 4],
            ]),
        })
    }
}
