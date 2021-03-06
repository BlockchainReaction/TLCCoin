use asn1::*;
use super::common::*;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Record {
    served_msisdn: u64,
}

named!(pub record<Record>,
    do_parse!(
        rec: value!(Record::default())
    >>  rec: apply!(parse, rec)
    >>  eof!()
    >>  (rec)
    )
);

named_args!(parse (rec: Record) <Record>,
    do_parse!(
        rec: apply!(subrec_1, rec)
    >>  rec: apply!(subrec_2, rec)
    >>  rec: apply!(subrec_3, rec)
    >>  rec: apply!(subrec_4, rec)
    >>  rec: apply!(subrec_5, rec)
    >>  rec: apply!(subrec_6, rec)
    >>  (rec)
    )
);

named_args!(subrec_1 (rec: Record) <Record>,
    do_parse!(
        _record_type: tlv!(0x80 => ignore)
    >>  _service_centre: tlv!(0x81 => ignore)
    >>  _served_imsi: tlv!(0x82 => ignore)
    >>  _served_imei: opt!(tlv!(0x83 => ignore))
    >>  rec: field!(0x84 => address => rec.served_msisdn)
    >>  _ms_classmark: tlv!(0x85 => ignore)
    >>  _recording_entity: tlv!(0x86 => ignore)
    >>  _location: tlv!(0xA7 => ignore)
    >>  (rec)
    )
);

named_args!(subrec_2 (rec: Record) <Record>,
    do_parse!(
        _delivery_time: tlv!(0x88 => ignore)
    >>  _sms_result: opt!(tlv!(0xA9 => ignore))
    >>  _recording_extensions: opt!(tlv!(0xAA => ignore))
    >>  _system_type: tlv!(0x8B => ignore)
    >>  _location_extension: opt!(tlv!(0x8D => ignore))
    >>  _camel_sms_information: opt!(tlv!(0xAC => ignore))
    >>  _basic_service: tlv!(0xBF, 0x81, 0x02 => ignore)
    >>  _additional_chg_info: tlv!(0xBF, 0x81, 0x05 => ignore)
    >>  (rec)
    )
);

named_args!(subrec_3 (rec: Record) <Record>,
    do_parse!(
        _classmark_3: opt!(tlv!(0x9F, 0x81, 0x0C => ignore))
    >>  _charged_party: tlv!(0x9F, 0x81, 0x0D => ignore)
    >>  _charge_area_code: opt!(tlv!(0x9F, 0x81, 0x11 => ignore))
    >>  _org_rnc_or_bsc_id: tlv!(0x9F, 0x81, 0x27 => ignore)
    >>  _org_msc_id: tlv!(0x9F, 0x81, 0x28 => ignore)
    >>  _global_area_id: tlv!(0x9F, 0x81, 0x3C => ignore)
    >>  _subscriber_category: tlv!(0x9F, 0x81, 0x3E => ignore)
    >>  _first_mnn_mnc: tlv!(0x9F, 0x81, 0x40 => ignore)
    >>  (rec)
    )
);

named_args!(subrec_4 (rec: Record) <Record>,
    do_parse!(
        _sms_user_data_type: tlv!(0x9F, 0x81, 0x43 => ignore)
    >>  _sms_text: opt!(tlv!(0x9F, 0x81, 0x44 => ignore))
    >>  _maximum_number_of_sms_in_the_concatenated_sms: opt!(tlv!(0x9F, 0x81, 0x45 => ignore))
    >>  _concatenated_sms_reference_number: opt!(tlv!(0x9F, 0x81, 0x46 => ignore))
    >>  _sequence_number_of_the_current_sms: opt!(tlv!(0x9F, 0x81, 0x47 => ignore))
    >>  _hot_billing_tag: opt!(tlv!(0x9F, 0x81, 0x48 => ignore))
    >>  _origination: opt!(tlv!(0x9F, 0x81, 0x49 => ignore))
    >>  _call_reference: tlv!(0x9F, 0x81, 0x4A => ignore)
    >>  (rec)
    )
);

named_args!(subrec_5 (rec: Record) <Record>,
    do_parse!(
        _tariff_code: opt!(tlv!(0x9F, 0x81, 0x4B => ignore))
    >>  _network_operator_id: opt!(tlv!(0x9F, 0x81, 0x5F => ignore))
    >>  _type_of_subscribers: opt!(tlv!(0x9F, 0x81, 0x60 => ignore))
    >>  _record_number: opt!(tlv!(0x9F, 0x81, 0x68 => ignore))
    >>  _charge_level: opt!(tlv!(0x9F, 0x81, 0x6D => ignore))
    >>  _zone_code: opt!(tlv!(0x9F, 0x81, 0x70 => ignore))
    >>  _routing_category: opt!(tlv!(0x9F, 0x82, 0x01 => ignore))
    >>  _vobb_user_flag: opt!(tlv!(0x9F, 0x82, 0x15 => ignore))
    >>  (rec)
    )
);

named_args!(subrec_6 (rec: Record) <Record>,
    do_parse!(
        _office_name: opt!(tlv!(0x9F, 0x82, 0x1D => ignore))
    >>  _msc_type: opt!(tlv!(0x9F, 0x82, 0x1E => ignore))
    >>  _sms_type: opt!(tlv!(0x9F, 0x82, 0x1F => ignore))
    >>  _switch_mode: opt!(tlv!(0x9F, 0x82, 0x22 => ignore))
    >>  _additional_routing_category: opt!(tlv!(0x9F, 0x82, 0x3D => ignore))
    >>  (rec)
    )
);
