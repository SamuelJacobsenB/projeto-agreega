use bytes::Bytes;

pub struct UploadedFile {
    pub original_name: String,
    pub content: Bytes,
}
