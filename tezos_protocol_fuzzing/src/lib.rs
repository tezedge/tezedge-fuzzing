#![allow(non_snake_case)]

pub fn BinaryReader_read_Z(data: &[u8]) {
    let _ = tezos_encoding::nom::zarith(data);
}

pub fn BinaryReader_read_Mutez(data: &[u8]) {
    let _ = tezos_encoding::nom::mutez(data);
}

pub fn Protocol_get_constants_for_rpc(data: &[u8]) {
    use tezos_messages::protocol::{get_constants_for_rpc, SupportedProtocol};

    let _ = get_constants_for_rpc(&data, &SupportedProtocol::Proto001);
    let _ = get_constants_for_rpc(&data, &SupportedProtocol::Proto002);
    let _ = get_constants_for_rpc(&data, &SupportedProtocol::Proto003);
    let _ = get_constants_for_rpc(&data, &SupportedProtocol::Proto004);
    let _ = get_constants_for_rpc(&data, &SupportedProtocol::Proto005);
    let _ = get_constants_for_rpc(&data, &SupportedProtocol::Proto005_2);
    let _ = get_constants_for_rpc(&data, &SupportedProtocol::Proto006);
    let _ = get_constants_for_rpc(&data, &SupportedProtocol::Proto007);
    let _ = get_constants_for_rpc(&data, &SupportedProtocol::Proto008);
    let _ = get_constants_for_rpc(&data, &SupportedProtocol::Proto008_2);
}

no_fuzz::no_fuzz_all! {
    BinaryReader_read_Z,
    BinaryReader_read_Mutez,
    Protocol_get_constants_for_rpc,
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        super::no_fuzz_all();
    }
}
