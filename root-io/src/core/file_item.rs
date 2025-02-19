use nom::multi::length_value;
use nom_supreme::ParserExt;

use crate::core::compression::decompress;
use crate::core::ReadError;
use crate::core::{checked_byte_count, wrap_parser, Context, Source, TKeyHeader};
use crate::tree_reader::{ttree, Tree};

/// Describes a single item within this file (e.g. a `Tree`)
#[derive(Debug)]
pub struct FileItem {
    source: Source,
    tkey_hdr: TKeyHeader,
}

impl FileItem {
    /// New file item from the information in a TKeyHeader and the associated file
    pub(crate) fn new(tkey_hdr: &TKeyHeader, source: Source) -> FileItem {
        FileItem {
            source,
            tkey_hdr: tkey_hdr.to_owned(),
        }
    }

    /// Information about this file item in Human readable form
    pub fn verbose_info(&self) -> String {
        format!("{:#?}", self.tkey_hdr)
    }
    pub fn name(&self) -> String {
        format!(
            "`{}` of type `{}`",
            self.tkey_hdr.obj_name, self.tkey_hdr.class_name
        )
    }

    async fn get_buffer(&self) -> Result<Vec<u8>, ReadError> {
        let start = self.tkey_hdr.seek_key + self.tkey_hdr.key_len as u64;
        let len = self.tkey_hdr.total_size - self.tkey_hdr.key_len as u32;
        let comp_buf = self.source.fetch(start, len as u64).await?;

        let buf = if self.tkey_hdr.total_size < self.tkey_hdr.uncomp_len {
            // Decompress the read buffer; buf is Vec<u8>
            decompress(comp_buf.as_slice())?
        } else {
            comp_buf
        };
        Ok(buf)
    }

    pub(crate) async fn get_context<'s>(&self) -> Result<Context, ReadError> {
        let buffer = self.get_buffer().await?;
        let k_map_offset = 2;
        Ok(Context {
            source: self.source.clone(),
            offset: (self.tkey_hdr.key_len + k_map_offset) as u64,
            s: buffer,
        })
    }

    /// Parse this `FileItem` as a `Tree`
    pub async fn as_tree(&self) -> Result<Tree, ReadError> {
        let ctx = self.get_context().await?;
        let buf = ctx.s.as_slice();

        let res = wrap_parser(
            length_value(checked_byte_count, ttree(&ctx))
                .complete()
                .all_consuming()
                .context("ttree wrapper"),
        )(buf)?;
        Ok(res)
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use std::path::Path;

    use crate::core::RootFile;
    use crate::core::UnwrapPrint;

    #[tokio::test]
    async fn open_simple() {
        let path = Path::new("./src/test_data/simple.root");
        let f = RootFile::new(path).await.unwrap_print();
        assert_eq!(f.items().len(), 1);
        assert_eq!(f.items()[0].tkey_hdr.obj_name, "tree");
        assert_eq!(f.streamer_infos().await.unwrap_print().len(), 18);
    }

    #[tokio::test]
    #[cfg(not(target_arch = "wasm32"))]
    async fn open_esd() {
        use alice_open_data;
        let path = alice_open_data::test_file().unwrap();

        let f = RootFile::new(path.as_path()).await.unwrap_print();

        assert_eq!(f.items().len(), 2);
        assert_eq!(f.items()[0].tkey_hdr.obj_name, "esdTree");
        assert_eq!(f.items()[1].tkey_hdr.obj_name, "HLTesdTree");
        assert_eq!(f.streamer_infos().await.unwrap_print().len(), 87);
    }
}
