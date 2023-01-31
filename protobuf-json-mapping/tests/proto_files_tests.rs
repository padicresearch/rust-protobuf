use prost::Message;
use protobuf::reflect::FileDescriptor;
use protobuf_json_mapping::Command;
use protobuf_json_mapping::CommandError;
use protobuf_json_mapping::ParseOptions;

#[derive(prost::Message, Clone)]
struct ProtoMessage {
    #[prost(bytes, tag = 1)]
    payload: Vec<u8>,
    #[prost(string, tag = 2)]
    name: String,
}

pub fn handle_command(cmd: &Command) -> Result<Vec<u8>, CommandError> {
    match cmd.op {
        "hex" => {
            hex::decode(cmd.data).map_err(|e| CommandError::FailedToParseNom(format!("{}", e)))
        }
        _ => Err(CommandError::FailedToParse),
    }
}

#[test]
fn test_proto_json_command_string() {
    let root = env!("CARGO_MANIFEST_DIR");
    let files = protobuf_parse::Parser::new()
        .include(format!("{root}/tests"))
        .input(format!("{root}/tests/data.proto"))
        .file_descriptor_set()
        .unwrap();

    let opts = ParseOptions {
        ignore_unknown_fields: false,
        handler: &handle_command,
        _future_options: (),
    };

    let hex_string = "c0ffee254729296a45a3885639AC7E10F9d54979";
    let hex_string_decoded = hex::decode(hex_string).unwrap();

    let proto_file = files.file[0].clone();
    let file = FileDescriptor::new_dynamic(proto_file, &[]).unwrap();
    let call_message = file.message_by_package_relative_name("Message").unwrap();
    let m = protobuf_json_mapping::parse_dyn_from_str_with_options(
        &call_message,
        r#"{ "payload" : "$hex(c0ffee254729296a45a3885639AC7E10F9d54979)", "sender" : "mambisi" }"#,
        &opts,
    )
    .unwrap();
    let mut out = Vec::new();
    m.write_to_vec_dyn(&mut out).unwrap();
    let message_out = ProtoMessage::decode(out.as_slice()).unwrap();
    assert_eq!(message_out.payload, hex_string_decoded);
}
