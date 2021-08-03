use std::mem::transmute;
use strum_macros::EnumString;

const SEPARATOR: &str = "|";
const MIN: u8 = 1;
const MAX: u8 = 5;

#[repr(u8)]
#[derive(Debug, PartialEq, EnumString, Copy, Clone)]
pub enum MessageType {
    NeedProof = MIN,
    GeneratedProof = 2,
    VerifyProof = 3,
    VerifiedProof = 4,
    IncorrectProof = MAX,
}

pub fn encode_message(message_type: &MessageType, data: &str) -> String {
    let message_number = type_to_number(message_type);

    message_number.to_string() + SEPARATOR + data
}

fn type_to_number(message_type: &MessageType) -> u8 {
    *message_type as u8
}

pub fn decode_message(message: &str) -> (Option<MessageType>, &str) {
    let split = message.split(SEPARATOR).collect::<Vec<&str>>();
    let message_number = split
        .get(0)
        .unwrap()
        .parse::<u8>()
        .expect("The first part of the message must be a number");
    let message_type = number_to_type(&message_number);
    let data = split.get(1).unwrap();

    (message_type, data)
}

fn number_to_type(number: &u8) -> Option<MessageType> {
    if MIN <= *number && *number <= MAX {
        Some(unsafe { transmute(*number) })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_message_returns_expected_info() {
        let (message_type, data) = decode_message("2|some data");
        assert_eq!(MessageType::GeneratedProof, message_type.unwrap());
        assert_eq!("some data", data);
    }

    macro_rules! decode_message_returns_none_message_type_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name(){
                    let (message_type, _) = decode_message($value);
                    assert_eq!(None, message_type);
                }
            )*
        };
    }

    decode_message_returns_none_message_type_tests! {
        decode_message_returns_none_message_type_min_minus_1: "0|",
        decode_message_returns_none_message_type_max_plus_1: "6|",
    }

    #[test]
    fn encode_message_returns_expected_message() {
        let encoded_message = encode_message(&MessageType::VerifyProof, "this is the proof data");
        assert_eq!("3|this is the proof data", encoded_message);
    }

    #[test]
    fn encode_and_decode_returns_original_message() {
        let original_message_type = MessageType::VerifyProof;
        let original_message_data = "original message data";
        let encoded_message = encode_message(&original_message_type, original_message_data);
        let (decoded_message_type, decoded_message_data) = decode_message(&(encoded_message));

        assert_eq!(original_message_type, decoded_message_type.unwrap());
        assert_eq!(original_message_data, decoded_message_data);
    }
}
