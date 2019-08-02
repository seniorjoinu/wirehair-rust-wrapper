Rust wrapper for Wirehair - O(N) Fountain Code for Large Data

### API
```rust
pub fn wirehair_init() -> Result<(), WirehairError> {}

pub fn WirehairEncoder::new(
    message: &mut [u8], 
    message_size_bytes: u64, 
    block_size_bytes: u32
) -> WirehairEncoder {}

pub fn WirehairEncoder::encode(
    &self, 
    block_id: u64, 
    block: &mut [u8], 
    block_size: u32, 
    block_out_bytes: &mut u32
) -> Result<WirehairResult, WirehairError> {}


pub fn WirehairDecoder::new(message_size_bytes: u64, block_size_bytes: u32) -> WirehairDecoder {}

pub fn WirehairDecoder::decode(
    &self, 
    block_id: u64, 
    block: &[u8], 
    block_out_size_bytes: u32
) -> Result<WirehairResult, WirehairError> {}

pub fn WirehairDecoder::recover(
    &self, 
    message: &mut [u8], 
    message_size_bytes: u64
) -> Result<WirehairResult, WirehairError> {}

pub fn wirehair_decoder_to_encoder(decoder: WirehairDecoder) -> Result<WirehairEncoder, WirehairError> {}
```
