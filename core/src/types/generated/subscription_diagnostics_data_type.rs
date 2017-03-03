// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SubscriptionDiagnosticsDataType {
    pub session_id: NodeId,
    pub subscription_id: UInt32,
    pub priority: Byte,
    pub publishing_interval: Double,
    pub max_keep_alive_count: UInt32,
    pub max_lifetime_count: UInt32,
    pub max_notifications_per_publish: UInt32,
    pub publishing_enabled: Boolean,
    pub modify_count: UInt32,
    pub enable_count: UInt32,
    pub disable_count: UInt32,
    pub republish_request_count: UInt32,
    pub republish_message_request_count: UInt32,
    pub republish_message_count: UInt32,
    pub transfer_request_count: UInt32,
    pub transferred_to_alt_client_count: UInt32,
    pub transferred_to_same_client_count: UInt32,
    pub publish_request_count: UInt32,
    pub data_change_notifications_count: UInt32,
    pub event_notifications_count: UInt32,
    pub notifications_count: UInt32,
    pub late_publish_request_count: UInt32,
    pub current_keep_alive_count: UInt32,
    pub current_lifetime_count: UInt32,
    pub unacknowledged_message_count: UInt32,
    pub discarded_message_count: UInt32,
    pub monitored_item_count: UInt32,
    pub disabled_monitored_item_count: UInt32,
    pub monitoring_queue_overflow_count: UInt32,
    pub next_sequence_number: UInt32,
    pub event_queue_over_flow_count: UInt32,
}

