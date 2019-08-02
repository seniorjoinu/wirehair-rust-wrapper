pub mod wirehair {
    use std::fmt::{Display, Error, Formatter};
    use std::os::raw::{c_int, c_void};
    use std::ptr::null;

    #[repr(C)]
    enum WirehairResultCode {
        // Success code
        Success = 0,
        // More data is needed to decode.  This is normal and does not indicate a failure
        NeedMore = 1,
        // Other values are failure codes:
        // A function parameter was invalid
        InvalidInput = 2,
        // Encoder needs a better dense seed
        BadDenseSeed = 3,
        // Encoder needs a better peel seed
        BadPeelSeed = 4,
        // N = ceil(messageBytes / blockBytes) is too small.
        // Try reducing block_size or use a larger message
        BadInputSmallN = 5,
        // N = ceil(messageBytes / blockBytes) is too large.
        // Try increasing block_size or use a smaller message
        BadInputLargeN = 6,
        // Not enough extra rows to solve it, must give up
        ExtraInsufficient = 7,
        // An error occurred during the request
        Error = 8,
        // Out of memory
        OOM = 9,
        // Platform is not supported yet
        UnsupportedPlatform = 10,
        Count,
        /* for asserts */
        Padding = 0x7fff_ffff,
        /* int32_t padding */
    }

    #[link(name = "wirehair")]
    extern "C" {
        fn wirehair_init_(version: c_int) -> WirehairResultCode;
        fn wirehair_encoder_create(
            reuse_codec_opt: *const c_void,
            message: *const u8,
            message_size_bytes: u64,
            block_size_bytes: u32,
        ) -> *const c_void;
        fn wirehair_encode(
            codec: *const c_void,
            block_id: u64,
            block: *mut u8,
            block_size: u32,
            block_out_bytes: &mut u32,
        ) -> WirehairResultCode;
        fn wirehair_decoder_create(
            reuse_codec_opt: *const c_void,
            message_size_bytes: u64,
            block_size_bytes: u32,
        ) -> *const c_void;
        fn wirehair_decode(
            codec: *const c_void,
            block_id: u64,
            block: *const u8,
            block_out_bytes: u32,
        ) -> WirehairResultCode;
        fn wirehair_recover(
            codec: *const c_void,
            message: *mut u8,
            message_size_bytes: u64,
        ) -> WirehairResultCode;
        fn wirehair_decoder_becomes_encoder(codec: *const c_void) -> WirehairResultCode;
        fn wirehair_free(codec: *const c_void) -> c_void;
    }

    #[derive(Debug)]
    pub enum WirehairError {
        InvalidInput,
        BadDenseSeed,
        BadPeelSeed,
        BadInputSmallN,
        BadInputLargeN,
        ExtraInsufficient,
        Error,
        OOM,
        UnsupportedPlatform,
    }

    impl Display for WirehairError {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            match *self {
                WirehairError::InvalidInput => write!(f, "A function parameter was invalid"),
                WirehairError::BadDenseSeed => write!(f, "Encoder needs a better dense seed"),
                WirehairError::BadPeelSeed => write!(f, "Encoder needs a better peel seed"),
                WirehairError::BadInputSmallN => write!(
                    f,
                    "Too less blocks! Try reducing block size or use a larger message"
                ),
                WirehairError::BadInputLargeN => write!(
                    f,
                    "Too many blocks! Try increasing block_size or use a smaller message"
                ),
                WirehairError::ExtraInsufficient => write!(
                    f,
                    "Not enough extra rows to solve it, possibly corrupted data"
                ),
                WirehairError::Error => write!(f, "Unexpected error"),
                WirehairError::OOM => write!(f, "Out of memory"),
                WirehairError::UnsupportedPlatform => write!(f, "Platform is not supported yet"),
            }
        }
    }

    #[derive(Debug)]
    pub enum WirehairResult {
        Success,
        NeedMore,
        Internal,
    }

    fn parse_wirehair_result(result: WirehairResultCode) -> Result<WirehairResult, WirehairError> {
        match result {
            WirehairResultCode::InvalidInput => Err(WirehairError::InvalidInput),
            WirehairResultCode::BadDenseSeed => Err(WirehairError::BadDenseSeed),
            WirehairResultCode::BadPeelSeed => Err(WirehairError::BadPeelSeed),
            WirehairResultCode::BadInputSmallN => Err(WirehairError::BadInputSmallN),
            WirehairResultCode::BadInputLargeN => Err(WirehairError::BadInputLargeN),
            WirehairResultCode::ExtraInsufficient => Err(WirehairError::ExtraInsufficient),
            WirehairResultCode::Error => Err(WirehairError::Error),
            WirehairResultCode::OOM => Err(WirehairError::OOM),
            WirehairResultCode::UnsupportedPlatform => Err(WirehairError::UnsupportedPlatform),
            WirehairResultCode::Success => Ok(WirehairResult::Success),
            WirehairResultCode::NeedMore => Ok(WirehairResult::NeedMore),
            _ => Ok(WirehairResult::Internal),
        }
    }

    pub fn wirehair_init() -> Result<(), WirehairError> {
        let result = unsafe { parse_wirehair_result(wirehair_init_(2)) };
        match result {
            Ok(_r) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn wirehair_decoder_to_encoder(
        decoder: WirehairDecoder,
    ) -> Result<WirehairEncoder, WirehairError> {
        let result = unsafe { wirehair_decoder_becomes_encoder(decoder.native_handler) };

        match parse_wirehair_result(result) {
            Ok(_) => Ok(WirehairEncoder {
                native_handler: decoder.native_handler,
            }),
            Err(e) => Err(e),
        }
    }

    pub struct WirehairEncoder {
        native_handler: *const c_void,
    }

    impl WirehairEncoder {
        pub fn new(
            message: &mut [u8],
            message_size_bytes: u64,
            block_size_bytes: u32,
        ) -> WirehairEncoder {
            WirehairEncoder {
                native_handler: unsafe {
                    wirehair_encoder_create(
                        null::<c_void>(),
                        message.as_ptr(),
                        message_size_bytes,
                        block_size_bytes,
                    )
                },
            }
        }

        pub fn encode(
            &self,
            block_id: u64,
            block: &mut [u8],
            block_size: u32,
            block_out_bytes: &mut u32,
        ) -> Result<WirehairResult, WirehairError> {
            let result = unsafe {
                wirehair_encode(
                    self.native_handler,
                    block_id,
                    block.as_mut_ptr(),
                    block_size,
                    block_out_bytes,
                )
            };

            parse_wirehair_result(result)
        }
    }

    impl Drop for WirehairEncoder {
        fn drop(&mut self) {
            unsafe { wirehair_free(self.native_handler) };
        }
    }

    pub struct WirehairDecoder {
        native_handler: *const c_void,
    }

    impl WirehairDecoder {
        pub fn new(message_size_bytes: u64, block_size_bytes: u32) -> WirehairDecoder {
            WirehairDecoder {
                native_handler: unsafe {
                    wirehair_decoder_create(null::<c_void>(), message_size_bytes, block_size_bytes)
                },
            }
        }

        pub fn decode(
            &self,
            block_id: u64,
            block: &[u8],
            block_out_size_bytes: u32,
        ) -> Result<WirehairResult, WirehairError> {
            let result = unsafe {
                wirehair_decode(
                    self.native_handler,
                    block_id,
                    block.as_ptr(),
                    block_out_size_bytes,
                )
            };

            parse_wirehair_result(result)
        }

        pub fn recover(
            &self,
            message: &mut [u8],
            message_size_bytes: u64,
        ) -> Result<WirehairResult, WirehairError> {
            let result = unsafe {
                wirehair_recover(
                    self.native_handler,
                    message.as_mut_ptr(),
                    message_size_bytes,
                )
            };

            parse_wirehair_result(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::wirehair::*;

    #[test]
    fn basic_flow_works() {
        assert!(wirehair_init().is_ok());

        let mut message = [0u8; 500];
        for i in 0..500 {
            message[i] = i as u8
        }

        let encoder = WirehairEncoder::new(&mut message, 500, 50);
        let decoder = WirehairDecoder::new(500, 50);

        let mut block_id = 0;

        loop {
            let mut block = [0u8; 50];
            let mut block_out_bytes: u32 = 0;
            let result = encoder.encode(block_id, &mut block, 50, &mut block_out_bytes);
            assert!(result.is_ok());

            if block_id % 5 == 0 {
                block_id += 1;
                continue;
            }

            let result = decoder.decode(block_id, &block, block_out_bytes);
            assert!(result.is_ok());

            block_id += 1;

            match result.unwrap() {
                WirehairResult::NeedMore => continue,
                WirehairResult::Success => break,
                _ => panic!(),
            }
        }

        let mut decoded_message = [0u8; 500];

        let result = decoder.recover(&mut decoded_message, 500);
        assert!(result.is_ok());

        assert!(wirehair_decoder_to_encoder(decoder).is_ok());
    }
}
