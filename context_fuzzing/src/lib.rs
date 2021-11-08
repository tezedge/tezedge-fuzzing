use tezos_context_api::{ContextKvStoreConfiguration, TezosContextTezEdgeStorageConfiguration};
use tezos_context::initializer::initialize_tezedge_context;
use tezos_context::working_tree::working_tree::FoldDepth;
use tezos_context::working_tree::working_tree::WorkingTree;
use tezos_context::ProtocolContextApi;
use tezos_context::TezedgeContext;

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

pub fn context_api_fuzz() -> impl FnMut(&[u8]) {
    let context = get_init_context();
    //let context = init().unwrap(); // shoudn't crash
    let key = ["a", "b", "c"];
    let key2 = ["e", "d", "f"];
    let mut work_tree: Option<WorkingTree> = None;
    move |data| {
        for n in 1..=data.len() {
            let _ = match data[n - 1] % 20 {
                0 => {
                    let _ = context.add(&key, &[1, 2, 3]);
                }
                1 => {
                    let _ = context.add(&key2, &[4, 5, 6]);
                }
                2 => {
                    let _ = context.delete(&key);
                }
                3 => {
                    let _ = context.delete(&key2);
                }
                4 => {
                    let _ = context.find(&key);
                }
                5 => {
                    let _ = context.find(&key2);
                }
                6 => {
                    let _ = context.mem(&key);
                }
                7 => {
                    let _ = context.mem(&key2);
                }
                8 => {
                    let _ = context.mem_tree(&key);
                }
                9 => {
                    let _ = context.mem_tree(&key2);
                }
                10 => {
                    if let Ok(tree) = context.find_tree(&key) {
                        work_tree = tree;
                    }
                }
                11 => {
                    if let Ok(tree) = context.find_tree(&key2) {
                        work_tree = tree;
                    }
                }
                12 => {
                    let _ = context.empty();
                }
                13 => {
                    let _ = context.list(Some(0), Some(2), &key);
                    let _ = context.list(Some(0), Some(0), &key);
                    let _ = context.list(Some(42), Some(2), &key);
                }
                14 => {
                    let _ = context.list(Some(0), Some(2), &key2);
                    let _ = context.list(Some(0), Some(0), &key2);
                    let _ = context.list(Some(42), Some(2), &key2);
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
                15 => {
                    // TODO - iterate over fold_iter result: TreeWalker
                    let _ = context.fold_iter(Some(FoldDepth::Eq(1)), &key);

                    let _ = context.fold_iter(Some(FoldDepth::Lt(1)), &key);

                    let _ = context.fold_iter(Some(FoldDepth::Le(1)), &key);

                    let _ = context.fold_iter(Some(FoldDepth::Gt(1)), &key);

                    let _ = context.fold_iter(Some(FoldDepth::Ge(1)), &key);
                }
                16 => {
                    // TODO - iterate over fold_iter result: TreeWalker
                    let _ = context.fold_iter(Some(FoldDepth::Eq(1)), &key2);

                    let _ = context.fold_iter(Some(FoldDepth::Lt(1)), &key2);

                    let _ = context.fold_iter(Some(FoldDepth::Le(1)), &key2);

                    let _ = context.fold_iter(Some(FoldDepth::Gt(1)), &key2);

                    let _ = context.fold_iter(Some(FoldDepth::Ge(1)), &key2);
                }

                17 => {
                    let _ = context.get_merkle_root();
                }
                18 => {
                    if work_tree.is_some() {
                        let _ = context.add_tree(&key, &work_tree.as_ref().unwrap());
                    }
                }
                19 => {
                    if work_tree.is_some() {
                        let _ = context.add_tree(&key2, &work_tree.as_ref().unwrap());
                    }
                }
                // TODO - implement add_tree
                _ => unreachable!(),
            };
        }
    }
}

use tezos_context::kv_store::in_memory::InMemory;
use tezos_context::kv_store::HashId;
use tezos_context::working_tree::serializer::{deserialize_object, serialize_object};
use tezos_context::working_tree::storage::Storage;
use tezos_context::working_tree::Object;
use tezos_timing::SerializeStats;

fn is_inline_blob_in_entry(object: &Object) -> bool {
    match object {
        Object::Blob(blob_id) => blob_id.is_inline(),
        _ => false,
    }
}

pub fn workingtree_deserialize_serialize_fuzz(data: &[u8]) {
    let fake_hash_id = HashId::new(101).unwrap(); // Never fails
    let mut stats = SerializeStats::default();
    let mut storage = Storage::new();
    let mut output = Vec::new();
    let mut repo = InMemory::try_new().expect("Fail to create InMemory repository");
    let mut batch = Vec::new();
    let mut old_objects = Vec::new();

    if let Ok(object) = deserialize_object(&data, &mut storage, &repo) {
        if is_inline_blob_in_entry(&object) {
            return;
        }

        // serialization shouldn't failed after deserialize
        let _ = serialize_object(
            &object,
            fake_hash_id,
            &mut output,
            &storage,
            &mut stats,
            &mut batch,
            &mut old_objects,
            &mut repo,
        )
        .map_err(|e| panic!("serialize_entry with entry shouldn't failed: {:#?}", e));

        let mut output2 = Vec::new();

        if let Ok(object2) = deserialize_object(&output, &mut storage, &repo) {
            if is_inline_blob_in_entry(&object2) {
                return;
            }

            // serialization shouldn't failed after deserialize
            let _ = serialize_object(
                &object2,
                fake_hash_id,
                &mut output2,
                &storage,
                &mut stats,
                &mut batch,
                &mut old_objects,
                &mut repo,
            )
            .map_err(|e| panic!("serialize_entry with object2 shouldn't failed: {:#?}", e));

            // data should be the same
            assert_eq!(output2, output);
        }
    }
}

pub fn no_fuzz_all() {
    no_fuzz::no_fuzz("context_api_fuzz", context_api_fuzz());
    no_fuzz::no_fuzz("workingtree_deserialize_serialize_fuzz", workingtree_deserialize_serialize_fuzz);
}
