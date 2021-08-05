use tezos_api::ffi::{ContextKvStoreConfiguration, TezosContextTezEdgeStorageConfiguration};
use tezos_new_context::initializer::initialize_tezedge_context;
use tezos_new_context::working_tree::serializer::{deserialize, serialize_entry};
use tezos_new_context::working_tree::storage::Storage;
use tezos_new_context::working_tree::working_tree::FoldDepth;
use tezos_new_context::working_tree::working_tree::WorkingTree;
use tezos_new_context::working_tree::Entry;
use tezos_new_context::ProtocolContextApi;
use tezos_new_context::TezedgeContext;
use tezos_timing::SerializeStats;

const KEY: [&str; 3] = ["a", "b", "c"];
const KEY2: [&str; 3] = ["e", "d", "f"];

pub fn context_api_fuzz() -> impl FnMut(&[u8]) {
    let mut context = get_init_context();
    let mut tree = None;
    move |data| {
        context_api(data, &mut context, &mut tree);
    }
}

pub fn get_init_context() -> TezedgeContext {
    let context = initialize_tezedge_context(&TezosContextTezEdgeStorageConfiguration {
        backend: ContextKvStoreConfiguration::InMem,
        ipc_socket_path: None,
    });
    let res = match context {
        Ok(result) => result,
        _ => loop {},
    };
    res
}

fn context_api(data: &[u8], context: &mut TezedgeContext, work_tree: &mut Option<WorkingTree>) {
    for n in 1..=data.len() {
        let _ = match data[n - 1] % 22 {
            0 => {
                let _ = context.add(&KEY, &[1, 2, 3]);
            }
            1 => {
                let _ = context.add(&KEY2, &[4, 5, 6]);
            }
            2 => {
                let _ = context.delete(&KEY);
            }
            3 => {
                let _ = context.delete(&KEY2);
            }
            4 => {
                if let Ok(Some(tree)) = context.find_tree(&KEY) {
                    let _ = context.add_tree(&KEY2, &tree);
                }
            }
            5 => {
                if let Ok(Some(tree)) = context.find_tree(&KEY2) {
                    let _ = context.add_tree(&KEY, &tree);
                }
            }
            6 => {
                let _ = context.find(&KEY);
            }
            7 => {
                let _ = context.find(&KEY2);
            }
            8 => {
                let _ = context.mem(&KEY);
            }
            9 => {
                let _ = context.mem(&KEY2);
            }
            10 => {
                let _ = context.mem_tree(&KEY);
            }
            11 => {
                let _ = context.mem_tree(&KEY2);
            }
            12 => {
                if let Ok(tree) = context.find_tree(&KEY) {
                    *work_tree = tree;
                }
            }
            13 => {
                if let Ok(tree) = context.find_tree(&KEY2) {
                    *work_tree = tree;
                }
            }
            14 => {
                let _ = context.empty();
            }
            15 => {
                let _ = context.list(Some(0), Some(2), &KEY);
                let _ = context.list(Some(0), Some(0), &KEY);
                let _ = context.list(Some(42), Some(2), &KEY);
            }
            16 => {
                let _ = context.list(Some(0), Some(2), &KEY2);
                let _ = context.list(Some(0), Some(0), &KEY2);
                let _ = context.list(Some(42), Some(2), &KEY2);
            }

            /*
                pub enum FoldDepth {
                Eq(i64), // folds over nodes and contents of depth exactly [d].
                Lt(i64), // folds over nodes and contents of depth strictly less than [d].
                Le(i64), // folds over nodes and contents of depth less than or equal to [d].
                Gt(i64), // folds over nodes and contents of depth strictly more than [d].
                Ge(i64), // folds over nodes and contents of depth more than or equal to [d].
            }
                 */
            17 => {
                // TODO - iterate over fold_iter result: TreeWalker
                let _ = context.fold_iter(Some(FoldDepth::Eq(1)), &KEY);

                let _ = context.fold_iter(Some(FoldDepth::Lt(1)), &KEY);

                let _ = context.fold_iter(Some(FoldDepth::Le(1)), &KEY);

                let _ = context.fold_iter(Some(FoldDepth::Gt(1)), &KEY);

                let _ = context.fold_iter(Some(FoldDepth::Ge(1)), &KEY);
            }
            18 => {
                // TODO - iterate over fold_iter result: TreeWalker
                let _ = context.fold_iter(Some(FoldDepth::Eq(1)), &KEY2);

                let _ = context.fold_iter(Some(FoldDepth::Lt(1)), &KEY2);

                let _ = context.fold_iter(Some(FoldDepth::Le(1)), &KEY2);

                let _ = context.fold_iter(Some(FoldDepth::Gt(1)), &KEY2);

                let _ = context.fold_iter(Some(FoldDepth::Ge(1)), &KEY2);
            }

            19 => {
                let _ = context.get_merkle_root();
            }
            20 => {
                if work_tree.is_some() {
                    let _ = context.add_tree(&KEY, &work_tree.as_ref().unwrap());
                }
            }
            21 => {
                if work_tree.is_some() {
                    let _ = context.add_tree(&KEY2, &work_tree.as_ref().unwrap());
                }
            }
            // TODO - implement add_tree
            _ => unreachable!(),
        };
    }
}

fn is_inline_blob_in_entry(entry: &Entry) -> bool {
    match entry {
        Entry::Blob(blob_id) => blob_id.is_inline(),
        _ => false,
    }
}

pub fn workingtree_deserialize_serialize_fuzz() -> impl FnMut(&[u8]) {
    |data| {
        let mut stats = SerializeStats::default();
        let mut storage = Storage::new();
        let mut output = Vec::new();

        if let Ok(entry) = deserialize(&data, &mut storage) {
            if is_inline_blob_in_entry(&entry) {
                return;
            }

            // serialization shouldn't failed after deserialize
            let _ = serialize_entry(&entry, &mut output, &storage, &mut stats)
                .map_err(|e| panic!("serialize_entry with entry shouldn't failed: {:#?}", e));

            let mut output2 = Vec::new();

            if let Ok(entry2) = deserialize(&output, &mut storage) {
                if is_inline_blob_in_entry(&entry2) {
                    return;
                }

                // serialization shouldn't failed after deserialize
                let _ = serialize_entry(&entry2, &mut output2, &storage, &mut stats)
                    .map_err(|e| panic!("serialize_entry with entry2 shouldn't failed: {:#?}", e));

                // data should be the same
                assert_eq!(output2, output);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use no_fuzz::no_fuzz;

    #[test]
    fn test() {
        no_fuzz("context_api_fuzz", super::context_api_fuzz());
        no_fuzz(
            "workingtree_deserialize_serialize_fuzz",
            super::workingtree_deserialize_serialize_fuzz(),
        );
    }
}