impl MessageInfo for SubscriptionDiagnosticsDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::SubscriptionDiagnosticsDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<SubscriptionDiagnosticsDataType> for SubscriptionDiagnosticsDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.session_id.byte_len();
        size += self.subscription_id.byte_len();
        size += self.priority.byte_len();
        size += self.publishing_interval.byte_len();
        size += self.max_keep_alive_count.byte_len();
        size += self.max_lifetime_count.byte_len();
        size += self.max_notifications_per_publish.byte_len();
        size += self.publishing_enabled.byte_len();
        size += self.modify_count.byte_len();
        size += self.enable_count.byte_len();
        size += self.disable_count.byte_len();
        size += self.republish_request_count.byte_len();
        size += self.republish_message_request_count.byte_len();
        size += self.republish_message_count.byte_len();
        size += self.transfer_request_count.byte_len();
        size += self.transferred_to_alt_client_count.byte_len();
        size += self.transferred_to_same_client_count.byte_len();
        size += self.publish_request_count.byte_len();
        size += self.data_change_notifications_count.byte_len();
        size += self.event_notifications_count.byte_len();
        size += self.notifications_count.byte_len();
        size += self.late_publish_request_count.byte_len();
        size += self.current_keep_alive_count.byte_len();
        size += self.current_lifetime_count.byte_len();
        size += self.unacknowledged_message_count.byte_len();
        size += self.discarded_message_count.byte_len();
        size += self.monitored_item_count.byte_len();
        size += self.disabled_monitored_item_count.byte_len();
        size += self.monitoring_queue_overflow_count.byte_len();
        size += self.next_sequence_number.byte_len();
        size += self.event_queue_over_flow_count.byte_len();
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.session_id.encode(stream)?;
        size += self.subscription_id.encode(stream)?;
        size += self.priority.encode(stream)?;
        size += self.publishing_interval.encode(stream)?;
        size += self.max_keep_alive_count.encode(stream)?;
        size += self.max_lifetime_count.encode(stream)?;
        size += self.max_notifications_per_publish.encode(stream)?;
        size += self.publishing_enabled.encode(stream)?;
        size += self.modify_count.encode(stream)?;
        size += self.enable_count.encode(stream)?;
        size += self.disable_count.encode(stream)?;
        size += self.republish_request_count.encode(stream)?;
        size += self.republish_message_request_count.encode(stream)?;
        size += self.republish_message_count.encode(stream)?;
        size += self.transfer_request_count.encode(stream)?;
        size += self.transferred_to_alt_client_count.encode(stream)?;
        size += self.transferred_to_same_client_count.encode(stream)?;
        size += self.publish_request_count.encode(stream)?;
        size += self.data_change_notifications_count.encode(stream)?;
        size += self.event_notifications_count.encode(stream)?;
        size += self.notifications_count.encode(stream)?;
        size += self.late_publish_request_count.encode(stream)?;
        size += self.current_keep_alive_count.encode(stream)?;
        size += self.current_lifetime_count.encode(stream)?;
        size += self.unacknowledged_message_count.encode(stream)?;
        size += self.discarded_message_count.encode(stream)?;
        size += self.monitored_item_count.encode(stream)?;
        size += self.disabled_monitored_item_count.encode(stream)?;
        size += self.monitoring_queue_overflow_count.encode(stream)?;
        size += self.next_sequence_number.encode(stream)?;
        size += self.event_queue_over_flow_count.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let session_id = NodeId::decode(stream)?;
        let subscription_id = UInt32::decode(stream)?;
        let priority = Byte::decode(stream)?;
        let publishing_interval = Double::decode(stream)?;
        let max_keep_alive_count = UInt32::decode(stream)?;
        let max_lifetime_count = UInt32::decode(stream)?;
        let max_notifications_per_publish = UInt32::decode(stream)?;
        let publishing_enabled = Boolean::decode(stream)?;
        let modify_count = UInt32::decode(stream)?;
        let enable_count = UInt32::decode(stream)?;
        let disable_count = UInt32::decode(stream)?;
        let republish_request_count = UInt32::decode(stream)?;
        let republish_message_request_count = UInt32::decode(stream)?;
        let republish_message_count = UInt32::decode(stream)?;
        let transfer_request_count = UInt32::decode(stream)?;
        let transferred_to_alt_client_count = UInt32::decode(stream)?;
        let transferred_to_same_client_count = UInt32::decode(stream)?;
        let publish_request_count = UInt32::decode(stream)?;
        let data_change_notifications_count = UInt32::decode(stream)?;
        let event_notifications_count = UInt32::decode(stream)?;
        let notifications_count = UInt32::decode(stream)?;
        let late_publish_request_count = UInt32::decode(stream)?;
        let current_keep_alive_count = UInt32::decode(stream)?;
        let current_lifetime_count = UInt32::decode(stream)?;
        let unacknowledged_message_count = UInt32::decode(stream)?;
        let discarded_message_count = UInt32::decode(stream)?;
        let monitored_item_count = UInt32::decode(stream)?;
        let disabled_monitored_item_count = UInt32::decode(stream)?;
        let monitoring_queue_overflow_count = UInt32::decode(stream)?;
        let next_sequence_number = UInt32::decode(stream)?;
        let event_queue_over_flow_count = UInt32::decode(stream)?;
        Ok(SubscriptionDiagnosticsDataType {
            session_id: session_id,
            subscription_id: subscription_id,
            priority: priority,
            publishing_interval: publishing_interval,
            max_keep_alive_count: max_keep_alive_count,
            max_lifetime_count: max_lifetime_count,
            max_notifications_per_publish: max_notifications_per_publish,
            publishing_enabled: publishing_enabled,
            modify_count: modify_count,
            enable_count: enable_count,
            disable_count: disable_count,
            republish_request_count: republish_request_count,
            republish_message_request_count: republish_message_request_count,
            republish_message_count: republish_message_count,
            transfer_request_count: transfer_request_count,
            transferred_to_alt_client_count: transferred_to_alt_client_count,
            transferred_to_same_client_count: transferred_to_same_client_count,
            publish_request_count: publish_request_count,
            data_change_notifications_count: data_change_notifications_count,
            event_notifications_count: event_notifications_count,
            notifications_count: notifications_count,
            late_publish_request_count: late_publish_request_count,
            current_keep_alive_count: current_keep_alive_count,
            current_lifetime_count: current_lifetime_count,
            unacknowledged_message_count: unacknowledged_message_count,
            discarded_message_count: discarded_message_count,
            monitored_item_count: monitored_item_count,
            disabled_monitored_item_count: disabled_monitored_item_count,
            monitoring_queue_overflow_count: monitoring_queue_overflow_count,
            next_sequence_number: next_sequence_number,
            event_queue_over_flow_count: event_queue_over_flow_count,
        })
    }
}
