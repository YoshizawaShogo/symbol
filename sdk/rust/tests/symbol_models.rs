#[cfg(not(feature = "private_network"))]
#[allow(unused)]
mod symbol_models_test {
    use hex::decode;
    use std::str::FromStr;
    use symbol::symbol::prelude::*;
    #[test]
    #[allow(non_snake_case)]
    fn AccountAddressRestrictionTransactionV1_account_address_restriction_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AccountAddressRestrictionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.restriction_flags = AccountRestrictionFlags::ADDRESS;
            tmp_struct.restriction_additions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap(),
                );
                tmp_vec.push(
                    UnresolvedAddress::from_str("TD2ASJ2LKL5LX66PPZ67PYQN4HIMH5SX7OCZLQI").unwrap(),
                );
                tmp_vec
            };
            tmp_struct.restriction_deletions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I").unwrap(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "C266BB54A09FDD31FE0CBAA5B67A6E3FD67145A508793BA95F558BAF24A09CCE",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("7695D855CBB6CB83D5BD08E9D76145674F805D741301883387B7101FD8CA84329BB14DBF2F0B4CD58AA84CF31AC6899D134FC38FAB0E7A76F6216ACD60914ACB").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("D0000000000000007695D855CBB6CB83D5BD08E9D76145674F805D741301883387B7101FD8CA84329BB14DBF2F0B4CD58AA84CF31AC6899D134FC38FAB0E7A76F6216ACD60914ACBC266BB54A09FDD31FE0CBAA5B67A6E3FD67145A508793BA95F558BAF24A09CCE0000000001985041E0FEEEEFFEEEEFFEE0711EE7711EE77101000201000000009841E5B8E40781CF74DABF592817DE48711D778648DEAFB298F409274B52FABBFBCF7E7DF7E20DE1D0C3F657FB8595C1989059321905F681BCF47EA33BBF5E6F8298B5440854FDED").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AccountAddressRestrictionTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountAddressRestrictionTransactionV1_account_address_restriction_single_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AccountAddressRestrictionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.restriction_flags = AccountRestrictionFlags::ADDRESS
                | AccountRestrictionFlags::OUTGOING
                | AccountRestrictionFlags::BLOCK;
            tmp_struct.restriction_additions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I").unwrap(),
                );
                tmp_vec
            };
            tmp_struct.restriction_deletions = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "E74A35FA0006629F5E46275AE33E1618BEAEB01F04C2F372EDE3B46A28DEE4A2",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("4E1E910A55162EA9D5E9B17EA6AB357290E97030969C2FAFC18BCF3D73E08827F0CC9A304088742D170E8B3CE1EC4AAAC813F0F7BB6C6BBAFAEBCAE9C23D4327").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("A0000000000000004E1E910A55162EA9D5E9B17EA6AB357290E97030969C2FAFC18BCF3D73E08827F0CC9A304088742D170E8B3CE1EC4AAAC813F0F7BB6C6BBAFAEBCAE9C23D4327E74A35FA0006629F5E46275AE33E1618BEAEB01F04C2F372EDE3B46A28DEE4A20000000001985041E0FEEEEFFEEEEFFEE0711EE7711EE77101C0010000000000989059321905F681BCF47EA33BBF5E6F8298B5440854FDED").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AccountAddressRestrictionTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountAddressRestrictionTransactionV1_account_address_restriction_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct =
                            EmbeddedAccountAddressRestrictionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.restriction_flags = AccountRestrictionFlags::ADDRESS;
                        tmp_struct.restriction_additions = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push(
                                UnresolvedAddress::from_str(
                                    "TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ",
                                )
                                .unwrap(),
                            );
                            tmp_vec.push(
                                UnresolvedAddress::from_str(
                                    "TD2ASJ2LKL5LX66PPZ67PYQN4HIMH5SX7OCZLQI",
                                )
                                .unwrap(),
                            );
                            tmp_vec
                        };
                        tmp_struct.restriction_deletions = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push(
                                UnresolvedAddress::from_str(
                                    "TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I",
                                )
                                .unwrap(),
                            );
                            tmp_vec
                        };
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "6CD4BB5CCA4914FD1B89BB6C90FECC1A0C661CDB98B5BB4E2A6EDA1D4B8D2423",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("6A1C14B723E973CC450165EFC629DCAC65F0A9B70517F27BD426BFEB9C21E33C91699BCDF34A0464DBA8D4A7237E4A4309139F2E9378BEC7B67C7EA1F92D5DC6").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "553D90AFA4B171840BCBA16DB6F82A767C98344D5F6D5F2B0B05A8902D01BD4D",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("28010000000000006A1C14B723E973CC450165EFC629DCAC65F0A9B70517F27BD426BFEB9C21E33C91699BCDF34A0464DBA8D4A7237E4A4309139F2E9378BEC7B67C7EA1F92D5DC66CD4BB5CCA4914FD1B89BB6C90FECC1A0C661CDB98B5BB4E2A6EDA1D4B8D24230000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771553D90AFA4B171840BCBA16DB6F82A767C98344D5F6D5F2B0B05A8902D01BD4D800000000000000080000000000000000000000000000000000000000000000000000000000000000000000000000000000000000198504101000201000000009841E5B8E40781CF74DABF592817DE48711D778648DEAFB298F409274B52FABBFBCF7E7DF7E20DE1D0C3F657FB8595C1989059321905F681BCF47EA33BBF5E6F8298B5440854FDED").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountAddressRestrictionTransactionV1_account_address_restriction_aggregate_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct =
                            EmbeddedAccountAddressRestrictionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.restriction_flags = AccountRestrictionFlags::ADDRESS
                            | AccountRestrictionFlags::OUTGOING
                            | AccountRestrictionFlags::BLOCK;
                        tmp_struct.restriction_additions = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push(
                                UnresolvedAddress::from_str(
                                    "TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I",
                                )
                                .unwrap(),
                            );
                            tmp_vec
                        };
                        tmp_struct.restriction_deletions = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec
                        };
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "B2801A07632A4FDD66A7B22693EA49F3432205A9B556B11EF96944EACA9CFA54",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("E9FF7CE62D0925F2AF5C7C8560B01856833B643B24FB7D850C307B9021065A9A58ADCF295D9A48AF2D59344ED8998E80607B3EB21458EE7DC7011B3869E2B4EC").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "FFBAABB2E7655A0D6388DA5736FB9BA45EF6F08DB5D2659F427467FD80D6A341",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("F800000000000000E9FF7CE62D0925F2AF5C7C8560B01856833B643B24FB7D850C307B9021065A9A58ADCF295D9A48AF2D59344ED8998E80607B3EB21458EE7DC7011B3869E2B4ECB2801A07632A4FDD66A7B22693EA49F3432205A9B556B11EF96944EACA9CFA540000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771FFBAABB2E7655A0D6388DA5736FB9BA45EF6F08DB5D2659F427467FD80D6A341500000000000000050000000000000000000000000000000000000000000000000000000000000000000000000000000000000000198504101C0010000000000989059321905F681BCF47EA33BBF5E6F8298B5440854FDED").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountKeyLinkTransactionV1_account_key_link_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AccountKeyLinkTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.linked_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.linked_public_key = PublicKey::from_str(
                "D56AA6EC560D5E3FE1A1447C6D5DEDED5B62B23692FF58FEDFA0A9F785DAD993",
            )
            .unwrap();
            tmp_struct.link_action = LinkAction::LINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "BD7986B12118AE2F8C9885D01286742D5C62BCAC7B4B0ED10C48A4497F28E537",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("F9197D11A025101D4396A3475EEBD790DC62DC63859C4FEA5DA57BE16D7BF3771AB705D063E05356AD3FBFA344425CAAB47B0831AEBB2D65A08C0C014B110C62").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("A100000000000000F9197D11A025101D4396A3475EEBD790DC62DC63859C4FEA5DA57BE16D7BF3771AB705D063E05356AD3FBFA344425CAAB47B0831AEBB2D65A08C0C014B110C62BD7986B12118AE2F8C9885D01286742D5C62BCAC7B4B0ED10C48A4497F28E5370000000001984C41E0FEEEEFFEEEEFFEE0711EE7711EE771D56AA6EC560D5E3FE1A1447C6D5DEDED5B62B23692FF58FEDFA0A9F785DAD99301").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AccountKeyLinkTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountKeyLinkTransactionV1_account_key_link_single_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AccountKeyLinkTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.linked_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.linked_public_key = PublicKey::from_str(
                "E787EDC111A3A8E3E8C94DF66755065D66A34563ED09C83755C38C24D3B7F08A",
            )
            .unwrap();
            tmp_struct.link_action = LinkAction::UNLINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "61100E44C2CB0D4287528AC588AC451D4F60FBFD51D262C01B5CB493A6D0FFDA",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("7E87B96BC9C9481B675BF1C0DB8618E4F5AFE5E95EEF7FB37231C252BA76534A1EE392CC2210350F35E7096A43003049ADC71F48556621896014227BC1DDF54F").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("A1000000000000007E87B96BC9C9481B675BF1C0DB8618E4F5AFE5E95EEF7FB37231C252BA76534A1EE392CC2210350F35E7096A43003049ADC71F48556621896014227BC1DDF54F61100E44C2CB0D4287528AC588AC451D4F60FBFD51D262C01B5CB493A6D0FFDA0000000001984C41E0FEEEEFFEEEEFFEE0711EE7711EE771E787EDC111A3A8E3E8C94DF66755065D66A34563ED09C83755C38C24D3B7F08A00").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AccountKeyLinkTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountKeyLinkTransactionV1_account_key_link_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedAccountKeyLinkTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.linked_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.linked_public_key = PublicKey::from_str(
                            "D56AA6EC560D5E3FE1A1447C6D5DEDED5B62B23692FF58FEDFA0A9F785DAD993",
                        )
                        .unwrap();
                        tmp_struct.link_action = LinkAction::LINK;
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "988C8CC841F8694F7A1F10846E1E654DA62381E67DDDF2A17F5E7B82981E1E81",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("8FF91B4D9027D09053E39059C271A3C633B6B740D0722172FB58838A1DDBE2B472D9537151EA989AF5BF183BD1DE42CC9117F466DAC0A4F3CA5C8424A7D24938").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "5E870D460A2A239AE83466240C3ED08742134705FB55A85E3536527D4EA07210",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("00010000000000008FF91B4D9027D09053E39059C271A3C633B6B740D0722172FB58838A1DDBE2B472D9537151EA989AF5BF183BD1DE42CC9117F466DAC0A4F3CA5C8424A7D24938988C8CC841F8694F7A1F10846E1E654DA62381E67DDDF2A17F5E7B82981E1E810000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE7715E870D460A2A239AE83466240C3ED08742134705FB55A85E3536527D4EA072105800000000000000510000000000000000000000000000000000000000000000000000000000000000000000000000000000000001984C41D56AA6EC560D5E3FE1A1447C6D5DEDED5B62B23692FF58FEDFA0A9F785DAD9930100000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountKeyLinkTransactionV1_account_key_link_aggregate_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedAccountKeyLinkTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.linked_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.linked_public_key = PublicKey::from_str(
                            "E787EDC111A3A8E3E8C94DF66755065D66A34563ED09C83755C38C24D3B7F08A",
                        )
                        .unwrap();
                        tmp_struct.link_action = LinkAction::UNLINK;
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "3708874579F6158FACC813D3287921004AB1EA800CB56F6B4493437ADB8088DF",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("EBB7CEC3AF0608CA6BB21826B2E5AE07BE95E49B20C98B0DB33D2DB36B09174A0D54C98855D68999DFBE81F893B6F5D496F9233730BA56B7FA8BDD7DE2DAA566").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "06C15A5AFC09E2EB3DE1B42E3B8E9674438C7D60995469ACDBED1D453F596269",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("0001000000000000EBB7CEC3AF0608CA6BB21826B2E5AE07BE95E49B20C98B0DB33D2DB36B09174A0D54C98855D68999DFBE81F893B6F5D496F9233730BA56B7FA8BDD7DE2DAA5663708874579F6158FACC813D3287921004AB1EA800CB56F6B4493437ADB8088DF0000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE77106C15A5AFC09E2EB3DE1B42E3B8E9674438C7D60995469ACDBED1D453F5962695800000000000000510000000000000000000000000000000000000000000000000000000000000000000000000000000000000001984C41E787EDC111A3A8E3E8C94DF66755065D66A34563ED09C83755C38C24D3B7F08A0000000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMetadataTransactionV1_account_metadata_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AccountMetadataTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.target_address =
                UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap();
            tmp_struct.scoped_metadata_key = 10;
            tmp_struct.value_size_delta = 10;
            tmp_struct.value = "313233424143".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "662F615041C51B4605AE4A16C74416141548EDDCE012D083172921C42E80ACB2",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("A3204BB3BDBCBFEF5BA954DAB9D6AE784A84B492AA9911B533C381BBB2BBD06A36B4F623A00CA60F7BAF93CCB46441506F469EBBAF4C18352AF548E8315F4B3D").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("AA00000000000000A3204BB3BDBCBFEF5BA954DAB9D6AE784A84B492AA9911B533C381BBB2BBD06A36B4F623A00CA60F7BAF93CCB46441506F469EBBAF4C18352AF548E8315F4B3D662F615041C51B4605AE4A16C74416141548EDDCE012D083172921C42E80ACB20000000001984441E0FEEEEFFEEEEFFEE0711EE7711EE7719841E5B8E40781CF74DABF592817DE48711D778648DEAFB20A000000000000000A000600313233424143").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AccountMetadataTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMetadataTransactionV1_account_metadata_single_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AccountMetadataTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.target_address =
                UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap();
            tmp_struct.scoped_metadata_key = 11258607;
            tmp_struct.value_size_delta = -6;
            tmp_struct.value = "313233424143".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "C9BAC0EAA76473E39107081F6A9094771ED2F616E98E53ABD30339DF422C4984",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("03B8387DAA75186536106B847E4AE26213EADCC166A70EAA20C2AF66646D9243D54413EBFA4BB0B614E0ADCAF2417EA350198A26F3DCDBB8B4DACCECC8B1D418").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("AA0000000000000003B8387DAA75186536106B847E4AE26213EADCC166A70EAA20C2AF66646D9243D54413EBFA4BB0B614E0ADCAF2417EA350198A26F3DCDBB8B4DACCECC8B1D418C9BAC0EAA76473E39107081F6A9094771ED2F616E98E53ABD30339DF422C49840000000001984441E0FEEEEFFEEEEFFEE0711EE7711EE7719841E5B8E40781CF74DABF592817DE48711D778648DEAFB2EFCAAB0000000000FAFF0600313233424143").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AccountMetadataTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMetadataTransactionV1_account_metadata_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedAccountMetadataTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.target_address =
                            UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ")
                                .unwrap();
                        tmp_struct.scoped_metadata_key = 10;
                        tmp_struct.value_size_delta = 10;
                        tmp_struct.value = "313233424143".as_bytes().to_vec();
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "D76E0D5917BB5B197D8E0AA392291D4D34589E3623D7D9C695EEC74850A707B3",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("267C52D9EC00722EFEE5696D994338270A9163F22B3248611FC0E37590BAE07B5FCFB08A075C086962A25D31B42AB283235021C6F8BE3C79EF70AE1B010D9567").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "21CD7DF1DCA82BB7DEF6F46B360EDF56376CCE4C8B80D17F22AD39D5321D052C",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("0801000000000000267C52D9EC00722EFEE5696D994338270A9163F22B3248611FC0E37590BAE07B5FCFB08A075C086962A25D31B42AB283235021C6F8BE3C79EF70AE1B010D9567D76E0D5917BB5B197D8E0AA392291D4D34589E3623D7D9C695EEC74850A707B30000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE77121CD7DF1DCA82BB7DEF6F46B360EDF56376CCE4C8B80D17F22AD39D5321D052C60000000000000005A00000000000000000000000000000000000000000000000000000000000000000000000000000000000000019844419841E5B8E40781CF74DABF592817DE48711D778648DEAFB20A000000000000000A000600313233424143000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMetadataTransactionV1_account_metadata_aggregate_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedAccountMetadataTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.target_address =
                            UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ")
                                .unwrap();
                        tmp_struct.scoped_metadata_key = 11258607;
                        tmp_struct.value_size_delta = -6;
                        tmp_struct.value = "313233424143".as_bytes().to_vec();
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "73D0F7E7ABE55E1001D73DB21C82B53A2B3F0F362CB5D6E20416B37603142569",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("A7F24315C43FD5DB5DA323E460CAA1EBE8D138E311ED26035343D24DB792E106251D97A2307CBDDAABD3F05C069C069FE25B0F131D9C53B46F76EF160360A8E8").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "5AA2E82C4CDE5674CF0EA42BB6128CF177E5135126645C2BE70956F2018A08B4",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("0801000000000000A7F24315C43FD5DB5DA323E460CAA1EBE8D138E311ED26035343D24DB792E106251D97A2307CBDDAABD3F05C069C069FE25B0F131D9C53B46F76EF160360A8E873D0F7E7ABE55E1001D73DB21C82B53A2B3F0F362CB5D6E20416B376031425690000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE7715AA2E82C4CDE5674CF0EA42BB6128CF177E5135126645C2BE70956F2018A08B460000000000000005A00000000000000000000000000000000000000000000000000000000000000000000000000000000000000019844419841E5B8E40781CF74DABF592817DE48711D778648DEAFB2EFCAAB0000000000FAFF0600313233424143000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMosaicRestrictionTransactionV1_account_mosaic_restriction_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AccountMosaicRestrictionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.restriction_flags = AccountRestrictionFlags::MOSAIC_ID;
            tmp_struct.restriction_additions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(UnresolvedMosaicId(1000));
                tmp_vec
            };
            tmp_struct.restriction_deletions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(UnresolvedMosaicId(2000));
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "8E100766561BDAEED1D75EB450C573CB8C4F4C6F48D113740EE97A12856E59CC",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("FD9028F3F1A77147A0A41A0159DC0AD8B13FDA38F7076684F769C1B0BB1CEBED212AA9D6590CE68FB976998D263A2B9C86A744215B35A2EAE02E492E4B788A74").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("9800000000000000FD9028F3F1A77147A0A41A0159DC0AD8B13FDA38F7076684F769C1B0BB1CEBED212AA9D6590CE68FB976998D263A2B9C86A744215B35A2EAE02E492E4B788A748E100766561BDAEED1D75EB450C573CB8C4F4C6F48D113740EE97A12856E59CC0000000001985042E0FEEEEFFEEEEFFEE0711EE7711EE7710200010100000000E803000000000000D007000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AccountMosaicRestrictionTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMosaicRestrictionTransactionV1_account_mosaic_restriction_single_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AccountMosaicRestrictionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.restriction_flags =
                AccountRestrictionFlags::MOSAIC_ID | AccountRestrictionFlags::BLOCK;
            tmp_struct.restriction_additions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(UnresolvedMosaicId(14624838436596993100));
                tmp_vec
            };
            tmp_struct.restriction_deletions = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "45BA575D679443214C5E3D705A32B7873B76598931192F843690BF9EFC446675",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("1EB35E0C52602BF150054BDB7B938335A7BB30311C66CEEA869F98CB8808AE214A004AFBEE92B091138C9C7969D08E7B12476C30E182644C3C2A9590BE206F7B").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("90000000000000001EB35E0C52602BF150054BDB7B938335A7BB30311C66CEEA869F98CB8808AE214A004AFBEE92B091138C9C7969D08E7B12476C30E182644C3C2A9590BE206F7B45BA575D679443214C5E3D705A32B7873B76598931192F843690BF9EFC4466750000000001985042E0FEEEEFFEEEEFFEE0711EE7711EE77102800100000000004CCCD78612DDF5CA").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AccountMosaicRestrictionTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMosaicRestrictionTransactionV1_account_mosaic_restriction_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct =
                            EmbeddedAccountMosaicRestrictionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.restriction_flags = AccountRestrictionFlags::MOSAIC_ID;
                        tmp_struct.restriction_additions = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push(UnresolvedMosaicId(1000));
                            tmp_vec
                        };
                        tmp_struct.restriction_deletions = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push(UnresolvedMosaicId(2000));
                            tmp_vec
                        };
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "969F6EA2DBFA0853910F838E4C88476847FE3359705F233C95729D89B8D716F8",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("C1BDC572211630B84D43ADFB11DA5004E42093E92CE96E144BF66E6F2A2CDDFBF5138CA52F32ED23E7D8DECDA8FFC78DFC024552CCC19D605E4F1885C74D369B").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "C9B816E2B225F39322E72150DADA9F4A8C6F46C2A429F6DF4C89776A4CA8443B",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("F000000000000000C1BDC572211630B84D43ADFB11DA5004E42093E92CE96E144BF66E6F2A2CDDFBF5138CA52F32ED23E7D8DECDA8FFC78DFC024552CCC19D605E4F1885C74D369B969F6EA2DBFA0853910F838E4C88476847FE3359705F233C95729D89B8D716F80000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771C9B816E2B225F39322E72150DADA9F4A8C6F46C2A429F6DF4C89776A4CA8443B48000000000000004800000000000000000000000000000000000000000000000000000000000000000000000000000000000000019850420200010100000000E803000000000000D007000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMosaicRestrictionTransactionV1_account_mosaic_restriction_aggregate_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct =
                            EmbeddedAccountMosaicRestrictionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.restriction_flags =
                            AccountRestrictionFlags::MOSAIC_ID | AccountRestrictionFlags::BLOCK;
                        tmp_struct.restriction_additions = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push(UnresolvedMosaicId(14624838436596993100));
                            tmp_vec
                        };
                        tmp_struct.restriction_deletions = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec
                        };
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "2A01C937F05F2E9923C6E7DFE51B3E937D989570BEB24F43A99AA96FA3FC5785",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("AFECA8EE9220F05F8EDEC66E27F698E9D3774B40FF1ED1B2501CBDE88690A901F19F8F03006F6C96083B1B0D09CC7D9CBA77E2D6A4A59E67FB7DFE105E9DE196").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "3B8D31922E345C3F457E73D6DA388FA8F09E0C157AA9E77680A4EBBC3B070562",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("E800000000000000AFECA8EE9220F05F8EDEC66E27F698E9D3774B40FF1ED1B2501CBDE88690A901F19F8F03006F6C96083B1B0D09CC7D9CBA77E2D6A4A59E67FB7DFE105E9DE1962A01C937F05F2E9923C6E7DFE51B3E937D989570BEB24F43A99AA96FA3FC57850000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE7713B8D31922E345C3F457E73D6DA388FA8F09E0C157AA9E77680A4EBBC3B070562400000000000000040000000000000000000000000000000000000000000000000000000000000000000000000000000000000000198504202800100000000004CCCD78612DDF5CA").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountOperationRestrictionTransactionV1_account_operation_restriction_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AccountOperationRestrictionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.restriction_flags =
                AccountRestrictionFlags::OUTGOING | AccountRestrictionFlags::TRANSACTION_TYPE;
            tmp_struct.restriction_additions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(TransactionType::SECRET_PROOF);
                tmp_vec
            };
            tmp_struct.restriction_deletions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(TransactionType::TRANSFER);
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "4B9B7DE062070BBD7702C1128936C24F3C2B5ACB9B65D796FCBE90C0EBA23B9C",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("DD7BC1EEFC484BB258024BF0CCEA65E49A83805A63948A60E52F0FD0349C731D1A9F4070FB21C1456FC8C265743BAE84D2D97A9EA3F9A2E4577B5A383C58642D").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("8C00000000000000DD7BC1EEFC484BB258024BF0CCEA65E49A83805A63948A60E52F0FD0349C731D1A9F4070FB21C1456FC8C265743BAE84D2D97A9EA3F9A2E4577B5A383C58642D4B9B7DE062070BBD7702C1128936C24F3C2B5ACB9B65D796FCBE90C0EBA23B9C0000000001985043E0FEEEEFFEEEEFFEE0711EE7711EE771044001010000000052425441").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AccountOperationRestrictionTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountOperationRestrictionTransactionV1_account_operation_restriction_single_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AccountOperationRestrictionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.restriction_flags = AccountRestrictionFlags::OUTGOING
                | AccountRestrictionFlags::TRANSACTION_TYPE
                | AccountRestrictionFlags::BLOCK;
            tmp_struct.restriction_additions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(TransactionType::ADDRESS_ALIAS);
                tmp_vec
            };
            tmp_struct.restriction_deletions = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "F579377D15EA7146434CF95636B3808CBB2D65C2D24CCB3634F451A03BEA5B76",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("3BB0589E7608A5BB4A6FC071A1CBD604DBCC4B34AFC46C97674C1AB287192DB41BF3BD7EB77DC7E68F310D4A62B81CB23511834E6BCB21048F4EA9883284D97E").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("8A000000000000003BB0589E7608A5BB4A6FC071A1CBD604DBCC4B34AFC46C97674C1AB287192DB41BF3BD7EB77DC7E68F310D4A62B81CB23511834E6BCB21048F4EA9883284D97EF579377D15EA7146434CF95636B3808CBB2D65C2D24CCB3634F451A03BEA5B760000000001985043E0FEEEEFFEEEEFFEE0711EE7711EE77104C00100000000004E42").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AccountOperationRestrictionTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountOperationRestrictionTransactionV1_account_operation_restriction_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct =
                            EmbeddedAccountOperationRestrictionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.restriction_flags = AccountRestrictionFlags::OUTGOING
                            | AccountRestrictionFlags::TRANSACTION_TYPE;
                        tmp_struct.restriction_additions = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push(TransactionType::SECRET_PROOF);
                            tmp_vec
                        };
                        tmp_struct.restriction_deletions = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push(TransactionType::TRANSFER);
                            tmp_vec
                        };
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "4DF6829E25ADBD4F88F1600BE811172C598D37D511338D1597D6EA18E1F96E10",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("645CCB69512162882D705DFB599DADD9AB082AE8BB59A9237C2819BF35F2F18ED5AE27881F79548003277B38BB7A46157EC56DC99F4E178C4DEF809075513970").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "C257D6202832DE1D7C1632853DA071244EAE31867DD5AEBD2E3A2232B7772D2D",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("E800000000000000645CCB69512162882D705DFB599DADD9AB082AE8BB59A9237C2819BF35F2F18ED5AE27881F79548003277B38BB7A46157EC56DC99F4E178C4DEF8090755139704DF6829E25ADBD4F88F1600BE811172C598D37D511338D1597D6EA18E1F96E100000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771C257D6202832DE1D7C1632853DA071244EAE31867DD5AEBD2E3A2232B7772D2D40000000000000003C000000000000000000000000000000000000000000000000000000000000000000000000000000000000000198504304400101000000005242544100000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountOperationRestrictionTransactionV1_account_operation_restriction_aggregate_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct =
                            EmbeddedAccountOperationRestrictionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.restriction_flags = AccountRestrictionFlags::OUTGOING
                            | AccountRestrictionFlags::TRANSACTION_TYPE
                            | AccountRestrictionFlags::BLOCK;
                        tmp_struct.restriction_additions = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push(TransactionType::ADDRESS_ALIAS);
                            tmp_vec
                        };
                        tmp_struct.restriction_deletions = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec
                        };
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "AD415D7F8D836AD64B415E239601E8D14B06E83E17BAD15E0142421FC38566B2",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("635961D27DBF6178FF952D1E98F55A09EAD1248141BC32248B29F1A7D11A6E9CE5BD011746D85D73A977046E85ADAB60547FC0FEA682E0C23286A1385B768D4C").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "85170A5F6579EC36FC651524D1F953744E635AEF2D890C3DD696C34F683A0391",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("E800000000000000635961D27DBF6178FF952D1E98F55A09EAD1248141BC32248B29F1A7D11A6E9CE5BD011746D85D73A977046E85ADAB60547FC0FEA682E0C23286A1385B768D4CAD415D7F8D836AD64B415E239601E8D14B06E83E17BAD15E0142421FC38566B20000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE77185170A5F6579EC36FC651524D1F953744E635AEF2D890C3DD696C34F683A039140000000000000003A000000000000000000000000000000000000000000000000000000000000000000000000000000000000000198504304C00100000000004E42000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AddressAliasTransactionV1_address_alias_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AddressAliasTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.namespace_id = NamespaceId(9562080086528621131);
            tmp_struct.address =
                Address::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap();
            tmp_struct.alias_action = AliasAction::LINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "6BAB8AC737FD4D1A9C1AAF75DFE489D7177ABD9DED3033DFF3F33F476DDF721A",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("A2B62B8383199C1030E1231E9BDB9FA0DA44646E7ADD17C91F9136438DF16D7C629C9B6F017DD47FC0AD066C05E2E71747C7834D188665FE2B1ACC474A27741B").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("A100000000000000A2B62B8383199C1030E1231E9BDB9FA0DA44646E7ADD17C91F9136438DF16D7C629C9B6F017DD47FC0AD066C05E2E71747C7834D188665FE2B1ACC474A27741B6BAB8AC737FD4D1A9C1AAF75DFE489D7177ABD9DED3033DFF3F33F476DDF721A0000000001984E42E0FEEEEFFEEEEFFEE0711EE7711EE7714BFA5F372D55B3849841E5B8E40781CF74DABF592817DE48711D778648DEAFB201").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AddressAliasTransactionV1::deserialize(&payload).unwrap().0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AddressAliasTransactionV1_address_alias_single_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AddressAliasTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.namespace_id = NamespaceId(9562080086528621131);
            tmp_struct.address =
                Address::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap();
            tmp_struct.alias_action = AliasAction::UNLINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "66876B4CD7DD63AD474E0512002D8285A8FB91E00EBC7D8601B3AAEC8A9DEAC6",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("128FA50612DF89B1A99A9D2624BEC9957408CCBB0149D82B7F3EB9A7EAC05EB964CE554CA36B86C3776F1B8E584AB6431EC2A1B848B7479A5CBB53049B622186").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("A100000000000000128FA50612DF89B1A99A9D2624BEC9957408CCBB0149D82B7F3EB9A7EAC05EB964CE554CA36B86C3776F1B8E584AB6431EC2A1B848B7479A5CBB53049B62218666876B4CD7DD63AD474E0512002D8285A8FB91E00EBC7D8601B3AAEC8A9DEAC60000000001984E42E0FEEEEFFEEEEFFEE0711EE7711EE7714BFA5F372D55B3849841E5B8E40781CF74DABF592817DE48711D778648DEAFB200").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AddressAliasTransactionV1::deserialize(&payload).unwrap().0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AddressAliasTransactionV1_address_alias_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedAddressAliasTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.namespace_id = NamespaceId(9562080086528621131);
                        tmp_struct.address =
                            Address::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap();
                        tmp_struct.alias_action = AliasAction::LINK;
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "E40104507B50568280B2A135AF3D97E64D0C9CA4FF0B866B280F0E6CD302EC6A",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("D746C915B445707307ED4533F414DF25E277820EB3C2305088A8798AB66041DA1224BE51AF9FDA79B7E9025DA21B14E1C81371440AE445EEAB0051564D6BAF76").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "CC226F4051790D1150EA87A77C6425DCC44CB90BB827C859F57CD2963147788F",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("0001000000000000D746C915B445707307ED4533F414DF25E277820EB3C2305088A8798AB66041DA1224BE51AF9FDA79B7E9025DA21B14E1C81371440AE445EEAB0051564D6BAF76E40104507B50568280B2A135AF3D97E64D0C9CA4FF0B866B280F0E6CD302EC6A0000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771CC226F4051790D1150EA87A77C6425DCC44CB90BB827C859F57CD2963147788F5800000000000000510000000000000000000000000000000000000000000000000000000000000000000000000000000000000001984E424BFA5F372D55B3849841E5B8E40781CF74DABF592817DE48711D778648DEAFB20100000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AddressAliasTransactionV1_address_alias_aggregate_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedAddressAliasTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.namespace_id = NamespaceId(9562080086528621131);
                        tmp_struct.address =
                            Address::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap();
                        tmp_struct.alias_action = AliasAction::UNLINK;
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "FAA41AAD459A44E4DF703BB721C2870FC350F8556703E12E30D01404983EFF65",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("F507BD152F611E07BA6433C70067B50C274AA00ED6F29C4CEB22A3120FFEC83BAFA482F990A89FBCE533E982F80C83FF7B8EE9156D3723CF8EA8E8A3EDB6267B").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "7C624B5B7854988BC31B8B7CBE48B9BD388A6247A45AB5591D4832A2ADB5C17A",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("0001000000000000F507BD152F611E07BA6433C70067B50C274AA00ED6F29C4CEB22A3120FFEC83BAFA482F990A89FBCE533E982F80C83FF7B8EE9156D3723CF8EA8E8A3EDB6267BFAA41AAD459A44E4DF703BB721C2870FC350F8556703E12E30D01404983EFF650000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE7717C624B5B7854988BC31B8B7CBE48B9BD388A6247A45AB5591D4832A2ADB5C17A5800000000000000510000000000000000000000000000000000000000000000000000000000000000000000000000000000000001984E424BFA5F372D55B3849841E5B8E40781CF74DABF592817DE48711D778648DEAFB20000000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AggregateBondedTransactionV1_aggregate_bonded_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.recipient_address =
                            UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ")
                                .unwrap();
                        tmp_struct.mosaics = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push({
                                let mut tmp_struct = UnresolvedMosaic::default();
                                tmp_struct.mosaic_id = UnresolvedMosaicId(95442763262823);
                                tmp_struct.amount = Amount(101);
                                tmp_struct
                            });
                            tmp_vec.sort_unstable();
                            tmp_vec
                        };
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedMosaicSupplyChangeTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.mosaic_id = UnresolvedMosaicId(6300565133566699913);
                        tmp_struct.action = MosaicSupplyChangeAction::INCREASE;
                        tmp_struct.delta = Amount(10);
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "DBB1761103F0BEB486E8AB19DA4759A2DA6F899CBC41C303D277B88B764DC6B7",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("B75CCB8D780D5CEF69C8D0B4F60959DD28537B54EED68588B29483D2871A6D78D988D2684EEF974D04BEDA0BFEE310A9EB4210F65F0552FC79EE1BAAA7E3228E").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push({let mut tmp_struct = Cosignature::default();tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();tmp_struct.signer_public_key = PublicKey::from_str("3F5647390EAC866E835764169ABCF13E14D2BBD0A7727C98321E8836E5CCE9F0").unwrap();tmp_struct.signature = Signature::from_str("622c0ca6cc2ec0c48776fc24bf34fb7f4912b3718457a44d41a32dfcd3dbcedd7d2aa65325ed925e86edeae6ab6ca54ed8b4c0dd090ed9db3860d295da9820ed").unwrap();tmp_struct });
                tmp_vec.push({let mut tmp_struct = Cosignature::default();tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();tmp_struct.signer_public_key = PublicKey::from_str("3D6D44EABCE15DBC515E74DCDB64B3B1EF64F56EB32BB424E7BD7696A9A237F3").unwrap();tmp_struct.signature = Signature::from_str("b3895f21837f76df15b3a6d97fd7ba1dc625011619a5542194ee4220ae34e50c510d942c2c306bc0637ecfc9d9befa907819c6477254fbad11c7a0dddc71b913").unwrap();tmp_struct });
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "61E0F8B9AB2FE3E008DCE1380FECDAF5BCFB1851247BF990771154177A0B7E78",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("2002000000000000B75CCB8D780D5CEF69C8D0B4F60959DD28537B54EED68588B29483D2871A6D78D988D2684EEF974D04BEDA0BFEE310A9EB4210F65F0552FC79EE1BAAA7E3228EDBB1761103F0BEB486E8AB19DA4759A2DA6F899CBC41C303D277B88B764DC6B70000000001984142E0FEEEEFFEEEEFFEE0711EE7711EE77161E0F8B9AB2FE3E008DCE1380FECDAF5BCFB1851247BF990771154177A0B7E78A8000000000000006000000000000000000000000000000000000000000000000000000000000000000000000000000000000000019854419841E5B8E40781CF74DABF592817DE48711D778648DEAFB20000010000000000672B0000CE5600006500000000000000410000000000000000000000000000000000000000000000000000000000000000000000000000000000000001984D428969746E9B1A70570A000000000000000100000000000000000000000000000067FA12789F80766D329C7F687C5C5F889A82F5E9C3E7996AE4FFE48C34299DE7622C0CA6CC2EC0C48776FC24BF34FB7F4912B3718457A44D41A32DFCD3DBCEDD7D2AA65325ED925E86EDEAE6AB6CA54ED8B4C0DD090ED9DB3860D295DA9820ED0000000000000000549676227A2A84F8A555F69892B49A3BE02A3B2C71E031E2E8968EBAB867C491B3895F21837F76DF15B3A6D97FD7BA1DC625011619A5542194EE4220AE34E50C510D942C2C306BC0637ECFC9D9BEFA907819C6477254FBAD11C7A0DDDC71B913").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AggregateBondedTransactionV2_aggregate_bonded_aggregate_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.recipient_address =
                            UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ")
                                .unwrap();
                        tmp_struct.mosaics = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push({
                                let mut tmp_struct = UnresolvedMosaic::default();
                                tmp_struct.mosaic_id = UnresolvedMosaicId(95442763262823);
                                tmp_struct.amount = Amount(100);
                                tmp_struct
                            });
                            tmp_vec.sort_unstable();
                            tmp_vec
                        };
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedMosaicSupplyChangeTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.mosaic_id = UnresolvedMosaicId(6300565133566699912);
                        tmp_struct.action = MosaicSupplyChangeAction::INCREASE;
                        tmp_struct.delta = Amount(10);
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "0344941A66EDD89296861CFFF7A654AE5FC0E81CE9E2D96F7948A78F234A8A04",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("3E080DCE5B32319CA6899808CA664C3961C77A85BB42B192F36394D7B46C79FE4EC2AD6DA50E38836D4ADCDD992C020137F047C1228C351B9533176AB18CE0AF").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push({let mut tmp_struct = Cosignature::default();tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();tmp_struct.signer_public_key = PublicKey::from_str("AA6BA5581E30356019E3102DBB504A3423BF9B11F8BF99969F0A17AF21091965").unwrap();tmp_struct.signature = Signature::from_str("119db71f2916e710bc2195251d422af0cb2cb378c2f0f2521907f8102912ea38ad3bed2820f6aea6656b0d89e5bda7b2533409864b8a6c961dca9d173ae39979").unwrap();tmp_struct });
                tmp_vec.push({let mut tmp_struct = Cosignature::default();tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();tmp_struct.signer_public_key = PublicKey::from_str("EAB1BD382A777CE73294D3EB69E6E2F2530BE39936D95C097610601ED00C404B").unwrap();tmp_struct.signature = Signature::from_str("f55d9667e12f30c7cec0280a51f09f02c26f28e435e1ca1617765fb792c3aaa3350bc8ecd2116b8bdd3fc26e779c2a05dd788f0e59502e92c92dada6c25c6a90").unwrap();tmp_struct });
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "3F2BE873F569828C88CD0DE37BB31C998FA0AAEB3308A1FFBF3D01CE49E8E9F7",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("20020000000000003E080DCE5B32319CA6899808CA664C3961C77A85BB42B192F36394D7B46C79FE4EC2AD6DA50E38836D4ADCDD992C020137F047C1228C351B9533176AB18CE0AF0344941A66EDD89296861CFFF7A654AE5FC0E81CE9E2D96F7948A78F234A8A040000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE7713F2BE873F569828C88CD0DE37BB31C998FA0AAEB3308A1FFBF3D01CE49E8E9F7A8000000000000006000000000000000000000000000000000000000000000000000000000000000000000000000000000000000019854419841E5B8E40781CF74DABF592817DE48711D778648DEAFB20000010000000000672B0000CE5600006400000000000000410000000000000000000000000000000000000000000000000000000000000000000000000000000000000001984D428869746E9B1A70570A0000000000000001000000000000000000000000000000BD6072E843DF052681FE12FCB825CC873C670BEC51E73F5B460043677D6B1EBB119DB71F2916E710BC2195251D422AF0CB2CB378C2F0F2521907F8102912EA38AD3BED2820F6AEA6656B0D89E5BDA7B2533409864B8A6C961DCA9D173AE399790000000000000000062F6371FD45C2ADB840D85B3E7AFCB22678365733264291705210A1661C6DC8F55D9667E12F30C7CEC0280A51F09F02C26F28E435E1CA1617765FB792C3AAA3350BC8ECD2116B8BDD3FC26E779C2A05DD788F0E59502E92C92DADA6C25C6A90").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AggregateCompleteTransactionV1_aggregate_complete_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateCompleteTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.recipient_address =
                            UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ")
                                .unwrap();
                        tmp_struct.mosaics = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.sort_unstable();
                            tmp_vec
                        };
                        tmp_struct.message = "Goodbye 👋".as_bytes().to_vec();
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.recipient_address =
                            UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I")
                                .unwrap();
                        tmp_struct.mosaics = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push({
                                let mut tmp_struct = UnresolvedMosaic::default();
                                tmp_struct.mosaic_id = UnresolvedMosaicId(95442763262823);
                                tmp_struct.amount = Amount(101);
                                tmp_struct
                            });
                            tmp_vec.push({
                                let mut tmp_struct = UnresolvedMosaic::default();
                                tmp_struct.mosaic_id = UnresolvedMosaicId(15358872602548358953);
                                tmp_struct.amount = Amount(1);
                                tmp_struct
                            });
                            tmp_vec.sort_unstable();
                            tmp_vec
                        };
                        tmp_struct.message = "D600000300504C5445000000FBAF93F7".as_bytes().to_vec();
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "33863F4E43C2CA10FF2D25F8E64D10874331C2538CBA1EFCCE49F2B396CE9FF2",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("6ABA6C43AE8B33939CF3452DBFE04525EA9628D0B254A59766AF70B5497BBD82E1859E64570D6441F31AA8BD77693581F42CA59E67B8B86D944D7EBD8D05FAC4").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push({let mut tmp_struct = Cosignature::default();tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();tmp_struct.signer_public_key = PublicKey::from_str("1072C686884AEB61E2EBC3D7111082AF105E96DE0815404D7E0AC0662186C5D7").unwrap();tmp_struct.signature = Signature::from_str("af3135255497c49c647ee1021f691d28221d47854708034f7939666a8fb8ad4560f8d734355151bb610be08c9f5d01ea977f1e6b56841ab085dca25effa4a37e").unwrap();tmp_struct });
                tmp_vec.push({let mut tmp_struct = Cosignature::default();tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();tmp_struct.signer_public_key = PublicKey::from_str("E0D7E06CBBAD69508BFC05EE6E5836AD97226595AF2896CDCABEE6C11BF1BE93").unwrap();tmp_struct.signature = Signature::from_str("e17df12fc341ead43586126508d09311a237436f3b2eec79111843b9a50bda9e22d2accafe5dfa67789a15e7be246861b218c8339adb7480f367ed354a523aba").unwrap();tmp_struct });
                tmp_vec.push({let mut tmp_struct = Cosignature::default();tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();tmp_struct.signer_public_key = PublicKey::from_str("8FBB4C8BBBAB676D0673F7FA842475204FDF5B67200DAD46FA13682130EA47E1").unwrap();tmp_struct.signature = Signature::from_str("6b1998d483fc9e20fbc420e197b4213d7f2c14f7e5fc4ba0874a88eef263fcb4f14abcb5d3e144ca61d4fae67b9d13c3c3ae6c4715c0e1faf34df7f9f8c4c9ae").unwrap();tmp_struct });
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "6655F5FCF2290442DD1B3AEBB649A49249E32EBAF259403183A9A847EA22E0B6",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("C0020000000000006ABA6C43AE8B33939CF3452DBFE04525EA9628D0B254A59766AF70B5497BBD82E1859E64570D6441F31AA8BD77693581F42CA59E67B8B86D944D7EBD8D05FAC433863F4E43C2CA10FF2D25F8E64D10874331C2538CBA1EFCCE49F2B396CE9FF20000000001984141E0FEEEEFFEEEEFFEE0711EE7711EE7716655F5FCF2290442DD1B3AEBB649A49249E32EBAF259403183A9A847EA22E0B6E0000000000000005C00000000000000000000000000000000000000000000000000000000000000000000000000000000000000019854419841E5B8E40781CF74DABF592817DE48711D778648DEAFB20C00000000000000476F6F6462796520F09F918B00000000800000000000000000000000000000000000000000000000000000000000000000000000000000000000000001985441989059321905F681BCF47EA33BBF5E6F8298B5440854FDED1000020000000000672B0000CE560000650000000000000029CF5FD941AD25D50100000000000000D600000300504C5445000000FBAF93F70000000000000000BBC9173F90BDCD2F8F83A387FC24F34F723C359FF422E519669B241CBE0945DAAF3135255497C49C647EE1021F691D28221D47854708034F7939666A8FB8AD4560F8D734355151BB610BE08C9F5D01EA977F1E6B56841AB085DCA25EFFA4A37E000000000000000058A079CE59F839E4A2A02432EF80F746314A38244FB17566222F7EF3AB6F42B8E17DF12FC341EAD43586126508D09311A237436F3B2EEC79111843B9A50BDA9E22D2ACCAFE5DFA67789A15E7BE246861B218C8339ADB7480F367ED354A523ABA000000000000000034DE86B4C43F3A8FAE4495FC0832FB156F358F7FB9AA801FB77814229C745E816B1998D483FC9E20FBC420E197B4213D7F2C14F7E5FC4BA0874A88EEF263FCB4F14ABCB5D3E144CA61D4FAE67B9D13C3C3AE6C4715C0E1FAF34DF7F9F8C4C9AE").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateCompleteTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn AggregateCompleteTransactionV2_aggregate_complete_aggregate_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateCompleteTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.recipient_address =
                            UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ")
                                .unwrap();
                        tmp_struct.mosaics = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.sort_unstable();
                            tmp_vec
                        };
                        tmp_struct.message = "Hello 👋".as_bytes().to_vec();
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.recipient_address =
                            UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I")
                                .unwrap();
                        tmp_struct.mosaics = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push({
                                let mut tmp_struct = UnresolvedMosaic::default();
                                tmp_struct.mosaic_id = UnresolvedMosaicId(95442763262823);
                                tmp_struct.amount = Amount(100);
                                tmp_struct
                            });
                            tmp_vec.push({
                                let mut tmp_struct = UnresolvedMosaic::default();
                                tmp_struct.mosaic_id = UnresolvedMosaicId(15358872602548358953);
                                tmp_struct.amount = Amount(1);
                                tmp_struct
                            });
                            tmp_vec.sort_unstable();
                            tmp_vec
                        };
                        tmp_struct.message = "D600000300504C5445000000FBAF93F7".as_bytes().to_vec();
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "631D53160E61103BF994913081B0B927508F53AB2A7A9754D9E9A1B64BD19A7B",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("BA5888AF968D00F57A9019A58421897EFC4863BEB8EF9B20C765B50B839DCFE4501FB0CD839CEF8EF43F4DD59CEB78BD8A80B011D9E8B577418C6415FFC7FA3D").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push({let mut tmp_struct = Cosignature::default();tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();tmp_struct.signer_public_key = PublicKey::from_str("0726CE31C91354336317881469E95A21B26032097980199F5E5BABDD5DE154F6").unwrap();tmp_struct.signature = Signature::from_str("7ce0e4577928b9f42f83453d13819a0e762adb37bda14ca6c7b773acb4dce3912b332087402cab4e5c52c3c8f58ff56c09af15fdf6592492aa5720852f8546e9").unwrap();tmp_struct });
                tmp_vec.push({let mut tmp_struct = Cosignature::default();tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();tmp_struct.signer_public_key = PublicKey::from_str("C2791EA2388F616F6BCD7EFCAB96172C860DC179A1A15B9CE9D402C493B99CC3").unwrap();tmp_struct.signature = Signature::from_str("cd18f8c52d0bfb1cc1c35d359ba2856dc956681e4ff1d72d2e1eeae280c1f8ccb2b8d1ce44f9760ee0985c5ff32e49e6159b7a249056d2f8549f31bd0477141f").unwrap();tmp_struct });
                tmp_vec.push({let mut tmp_struct = Cosignature::default();tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();tmp_struct.signer_public_key = PublicKey::from_str("FC7D4E4333B9D59F1020E70C75CA9E50CD1965D964D96EBF3733581D2EB3E19F").unwrap();tmp_struct.signature = Signature::from_str("5ba52f0194c2b04fbb92ef1ee80feec0cf2a4d02fc0bd39817f3b8228343797c88493416fe316460124f89eee6f32a047f59b2937c85f3f3a4b5973465fb71f4").unwrap();tmp_struct });
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "DCE7DC355A58AEDC834B89C2E3D42DD07DBB8C9167A046856CA56EBE4EEE5AC2",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("C002000000000000BA5888AF968D00F57A9019A58421897EFC4863BEB8EF9B20C765B50B839DCFE4501FB0CD839CEF8EF43F4DD59CEB78BD8A80B011D9E8B577418C6415FFC7FA3D631D53160E61103BF994913081B0B927508F53AB2A7A9754D9E9A1B64BD19A7B0000000002984141E0FEEEEFFEEEEFFEE0711EE7711EE771DCE7DC355A58AEDC834B89C2E3D42DD07DBB8C9167A046856CA56EBE4EEE5AC2E0000000000000005A00000000000000000000000000000000000000000000000000000000000000000000000000000000000000019854419841E5B8E40781CF74DABF592817DE48711D778648DEAFB20A0000000000000048656C6C6F20F09F918B000000000000800000000000000000000000000000000000000000000000000000000000000000000000000000000000000001985441989059321905F681BCF47EA33BBF5E6F8298B5440854FDED1000020000000000672B0000CE560000640000000000000029CF5FD941AD25D50100000000000000D600000300504C5445000000FBAF93F70000000000000000264E45B83FCF538B9B58CCF252BD39486A6D1B139300EFD2DB357CE4EC225CB47CE0E4577928B9F42F83453D13819A0E762ADB37BDA14CA6C7B773ACB4DCE3912B332087402CAB4E5C52C3C8F58FF56C09AF15FDF6592492AA5720852F8546E9000000000000000000A0437049F578C2C64B9BEA3E6D19BD2A5B521F8447749B2D6006B188E32A04CD18F8C52D0BFB1CC1C35D359BA2856DC956681E4FF1D72D2E1EEAE280C1F8CCB2B8D1CE44F9760EE0985C5FF32E49E6159B7A249056D2F8549F31BD0477141F0000000000000000188CB4361E1E76F98CF3E4D313F5EAA202582F2823EB8A92AEC3EF71E792090F5BA52F0194C2B04FBB92EF1EE80FEEC0CF2A4D02FC0BD39817F3B8228343797C88493416FE316460124F89EEE6F32A047F59B2937C85F3F3A4B5973465FB71F4").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateCompleteTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn HashLockTransactionV1_hash_lock_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = HashLockTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.mosaic = {
                let mut tmp_struct = UnresolvedMosaic::default();
                tmp_struct.mosaic_id = UnresolvedMosaicId(9636553580561478212);
                tmp_struct.amount = Amount(10000000);
                tmp_struct
            };
            tmp_struct.duration = BlockDuration(100);
            tmp_struct.hash = Hash256::from_str(
                "8498B38D89C1DC8A448EA5824938FF828926CD9F7747B1844B59B4B6807E878B",
            )
            .unwrap();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "DE254F0E4D31159CC5B32D0ABDAFE2FA96255BB8F5879717BC92ED9E799A5F97",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("DCE85092A4AA448260E9C849FBC5FA51CA92BF90DFD1831FBFBE44D0B7FB4973E243B0D651CD5DC0EE35EC60472C1598C0BF182B344FD80D26938E3DFF5F9491").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("B800000000000000DCE85092A4AA448260E9C849FBC5FA51CA92BF90DFD1831FBFBE44D0B7FB4973E243B0D651CD5DC0EE35EC60472C1598C0BF182B344FD80D26938E3DFF5F9491DE254F0E4D31159CC5B32D0ABDAFE2FA96255BB8F5879717BC92ED9E799A5F970000000001984841E0FEEEEFFEEEEFFEE0711EE7711EE77144B262C46CEABB85809698000000000064000000000000008498B38D89C1DC8A448EA5824938FF828926CD9F7747B1844B59B4B6807E878B").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = HashLockTransactionV1::deserialize(&payload).unwrap().0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn HashLockTransactionV1_hash_lock_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedHashLockTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.mosaic = {
                            let mut tmp_struct = UnresolvedMosaic::default();
                            tmp_struct.mosaic_id = UnresolvedMosaicId(9636553580561478212);
                            tmp_struct.amount = Amount(10000000);
                            tmp_struct
                        };
                        tmp_struct.duration = BlockDuration(100);
                        tmp_struct.hash = Hash256::from_str(
                            "8498B38D89C1DC8A448EA5824938FF828926CD9F7747B1844B59B4B6807E878B",
                        )
                        .unwrap();
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "F845CBC26FD61D1411E3D3109D53BD5490975E2E519FD6ACE13E1DFFD894D09A",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("86FA92B1514E3AE2EAE225EE402C4390B2CF4C481573501B8AC793AFA9DBEA0C4C63E7F9993E62F9F100C435315BD4B0EC5F473CA7BF1A7939454F04B6168C4A").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "F0197674A946DD65165C9E7FFD0CAA15745F2E304BB9DD41ABAF2630112592D8",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("100100000000000086FA92B1514E3AE2EAE225EE402C4390B2CF4C481573501B8AC793AFA9DBEA0C4C63E7F9993E62F9F100C435315BD4B0EC5F473CA7BF1A7939454F04B6168C4AF845CBC26FD61D1411E3D3109D53BD5490975E2E519FD6ACE13E1DFFD894D09A0000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771F0197674A946DD65165C9E7FFD0CAA15745F2E304BB9DD41ABAF2630112592D8680000000000000068000000000000000000000000000000000000000000000000000000000000000000000000000000000000000198484144B262C46CEABB85809698000000000064000000000000008498B38D89C1DC8A448EA5824938FF828926CD9F7747B1844B59B4B6807E878B").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicAddressRestrictionTransactionV1_mosaic_address_restriction_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = MosaicAddressRestrictionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.mosaic_id = UnresolvedMosaicId(1);
            tmp_struct.restriction_key = 1311768467294898927;
            tmp_struct.previous_restriction_value = 9;
            tmp_struct.new_restriction_value = 8;
            tmp_struct.target_address =
                UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "FA51BA3A392F60D62EEB84BBB8CBB5311A1749E5AF6A687F434EF79D97A324C1",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("D540747095A39055383EA6A199959BE21A43DC6324DFD215EBE2888904D6F5D6F61D259D84456DC6D731DABBCFD26C747E4A80970D56C1741D82FFE9CDB0E540").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("B800000000000000D540747095A39055383EA6A199959BE21A43DC6324DFD215EBE2888904D6F5D6F61D259D84456DC6D731DABBCFD26C747E4A80970D56C1741D82FFE9CDB0E540FA51BA3A392F60D62EEB84BBB8CBB5311A1749E5AF6A687F434EF79D97A324C10000000001985142E0FEEEEFFEEEEFFEE0711EE7711EE7710100000000000000EFCAAB9078563412090000000000000008000000000000009841E5B8E40781CF74DABF592817DE48711D778648DEAFB2").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = MosaicAddressRestrictionTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicAddressRestrictionTransactionV1_mosaic_address_restriction_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct =
                            EmbeddedMosaicAddressRestrictionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.mosaic_id = UnresolvedMosaicId(1);
                        tmp_struct.restriction_key = 1311768467294898927;
                        tmp_struct.previous_restriction_value = 9;
                        tmp_struct.new_restriction_value = 8;
                        tmp_struct.target_address =
                            UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ")
                                .unwrap();
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "FBD4920A244A03C88612F77B37198D164326739DA3C610256D68583A35D3F08B",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("422F08BBF26F3589B048C8C2079B18E90555724C99F8AE37FFC14CA5A2B2943C9759DDDD54837C6630A5A138AD96DD5BC478D78F7E677445EEBFC55EFA9E35C8").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "7B6ED24A1F78B4BEC3900FBFED34AC0D18ECD29D2EB179BD0C46291107EDDEEF",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("1001000000000000422F08BBF26F3589B048C8C2079B18E90555724C99F8AE37FFC14CA5A2B2943C9759DDDD54837C6630A5A138AD96DD5BC478D78F7E677445EEBFC55EFA9E35C8FBD4920A244A03C88612F77B37198D164326739DA3C610256D68583A35D3F08B0000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE7717B6ED24A1F78B4BEC3900FBFED34AC0D18ECD29D2EB179BD0C46291107EDDEEF68000000000000006800000000000000000000000000000000000000000000000000000000000000000000000000000000000000019851420100000000000000EFCAAB9078563412090000000000000008000000000000009841E5B8E40781CF74DABF592817DE48711D778648DEAFB2").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicAliasTransactionV1_mosaic_alias_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = MosaicAliasTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.namespace_id = NamespaceId(13182596108967839652);
            tmp_struct.mosaic_id = MosaicId(10);
            tmp_struct.alias_action = AliasAction::LINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "6A4D9F52A9C07A3782571FBB88EEFFB40E2DA1E2C765D8FE062796E255B0BDA5",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("8D5BDBFC1344DA6738E928C2547B8422D34AAB8AA80E77E9657AEC80937DA19D782F837545CFF48DD4880D08C35B7C39119B9F75F3E50DFAB0D917D4D2598BF0").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("91000000000000008D5BDBFC1344DA6738E928C2547B8422D34AAB8AA80E77E9657AEC80937DA19D782F837545CFF48DD4880D08C35B7C39119B9F75F3E50DFAB0D917D4D2598BF06A4D9F52A9C07A3782571FBB88EEFFB40E2DA1E2C765D8FE062796E255B0BDA50000000001984E43E0FEEEEFFEEEEFFEE0711EE7711EE771A487791451FDF1B60A0000000000000001").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = MosaicAliasTransactionV1::deserialize(&payload).unwrap().0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicAliasTransactionV1_mosaic_alias_single_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = MosaicAliasTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.namespace_id = NamespaceId(16233676262248077354);
            tmp_struct.mosaic_id = MosaicId(14624838436596993100);
            tmp_struct.alias_action = AliasAction::UNLINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "69F83B2313C35E758B70361E9B0C3560C5C7A182E504BDC663DBAE6500E4D106",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("6B1750C3D6F272C316EBB3916E177F2DAF2F6837CF43201A631059D02EC0FFCC554C2C64E9AB10F6B154EFE152DAAA04CCA11082DB6E81EA411E7E416A298142").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("91000000000000006B1750C3D6F272C316EBB3916E177F2DAF2F6837CF43201A631059D02EC0FFCC554C2C64E9AB10F6B154EFE152DAAA04CCA11082DB6E81EA411E7E416A29814269F83B2313C35E758B70361E9B0C3560C5C7A182E504BDC663DBAE6500E4D1060000000001984E43E0FEEEEFFEEEEFFEE0711EE7711EE7712AD8FC018D9A49E14CCCD78612DDF5CA00").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = MosaicAliasTransactionV1::deserialize(&payload).unwrap().0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicAliasTransactionV1_mosaic_alias_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedMosaicAliasTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.namespace_id = NamespaceId(13182596108967839652);
                        tmp_struct.mosaic_id = MosaicId(10);
                        tmp_struct.alias_action = AliasAction::LINK;
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "1FAC82F9AFF382D692499D106910F8EAA5A940BD3C243A51D562958E8A370927",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("F620B8DC54CA880724AAD84C1B4D260D02DE838AA661995ED90FEF2425EC29C948C0BC68D09B09B956CA0A4457ED85B26F246F6C0471D830F74B8A776438BAA8").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "2FDCAABBB776C8A409B39AB27525383DC06A271643372B03F622F886C08B44B6",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("F000000000000000F620B8DC54CA880724AAD84C1B4D260D02DE838AA661995ED90FEF2425EC29C948C0BC68D09B09B956CA0A4457ED85B26F246F6C0471D830F74B8A776438BAA81FAC82F9AFF382D692499D106910F8EAA5A940BD3C243A51D562958E8A3709270000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE7712FDCAABBB776C8A409B39AB27525383DC06A271643372B03F622F886C08B44B64800000000000000410000000000000000000000000000000000000000000000000000000000000000000000000000000000000001984E43A487791451FDF1B60A000000000000000100000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicAliasTransactionV1_mosaic_alias_aggregate_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedMosaicAliasTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.namespace_id = NamespaceId(16233676262248077354);
                        tmp_struct.mosaic_id = MosaicId(14624838436596993100);
                        tmp_struct.alias_action = AliasAction::UNLINK;
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "A4C88C079FA8ECF3A7C6FC46731E9A6ECE4D4657D3162CC7C46B39ECB694180D",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("072AD7B3441046032836E13A21FF0591FFCFCCB6B80CC99BBA4EA0291B1E13830560B3BD33E1D2368C4CAD9FFC812A7F64A6029774DDA784F25290B54059CF88").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "05270A1D4E45A3D4898353D52F890D573445F81914D96DBF5A9A7EA113564E34",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("F000000000000000072AD7B3441046032836E13A21FF0591FFCFCCB6B80CC99BBA4EA0291B1E13830560B3BD33E1D2368C4CAD9FFC812A7F64A6029774DDA784F25290B54059CF88A4C88C079FA8ECF3A7C6FC46731E9A6ECE4D4657D3162CC7C46B39ECB694180D0000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE77105270A1D4E45A3D4898353D52F890D573445F81914D96DBF5A9A7EA113564E344800000000000000410000000000000000000000000000000000000000000000000000000000000000000000000000000000000001984E432AD8FC018D9A49E14CCCD78612DDF5CA0000000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicDefinitionTransactionV1_mosaic_definition_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = MosaicDefinitionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.duration = BlockDuration(10000);
            tmp_struct.nonce = MosaicNonce(0);
            tmp_struct.flags = MosaicFlags::RESTRICTABLE | MosaicFlags::SUPPLY_MUTABLE;
            tmp_struct.divisibility = 4;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "AFE26F6B9F61B0F1227A149D482D221B4D50F0FB780BC42D2DBA3FCD1F0250B9",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("5D3116D285D4ED8883DBBBC8E59FED08A888DAB21C6E4B918434BE2B3AF1105EE1B94EAA9C4BB54428F4A71C711964F00848B9A9E00D8F55670991AADC16119F").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("96000000000000005D3116D285D4ED8883DBBBC8E59FED08A888DAB21C6E4B918434BE2B3AF1105EE1B94EAA9C4BB54428F4A71C711964F00848B9A9E00D8F55670991AADC16119FAFE26F6B9F61B0F1227A149D482D221B4D50F0FB780BC42D2DBA3FCD1F0250B90000000001984D41E0FEEEEFFEEEEFFEE0711EE7711EE7719AAEBB6AA74736151027000000000000000000000504").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = MosaicDefinitionTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicDefinitionTransactionV1_mosaic_definition_single_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = MosaicDefinitionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.duration = BlockDuration(1000);
            tmp_struct.nonce = MosaicNonce(3095715558);
            tmp_struct.flags = MosaicFlags::NONE;
            tmp_struct.divisibility = 3;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "F4B720AE0148D769C0F4C73355BBA1987EC6FA38C1E13F342C27BD3D52E76E95",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("D3B6BEF55F6D99281079B8A138EECE9A4CACC052BC3E84D83D72C3FCF0CFA85DEA390B8FCD50F1A6A6E196DDDED52CB92FC3C216C6B5F06F96E89B23FA62B4BE").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("9600000000000000D3B6BEF55F6D99281079B8A138EECE9A4CACC052BC3E84D83D72C3FCF0CFA85DEA390B8FCD50F1A6A6E196DDDED52CB92FC3C216C6B5F06F96E89B23FA62B4BEF4B720AE0148D769C0F4C73355BBA1987EC6FA38C1E13F342C27BD3D52E76E950000000001984D41E0FEEEEFFEEEEFFEE0711EE7711EE7719CBBDB70BCB8CB64E803000000000000E6DE84B80003").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = MosaicDefinitionTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicDefinitionTransactionV1_mosaic_definition_single_3() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = MosaicDefinitionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.duration = BlockDuration(0);
            tmp_struct.nonce = MosaicNonce(3095715558);
            tmp_struct.flags = MosaicFlags::REVOKABLE | MosaicFlags::TRANSFERABLE;
            tmp_struct.divisibility = 2;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "03F1E5E2ACDCABFCCF3CAB57CA7EA14A27934C970F7BC6E110EEB3B3BA27EB58",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("3712BC4F3932457AD1A7CC967CC45C3D5F04A52F6B802AEC7D377E504432F1DA40DD1EDAFE9F5899BD04DFBFB1324B198CCEE3344883DEA75DCCE2D1778B6529").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("96000000000000003712BC4F3932457AD1A7CC967CC45C3D5F04A52F6B802AEC7D377E504432F1DA40DD1EDAFE9F5899BD04DFBFB1324B198CCEE3344883DEA75DCCE2D1778B652903F1E5E2ACDCABFCCF3CAB57CA7EA14A27934C970F7BC6E110EEB3B3BA27EB580000000001984D41E0FEEEEFFEEEEFFEE0711EE7711EE771373C73AFF80478750000000000000000E6DE84B80A02").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = MosaicDefinitionTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicDefinitionTransactionV1_mosaic_definition_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedMosaicDefinitionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.duration = BlockDuration(10000);
                        tmp_struct.nonce = MosaicNonce(0);
                        tmp_struct.flags = MosaicFlags::RESTRICTABLE | MosaicFlags::SUPPLY_MUTABLE;
                        tmp_struct.divisibility = 4;
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "C80426D92D5FCE5A233B28C9A667275421454FCEF354171A3D47BAB904B04165",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("32D1342734620E653DE0D6987C77EBD99D8B9818E9BF20BB7E042BF96FD2A288CB16DA71B3D60AB7627DDD5C3BCEA5901DACBDAF42B55184C51D1F19E04C62AD").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "E22385E28D66F4A783AC56C45640070DB628B0A9192B1F773DED09C41123ADFA",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("F00000000000000032D1342734620E653DE0D6987C77EBD99D8B9818E9BF20BB7E042BF96FD2A288CB16DA71B3D60AB7627DDD5C3BCEA5901DACBDAF42B55184C51D1F19E04C62ADC80426D92D5FCE5A233B28C9A667275421454FCEF354171A3D47BAB904B041650000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771E22385E28D66F4A783AC56C45640070DB628B0A9192B1F773DED09C41123ADFA4800000000000000460000000000000000000000000000000000000000000000000000000000000000000000000000000000000001984D4101CE59EBE6B06F3210270000000000000000000005040000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicDefinitionTransactionV1_mosaic_definition_aggregate_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedMosaicDefinitionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.duration = BlockDuration(1000);
                        tmp_struct.nonce = MosaicNonce(3095715558);
                        tmp_struct.flags = MosaicFlags::NONE;
                        tmp_struct.divisibility = 3;
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "1DC7F9ACF33B0F3C8EFD4EEF6F3A2235763BCDC7443206CA77A3AE159EC06760",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("15C8937E60CE6D6EBFC8CD244B3C617E92E45E5C386C2DC7F05010039DF9B95D65529BD7646A7772390ED95828F65792399C78C53B00F349F142B6FBBC749BF8").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "9F08B173200F10F08F6FC4C6E1B37DAE1C3B425A98C8D1EB4B3BC44AF6B2906E",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("F00000000000000015C8937E60CE6D6EBFC8CD244B3C617E92E45E5C386C2DC7F05010039DF9B95D65529BD7646A7772390ED95828F65792399C78C53B00F349F142B6FBBC749BF81DC7F9ACF33B0F3C8EFD4EEF6F3A2235763BCDC7443206CA77A3AE159EC067600000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE7719F08B173200F10F08F6FC4C6E1B37DAE1C3B425A98C8D1EB4B3BC44AF6B2906E4800000000000000460000000000000000000000000000000000000000000000000000000000000000000000000000000000000001984D41B685550629D42453E803000000000000E6DE84B800030000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicDefinitionTransactionV1_mosaic_definition_aggregate_3() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedMosaicDefinitionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.duration = BlockDuration(0);
                        tmp_struct.nonce = MosaicNonce(3095715558);
                        tmp_struct.flags = MosaicFlags::REVOKABLE | MosaicFlags::TRANSFERABLE;
                        tmp_struct.divisibility = 2;
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "2BCD61B438EF36AF057DD6EA48110848BA1315559140E912D5C2FD4FFD1924E5",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("4AEF7D973B45E8D60E030DDE3D6EEB6CDA947FA7663A87223B780A6E9F23C28FC78B196EFFF719894B4E09D223D77F2B87D7334C06F47D95762E284326D10ADC").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "D1C267AFAC897195F41696647A89AC5E0B75A0910D0F2A3DD404F93113C35633",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("F0000000000000004AEF7D973B45E8D60E030DDE3D6EEB6CDA947FA7663A87223B780A6E9F23C28FC78B196EFFF719894B4E09D223D77F2B87D7334C06F47D95762E284326D10ADC2BCD61B438EF36AF057DD6EA48110848BA1315559140E912D5C2FD4FFD1924E50000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771D1C267AFAC897195F41696647A89AC5E0B75A0910D0F2A3DD404F93113C356334800000000000000460000000000000000000000000000000000000000000000000000000000000000000000000000000000000001984D41B685550629D424530000000000000000E6DE84B80A020000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicGlobalRestrictionTransactionV1_mosaic_global_restriction_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = MosaicGlobalRestrictionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.mosaic_id = UnresolvedMosaicId(1875072453572000775);
            tmp_struct.reference_mosaic_id = UnresolvedMosaicId(7706325451784159270);
            tmp_struct.restriction_key = 1;
            tmp_struct.previous_restriction_value = 9;
            tmp_struct.new_restriction_value = 8;
            tmp_struct.previous_restriction_type = MosaicRestrictionType::EQ;
            tmp_struct.new_restriction_type = MosaicRestrictionType::GE;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "087129803C315CE139F50755066C0E09C16BC2F7577C16D896035EBAC88FA0BB",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("A70ECFC4FAD876EDD481D02AF560DBC319E6AAB21DD33A9095BD45B1A5994844527F5DDBE7C10AE28D960436ACD0D6076D3D9F7ABE9473832F2839FB3370B95A").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("AA00000000000000A70ECFC4FAD876EDD481D02AF560DBC319E6AAB21DD33A9095BD45B1A5994844527F5DDBE7C10AE28D960436ACD0D6076D3D9F7ABE9473832F2839FB3370B95A087129803C315CE139F50755066C0E09C16BC2F7577C16D896035EBAC88FA0BB0000000001985141E0FEEEEFFEEEEFFEE0711EE7711EE771077C47437698051A268025252B5EF26A0100000000000000090000000000000008000000000000000106").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = MosaicGlobalRestrictionTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicGlobalRestrictionTransactionV1_mosaic_global_restriction_single_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = MosaicGlobalRestrictionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.mosaic_id = UnresolvedMosaicId(6207017352306769745);
            tmp_struct.reference_mosaic_id = UnresolvedMosaicId(0);
            tmp_struct.restriction_key = 4444;
            tmp_struct.previous_restriction_value = 0;
            tmp_struct.new_restriction_value = 0;
            tmp_struct.previous_restriction_type = MosaicRestrictionType::NONE;
            tmp_struct.new_restriction_type = MosaicRestrictionType::GE;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "65BC4608B8220C9E1AADFD8BCA1A813E0D67418A57664FF431FC1CD0803661CB",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("BB9436193DB00910693878E9530966643BAB80AF00C026FD3FD85327422707AD2E5F21B890C22220BC510301F5DC8DE7FAC2445F7022B4B8DEDC5D751E95ADF1").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("AA00000000000000BB9436193DB00910693878E9530966643BAB80AF00C026FD3FD85327422707AD2E5F21B890C22220BC510301F5DC8DE7FAC2445F7022B4B8DEDC5D751E95ADF165BC4608B8220C9E1AADFD8BCA1A813E0D67418A57664FF431FC1CD0803661CB0000000001985141E0FEEEEFFEEEEFFEE0711EE7711EE771513FEE4E65C1235600000000000000005C11000000000000000000000000000000000000000000000006").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = MosaicGlobalRestrictionTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicGlobalRestrictionTransactionV1_mosaic_global_restriction_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct =
                            EmbeddedMosaicGlobalRestrictionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.mosaic_id = UnresolvedMosaicId(1875072453572000775);
                        tmp_struct.reference_mosaic_id = UnresolvedMosaicId(7706325451784159270);
                        tmp_struct.restriction_key = 1;
                        tmp_struct.previous_restriction_value = 9;
                        tmp_struct.new_restriction_value = 8;
                        tmp_struct.previous_restriction_type = MosaicRestrictionType::EQ;
                        tmp_struct.new_restriction_type = MosaicRestrictionType::GE;
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "182A24DEA57CD468FF0AFE80790AC3362263D153AFC88301D440B14806322E0E",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("4FB58CAE68814089A1A4D74CA49B33288E426AC7AB956EA3AE9EBFFA8C3D0C26CDC0CBDE2B597425C53AAC2E27AC6DF11776A5A50C54436364EA0BE9E60BA746").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "94049515EBF52723CC7B217DE82D79D5ADFFF719C1934CB50AE91693FADEDC25",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("08010000000000004FB58CAE68814089A1A4D74CA49B33288E426AC7AB956EA3AE9EBFFA8C3D0C26CDC0CBDE2B597425C53AAC2E27AC6DF11776A5A50C54436364EA0BE9E60BA746182A24DEA57CD468FF0AFE80790AC3362263D153AFC88301D440B14806322E0E0000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE77194049515EBF52723CC7B217DE82D79D5ADFFF719C1934CB50AE91693FADEDC2560000000000000005A0000000000000000000000000000000000000000000000000000000000000000000000000000000000000001985141077C47437698051A268025252B5EF26A0100000000000000090000000000000008000000000000000106000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicGlobalRestrictionTransactionV1_mosaic_global_restriction_aggregate_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct =
                            EmbeddedMosaicGlobalRestrictionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.mosaic_id = UnresolvedMosaicId(6207017352306769745);
                        tmp_struct.reference_mosaic_id = UnresolvedMosaicId(0);
                        tmp_struct.restriction_key = 4444;
                        tmp_struct.previous_restriction_value = 0;
                        tmp_struct.new_restriction_value = 0;
                        tmp_struct.previous_restriction_type = MosaicRestrictionType::NONE;
                        tmp_struct.new_restriction_type = MosaicRestrictionType::GE;
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "FDA1EB87F40F913430F67F609534D1B31E6810E0AB0247D629E67ABC957C3470",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("AA268835334E38F0FF478710AE0E474399CDF49653E9924F8627F0BB09B59DC47C3231D8EA38F188E61407857BC2F918657E849939FBFC4FE6FD377DB5581773").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "8D93F6BF096B6D02432E54A73A39F70937971A10926195552EFC67396C9F33AB",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("0801000000000000AA268835334E38F0FF478710AE0E474399CDF49653E9924F8627F0BB09B59DC47C3231D8EA38F188E61407857BC2F918657E849939FBFC4FE6FD377DB5581773FDA1EB87F40F913430F67F609534D1B31E6810E0AB0247D629E67ABC957C34700000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE7718D93F6BF096B6D02432E54A73A39F70937971A10926195552EFC67396C9F33AB60000000000000005A0000000000000000000000000000000000000000000000000000000000000000000000000000000000000001985141513FEE4E65C1235600000000000000005C11000000000000000000000000000000000000000000000006000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicMetadataTransactionV1_mosaic_metadata_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = MosaicMetadataTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.target_address =
                UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap();
            tmp_struct.scoped_metadata_key = 10;
            tmp_struct.target_mosaic_id = UnresolvedMosaicId(1000);
            tmp_struct.value_size_delta = 10;
            tmp_struct.value = "313233414243".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "6693FC8E692C0848B0D0B0FAA89BE9D87FD6CB071642503A8F924F2A1255CD87",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("21F6DF84B68468A19A0E204EFC45A826C02991737D0C3334F42CB64928D9537886359B83316B16060A859A5A2C1819CBC36FF520DF5F17D1529240F256CEA94C").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("B20000000000000021F6DF84B68468A19A0E204EFC45A826C02991737D0C3334F42CB64928D9537886359B83316B16060A859A5A2C1819CBC36FF520DF5F17D1529240F256CEA94C6693FC8E692C0848B0D0B0FAA89BE9D87FD6CB071642503A8F924F2A1255CD870000000001984442E0FEEEEFFEEEEFFEE0711EE7711EE7719841E5B8E40781CF74DABF592817DE48711D778648DEAFB20A00000000000000E8030000000000000A000600313233414243").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = MosaicMetadataTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicMetadataTransactionV1_mosaic_metadata_single_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = MosaicMetadataTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.target_address =
                UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap();
            tmp_struct.scoped_metadata_key = 10;
            tmp_struct.target_mosaic_id = UnresolvedMosaicId(1000);
            tmp_struct.value_size_delta = -5;
            tmp_struct.value = "313233414243".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "ED1B9F006AFCAB3998EB0CE66E7D38D5F274097C8AC646CD08FD3DA5E81AAC8C",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("EE4682A5FBE4BA1C8F8131DF3C0DF7BE4E8BAF0E3A2B2D288101F2C5261932F03E02FDC4207B5FD7E44A4771E6D3895388213C48789982B42AF05CDEB7F88E26").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("B200000000000000EE4682A5FBE4BA1C8F8131DF3C0DF7BE4E8BAF0E3A2B2D288101F2C5261932F03E02FDC4207B5FD7E44A4771E6D3895388213C48789982B42AF05CDEB7F88E26ED1B9F006AFCAB3998EB0CE66E7D38D5F274097C8AC646CD08FD3DA5E81AAC8C0000000001984442E0FEEEEFFEEEEFFEE0711EE7711EE7719841E5B8E40781CF74DABF592817DE48711D778648DEAFB20A00000000000000E803000000000000FBFF0600313233414243").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = MosaicMetadataTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicMetadataTransactionV1_mosaic_metadata_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedMosaicMetadataTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.target_address =
                            UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ")
                                .unwrap();
                        tmp_struct.scoped_metadata_key = 10;
                        tmp_struct.target_mosaic_id = UnresolvedMosaicId(1000);
                        tmp_struct.value_size_delta = 10;
                        tmp_struct.value = "313233414243".as_bytes().to_vec();
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "D3FBC7DA6B186EAE0A96D7895CC452BF5A1574877FCB4181EA868534AD040A3E",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("8EA0617C41D6BD51E768BCFA2A63F380E909BC8291F8BA1F4855D0C52EDFAA07EFD4DA8E20D3506CEBF72B8509B420447254300ABE817242B7D6D0D7C2B53221").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "E2553E2E3FC4A959406B0F1AF9ADB9FC67D558615D523FD24119A7915FD00468",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("10010000000000008EA0617C41D6BD51E768BCFA2A63F380E909BC8291F8BA1F4855D0C52EDFAA07EFD4DA8E20D3506CEBF72B8509B420447254300ABE817242B7D6D0D7C2B53221D3FBC7DA6B186EAE0A96D7895CC452BF5A1574877FCB4181EA868534AD040A3E0000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771E2553E2E3FC4A959406B0F1AF9ADB9FC67D558615D523FD24119A7915FD0046868000000000000006200000000000000000000000000000000000000000000000000000000000000000000000000000000000000019844429841E5B8E40781CF74DABF592817DE48711D778648DEAFB20A00000000000000E8030000000000000A000600313233414243000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicMetadataTransactionV1_mosaic_metadata_aggregate_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedMosaicMetadataTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.target_address =
                            UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ")
                                .unwrap();
                        tmp_struct.scoped_metadata_key = 10;
                        tmp_struct.target_mosaic_id = UnresolvedMosaicId(1000);
                        tmp_struct.value_size_delta = -5;
                        tmp_struct.value = "313233414243".as_bytes().to_vec();
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "E8AEF6175C9CCD2726DBD499C67818B3F29995007026D5D4C47634D980DAF336",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("BF2CAD734C8F4AFB309D4543F9183B3BA7320D1302C0024D30BC310C81140D9DE47D22883282A995B614B80FBA8DFEDF1B4BE1D97AFB2192E881482B95BDCFD9").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "494C04ABA6F7275CDE4C6C829C99AC3C668EE50E46F6324020E0EADA8B08E518",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("1001000000000000BF2CAD734C8F4AFB309D4543F9183B3BA7320D1302C0024D30BC310C81140D9DE47D22883282A995B614B80FBA8DFEDF1B4BE1D97AFB2192E881482B95BDCFD9E8AEF6175C9CCD2726DBD499C67818B3F29995007026D5D4C47634D980DAF3360000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771494C04ABA6F7275CDE4C6C829C99AC3C668EE50E46F6324020E0EADA8B08E51868000000000000006200000000000000000000000000000000000000000000000000000000000000000000000000000000000000019844429841E5B8E40781CF74DABF592817DE48711D778648DEAFB20A00000000000000E803000000000000FBFF0600313233414243000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicSupplyChangeTransactionV1_mosaic_supply_change_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = MosaicSupplyChangeTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.mosaic_id = UnresolvedMosaicId(6300565133566699912);
            tmp_struct.action = MosaicSupplyChangeAction::INCREASE;
            tmp_struct.delta = Amount(10);
            tmp_struct.signer_public_key = PublicKey::from_str(
                "99A5B3328468287FF26BD7BD848F60077856DF7B648F877B4F35DA192933D0FF",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("E2170899C9BFFDB63EA730C1EE0AA60A9AB086C9127242101ACF0DEFCEA8A31D9B4CA37B6644AC2B6928527338C1CB2C87EA4ADBD98A9EFAC34430B9245C6F93").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("9100000000000000E2170899C9BFFDB63EA730C1EE0AA60A9AB086C9127242101ACF0DEFCEA8A31D9B4CA37B6644AC2B6928527338C1CB2C87EA4ADBD98A9EFAC34430B9245C6F9399A5B3328468287FF26BD7BD848F60077856DF7B648F877B4F35DA192933D0FF0000000001984D42E0FEEEEFFEEEEFFEE0711EE7711EE7718869746E9B1A70570A0000000000000001").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = MosaicSupplyChangeTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicSupplyChangeTransactionV1_mosaic_supply_change_single_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = MosaicSupplyChangeTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.mosaic_id = UnresolvedMosaicId(14624838436596993100);
            tmp_struct.action = MosaicSupplyChangeAction::DECREASE;
            tmp_struct.delta = Amount(10);
            tmp_struct.signer_public_key = PublicKey::from_str(
                "B62BF0114C49516097675612B2625882AD4727A9D29712E38D60E457F71BB0FE",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("575F393E380C091DCC5729454D7B839D52158AD00CFD07A735F385DBC0574266EAD33478F15B0F38788437B0F9249A4732808002E23ADC95B9BA1F3F1B86A222").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("9100000000000000575F393E380C091DCC5729454D7B839D52158AD00CFD07A735F385DBC0574266EAD33478F15B0F38788437B0F9249A4732808002E23ADC95B9BA1F3F1B86A222B62BF0114C49516097675612B2625882AD4727A9D29712E38D60E457F71BB0FE0000000001984D42E0FEEEEFFEEEEFFEE0711EE7711EE7714CCCD78612DDF5CA0A0000000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = MosaicSupplyChangeTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicSupplyChangeTransactionV1_mosaic_supply_change_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedMosaicSupplyChangeTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.mosaic_id = UnresolvedMosaicId(6300565133566699912);
                        tmp_struct.action = MosaicSupplyChangeAction::INCREASE;
                        tmp_struct.delta = Amount(10);
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "902ED39C89FB419BC56EB091AF48DB37A601C7D00E53C1F901617886FD7B2D50",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("654E486DC8DB96B8C5307ABA03C65DEBD61270A24AFC473703D1FF1DB0B554C91945A07C6B1D77DE5F1406E8B48EE09480097F0402397A5AF925E3B00C091EAC").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "6FFAC840B2C866960FCBCF42AF16B113FFE309A0991DFC0E4F3772E7AFC2FB69",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("F000000000000000654E486DC8DB96B8C5307ABA03C65DEBD61270A24AFC473703D1FF1DB0B554C91945A07C6B1D77DE5F1406E8B48EE09480097F0402397A5AF925E3B00C091EAC902ED39C89FB419BC56EB091AF48DB37A601C7D00E53C1F901617886FD7B2D500000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE7716FFAC840B2C866960FCBCF42AF16B113FFE309A0991DFC0E4F3772E7AFC2FB694800000000000000410000000000000000000000000000000000000000000000000000000000000000000000000000000000000001984D428869746E9B1A70570A000000000000000100000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicSupplyChangeTransactionV1_mosaic_supply_change_aggregate_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedMosaicSupplyChangeTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.mosaic_id = UnresolvedMosaicId(14624838436596993100);
                        tmp_struct.action = MosaicSupplyChangeAction::DECREASE;
                        tmp_struct.delta = Amount(10);
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "2A2DCED4834DEA7080D51288C0B6EDF62338D5BFDDEAAF92BF6879BAC28B49AE",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("8C31F816AE5EDBDCF507DF2F6E05CA7EDF4DE59AB8C3F67AA7474D44065A9E53618859CE09F2D76B92028EA267255B2DCCA6D1E0D7A10A5F3884F936883E25DF").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "137E5D6F7F63CCB9E2B51A4C22481D2766E8A4FCD6A387E667A35723F2C68428",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("F0000000000000008C31F816AE5EDBDCF507DF2F6E05CA7EDF4DE59AB8C3F67AA7474D44065A9E53618859CE09F2D76B92028EA267255B2DCCA6D1E0D7A10A5F3884F936883E25DF2A2DCED4834DEA7080D51288C0B6EDF62338D5BFDDEAAF92BF6879BAC28B49AE0000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771137E5D6F7F63CCB9E2B51A4C22481D2766E8A4FCD6A387E667A35723F2C684284800000000000000410000000000000000000000000000000000000000000000000000000000000000000000000000000000000001984D424CCCD78612DDF5CA0A000000000000000000000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MultisigAccountModificationTransactionV1_multisig_account_modification_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = MultisigAccountModificationTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.min_removal_delta = 1;
            tmp_struct.min_approval_delta = 2;
            tmp_struct.address_additions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap(),
                );
                tmp_vec.push(
                    UnresolvedAddress::from_str("TD2ASJ2LKL5LX66PPZ67PYQN4HIMH5SX7OCZLQI").unwrap(),
                );
                tmp_vec
            };
            tmp_struct.address_deletions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I").unwrap(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "A17456F637644D6F2E2DFD09CC8F5DE037F93829978949AD010FF6E9F5A0EBC6",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("4E30A6E6477467C8314BEFF4922D58C33ED32AF351DD88640AF200EB4EE9C6FAD92B42D7FA236485F99D4D2C253993A66B2B00454A1159E71CBB3EB51394AC67").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("D0000000000000004E30A6E6477467C8314BEFF4922D58C33ED32AF351DD88640AF200EB4EE9C6FAD92B42D7FA236485F99D4D2C253993A66B2B00454A1159E71CBB3EB51394AC67A17456F637644D6F2E2DFD09CC8F5DE037F93829978949AD010FF6E9F5A0EBC60000000001985541E0FEEEEFFEEEEFFEE0711EE7711EE77101020201000000009841E5B8E40781CF74DABF592817DE48711D778648DEAFB298F409274B52FABBFBCF7E7DF7E20DE1D0C3F657FB8595C1989059321905F681BCF47EA33BBF5E6F8298B5440854FDED").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = MultisigAccountModificationTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn MultisigAccountModificationTransactionV1_multisig_account_modification_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct =
                            EmbeddedMultisigAccountModificationTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.min_removal_delta = 1;
                        tmp_struct.min_approval_delta = 2;
                        tmp_struct.address_additions = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push(
                                UnresolvedAddress::from_str(
                                    "TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ",
                                )
                                .unwrap(),
                            );
                            tmp_vec.push(
                                UnresolvedAddress::from_str(
                                    "TD2ASJ2LKL5LX66PPZ67PYQN4HIMH5SX7OCZLQI",
                                )
                                .unwrap(),
                            );
                            tmp_vec
                        };
                        tmp_struct.address_deletions = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push(
                                UnresolvedAddress::from_str(
                                    "TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I",
                                )
                                .unwrap(),
                            );
                            tmp_vec
                        };
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "16C8E2E90D757A1C3120DA3F28591C808925E97237165D58D5D48CF761718A7C",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("5DCE50608AF36771B8DBD26FF07CE6284500C24606DAD95ECBAEEB51220896AFC59BE14C353FC1037F6089FB70480752D6402A68278E9457C6D66D85719A4E92").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "23CC3A9D303266D2E163A8B8AA1A991F3EC9012B7F7490C6870BB5F6ED9E4D8D",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("28010000000000005DCE50608AF36771B8DBD26FF07CE6284500C24606DAD95ECBAEEB51220896AFC59BE14C353FC1037F6089FB70480752D6402A68278E9457C6D66D85719A4E9216C8E2E90D757A1C3120DA3F28591C808925E97237165D58D5D48CF761718A7C0000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE77123CC3A9D303266D2E163A8B8AA1A991F3EC9012B7F7490C6870BB5F6ED9E4D8D800000000000000080000000000000000000000000000000000000000000000000000000000000000000000000000000000000000198554101020201000000009841E5B8E40781CF74DABF592817DE48711D778648DEAFB298F409274B52FABBFBCF7E7DF7E20DE1D0C3F657FB8595C1989059321905F681BCF47EA33BBF5E6F8298B5440854FDED").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceMetadataTransactionV1_namespace_metadata_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = NamespaceMetadataTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.target_address =
                UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I").unwrap();
            tmp_struct.scoped_metadata_key = 10;
            tmp_struct.target_namespace_id = NamespaceId(1000);
            tmp_struct.value_size_delta = 10;
            tmp_struct.value = "ABC123".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "32AA3E853021087C2DB1349483FED1C1A729A7F41B0BE84905D9E0463EEEC417",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("067D7153D66ED03E696208DAFB698C1EC0ECD92DA3AFEC180E082FA84133F1E5B9B0F0ACD14CEBE867DBA15DD37CB9CC413AAB3EF73E9929977337E6A8F2AB44").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("B200000000000000067D7153D66ED03E696208DAFB698C1EC0ECD92DA3AFEC180E082FA84133F1E5B9B0F0ACD14CEBE867DBA15DD37CB9CC413AAB3EF73E9929977337E6A8F2AB4432AA3E853021087C2DB1349483FED1C1A729A7F41B0BE84905D9E0463EEEC4170000000001984443E0FEEEEFFEEEEFFEE0711EE7711EE771989059321905F681BCF47EA33BBF5E6F8298B5440854FDED0A00000000000000E8030000000000000A000600414243313233").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = NamespaceMetadataTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceMetadataTransactionV1_namespace_metadata_single_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = NamespaceMetadataTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.target_address =
                UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I").unwrap();
            tmp_struct.scoped_metadata_key = 10;
            tmp_struct.target_namespace_id = NamespaceId(1000);
            tmp_struct.value_size_delta = -3;
            tmp_struct.value = "ABC123".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "81C75ECF85231AA30DA72890D10825A7BB419044FE959DD13C9CDE850D9D43DC",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("FD0DDFDC372E15AE261A4AF8C61EFE37FAE5E4D6D8B6E53AA83ED616BD002C1700D2B594C841472C3DC24E4B74DE5E01968A943F8AE7BC34B9C59C9918DA2A46").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("B200000000000000FD0DDFDC372E15AE261A4AF8C61EFE37FAE5E4D6D8B6E53AA83ED616BD002C1700D2B594C841472C3DC24E4B74DE5E01968A943F8AE7BC34B9C59C9918DA2A4681C75ECF85231AA30DA72890D10825A7BB419044FE959DD13C9CDE850D9D43DC0000000001984443E0FEEEEFFEEEEFFEE0711EE7711EE771989059321905F681BCF47EA33BBF5E6F8298B5440854FDED0A00000000000000E803000000000000FDFF0600414243313233").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = NamespaceMetadataTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceMetadataTransactionV1_namespace_metadata_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedNamespaceMetadataTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.target_address =
                            UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I")
                                .unwrap();
                        tmp_struct.scoped_metadata_key = 10;
                        tmp_struct.target_namespace_id = NamespaceId(1000);
                        tmp_struct.value_size_delta = 10;
                        tmp_struct.value = "ABC123".as_bytes().to_vec();
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "F926C14BE870332CE4DA137D5CAE32B8F9DF0AEEC11A07231C0C6BF47F4F730F",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("E4BA86E9FD077A8D66EC1B283EE9D771DB7BEA4B169F77A2B24D3D83362F48A028FF8762FE47B1B21B1D8F2515F97485CB25B251F8DE540BBCF5E7B5485F7B57").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "A716D958F0076204E3F1DDD9CCFB4087C8B934826E977A978914CF3D619494EE",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("1001000000000000E4BA86E9FD077A8D66EC1B283EE9D771DB7BEA4B169F77A2B24D3D83362F48A028FF8762FE47B1B21B1D8F2515F97485CB25B251F8DE540BBCF5E7B5485F7B57F926C14BE870332CE4DA137D5CAE32B8F9DF0AEEC11A07231C0C6BF47F4F730F0000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771A716D958F0076204E3F1DDD9CCFB4087C8B934826E977A978914CF3D619494EE6800000000000000620000000000000000000000000000000000000000000000000000000000000000000000000000000000000001984443989059321905F681BCF47EA33BBF5E6F8298B5440854FDED0A00000000000000E8030000000000000A000600414243313233000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceMetadataTransactionV1_namespace_metadata_aggregate_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedNamespaceMetadataTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.target_address =
                            UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I")
                                .unwrap();
                        tmp_struct.scoped_metadata_key = 10;
                        tmp_struct.target_namespace_id = NamespaceId(1000);
                        tmp_struct.value_size_delta = -3;
                        tmp_struct.value = "ABC123".as_bytes().to_vec();
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "1623C5D275D0DFE188F944922E884A9C83E20E2ACA7555D4B79CBACA15CDB82F",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("E241E4975832859BFC491EA4F2DABB1F29C984C0035D403C151CEBDB5DFB6F6FCDAFE7F79921213DBB0733C593CDF711E61CACE4A83877C1AFB2D0C2CF6A4E3C").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "A92D6804B56AF5C4439906441DEE2EC265756E9D95914230483A4D1BF6283C1D",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("1001000000000000E241E4975832859BFC491EA4F2DABB1F29C984C0035D403C151CEBDB5DFB6F6FCDAFE7F79921213DBB0733C593CDF711E61CACE4A83877C1AFB2D0C2CF6A4E3C1623C5D275D0DFE188F944922E884A9C83E20E2ACA7555D4B79CBACA15CDB82F0000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771A92D6804B56AF5C4439906441DEE2EC265756E9D95914230483A4D1BF6283C1D6800000000000000620000000000000000000000000000000000000000000000000000000000000000000000000000000000000001984443989059321905F681BCF47EA33BBF5E6F8298B5440854FDED0A00000000000000E803000000000000FDFF0600414243313233000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceRegistrationTransactionV1_namespace_registration_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = NamespaceRegistrationTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.duration = BlockDuration(10000);
            tmp_struct.id = NamespaceId(13858666424160217470);
            tmp_struct.registration_type = NamespaceRegistrationType::ROOT;
            tmp_struct.name = "newnamespace".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "02632A2AEB78C3155D3008461457706F2540EDD3F80264245A64EF82ACDEDF5A",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("6F81F080720F6F641386F1320BCD4B641345CA1D3FF4D7DE302B0EA28D0E8869F3FCC0BACD72C3FF897CB620ED6B713B07F68B6312428A3C6C09B88FCAD0789A").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("9E000000000000006F81F080720F6F641386F1320BCD4B641345CA1D3FF4D7DE302B0EA28D0E8869F3FCC0BACD72C3FF897CB620ED6B713B07F68B6312428A3C6C09B88FCAD0789A02632A2AEB78C3155D3008461457706F2540EDD3F80264245A64EF82ACDEDF5A0000000001984E41E0FEEEEFFEEEEFFEE0711EE7711EE77110270000000000007EE9B3B8AFDF53C0000C6E65776E616D657370616365").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = NamespaceRegistrationTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceRegistrationTransactionV1_namespace_registration_single_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = NamespaceRegistrationTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.parent_id = NamespaceId(4635294387305441662);
            tmp_struct.id = NamespaceId(17411894141110456835);
            tmp_struct.registration_type = NamespaceRegistrationType::CHILD;
            tmp_struct.name = "subnamespace".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "D3AFA86AEDDE967FDD6663BAB6B64325700AD88C9C56CA6FC9CBC3D68233F1D4",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("59C951AD8691705F1EB49D80B78B850B4114F38E0FCC64DAC404E9AA44DCBAA8A3DCFE82DF1275E278F8B8C98D3B83FB6328F257937AD4490B944C4AE27904B3").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("9E0000000000000059C951AD8691705F1EB49D80B78B850B4114F38E0FCC64DAC404E9AA44DCBAA8A3DCFE82DF1275E278F8B8C98D3B83FB6328F257937AD4490B944C4AE27904B3D3AFA86AEDDE967FDD6663BAB6B64325700AD88C9C56CA6FC9CBC3D68233F1D40000000001984E41E0FEEEEFFEEEEFFEE0711EE7711EE7717EE9B3B8AFDF53400312981B7879A3F1010C7375626E616D657370616365").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = NamespaceRegistrationTransactionV1::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceRegistrationTransactionV1_namespace_registration_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedNamespaceRegistrationTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.duration = BlockDuration(10000);
                        tmp_struct.id = NamespaceId(13858666424160217470);
                        tmp_struct.registration_type = NamespaceRegistrationType::ROOT;
                        tmp_struct.name = "newnamespace".as_bytes().to_vec();
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "FDE5BEA4E510DC5995F62232864A504F48DF767C84019CBEB5E5389F35E19411",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("4C08B813E15C24982EE1D908942CBC07F7EE373EB78F99935D657CAB1CE6397156FF07C97D334F8E2E71B57E293E98B0523633FF36C052E3AB0B5E3FF4924310").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "64148373332A1284E316AC070194019D786C29F3B879A0AAFACEC2E393D0FCB5",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("F8000000000000004C08B813E15C24982EE1D908942CBC07F7EE373EB78F99935D657CAB1CE6397156FF07C97D334F8E2E71B57E293E98B0523633FF36C052E3AB0B5E3FF4924310FDE5BEA4E510DC5995F62232864A504F48DF767C84019CBEB5E5389F35E194110000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE77164148373332A1284E316AC070194019D786C29F3B879A0AAFACEC2E393D0FCB550000000000000004E0000000000000000000000000000000000000000000000000000000000000000000000000000000000000001984E4110270000000000007EE9B3B8AFDF53C0000C6E65776E616D6573706163650000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceRegistrationTransactionV1_namespace_registration_aggregate_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedNamespaceRegistrationTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.parent_id = NamespaceId(4635294387305441662);
                        tmp_struct.id = NamespaceId(17411894141110456835);
                        tmp_struct.registration_type = NamespaceRegistrationType::CHILD;
                        tmp_struct.name = "subnamespace".as_bytes().to_vec();
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "BE3BB43008F493D70FC721B1D845D6646740E9D9DF834E8C02C7930737FCF0CC",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("BC0F54C2F8ECC9AF9964BA7BBD76797981A7030F037228B256F220C04E0CA1A9C2C45C9E4A8914143E1AAD5E1DFDB2A4503BC1D0095EB21FC2CD8B0DF21D31A2").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "9777CD6B81EED8832122E7D4692D5AC09D6144D30E3A8D1DF559FDB21C1B4FAC",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("F800000000000000BC0F54C2F8ECC9AF9964BA7BBD76797981A7030F037228B256F220C04E0CA1A9C2C45C9E4A8914143E1AAD5E1DFDB2A4503BC1D0095EB21FC2CD8B0DF21D31A2BE3BB43008F493D70FC721B1D845D6646740E9D9DF834E8C02C7930737FCF0CC0000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE7719777CD6B81EED8832122E7D4692D5AC09D6144D30E3A8D1DF559FDB21C1B4FAC50000000000000004E0000000000000000000000000000000000000000000000000000000000000000000000000000000000000001984E417EE9B3B8AFDF53400312981B7879A3F1010C7375626E616D6573706163650000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn NodeKeyLinkTransactionV1_node_key_link_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = NodeKeyLinkTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.linked_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.linked_public_key = PublicKey::from_str(
                "E787EDC111A3A8E3E8C94DF66755065D66A34563ED09C83755C38C24D3B7F08A",
            )
            .unwrap();
            tmp_struct.link_action = LinkAction::LINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "B2ADB234491F5093B682643136119998A9960477391ACF84A0D003BDF7E7790D",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("3CCE9BCD544BFF665A3400F7337A5115307ABB490AD821B6EE8F2906805B4B4C7D525EC20B52B9F6D7FEAA0CC6C20E6A613F2395916AC07F4ACC34FAD57F177D").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("A1000000000000003CCE9BCD544BFF665A3400F7337A5115307ABB490AD821B6EE8F2906805B4B4C7D525EC20B52B9F6D7FEAA0CC6C20E6A613F2395916AC07F4ACC34FAD57F177DB2ADB234491F5093B682643136119998A9960477391ACF84A0D003BDF7E7790D0000000001984C42E0FEEEEFFEEEEFFEE0711EE7711EE771E787EDC111A3A8E3E8C94DF66755065D66A34563ED09C83755C38C24D3B7F08A01").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = NodeKeyLinkTransactionV1::deserialize(&payload).unwrap().0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn NodeKeyLinkTransactionV1_node_key_link_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedNodeKeyLinkTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.linked_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.linked_public_key = PublicKey::from_str(
                            "E787EDC111A3A8E3E8C94DF66755065D66A34563ED09C83755C38C24D3B7F08A",
                        )
                        .unwrap();
                        tmp_struct.link_action = LinkAction::LINK;
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "A97F5A4FD3D87FA2136CD42D9834A76D76F1247015F007513998CE7C9ADF3B87",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("4A84BB8D0EFC259B98887D4E14C146041D4C9C8ED06F56E224893AA19969A8DB3515ED03430BCA5F47A17F386C184741596EBCFA7110D9CF569B8FFC8300693F").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "D1E0E4543AD54FB41747EAA74009AE05DB685DD0FB2B8CB6385327DCC71ED8B2",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("00010000000000004A84BB8D0EFC259B98887D4E14C146041D4C9C8ED06F56E224893AA19969A8DB3515ED03430BCA5F47A17F386C184741596EBCFA7110D9CF569B8FFC8300693FA97F5A4FD3D87FA2136CD42D9834A76D76F1247015F007513998CE7C9ADF3B870000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771D1E0E4543AD54FB41747EAA74009AE05DB685DD0FB2B8CB6385327DCC71ED8B25800000000000000510000000000000000000000000000000000000000000000000000000000000000000000000000000000000001984C42E787EDC111A3A8E3E8C94DF66755065D66A34563ED09C83755C38C24D3B7F08A0100000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn SecretLockTransactionV1_secret_lock_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = SecretLockTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.recipient_address =
                UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap();
            tmp_struct.secret = Hash256::from_str(
                "3FC8BA10229AB5778D05D9C4B7F56676A88BF9295C185ACFC0F961DB5408CAFE",
            )
            .unwrap();
            tmp_struct.mosaic = {
                let mut tmp_struct = UnresolvedMosaic::default();
                tmp_struct.mosaic_id = UnresolvedMosaicId(9636553580561478212);
                tmp_struct.amount = Amount(10000000);
                tmp_struct
            };
            tmp_struct.duration = BlockDuration(100);
            tmp_struct.hash_algorithm = LockHashAlgorithm::SHA3_256;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "506A1C9364164E40422BDAC9135F19F6E3D186068F883FD9AF4C677D433C39A3",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("5E9808E11624AAABD2826CF7F464B93F309B8F15506BE6FC7E8C1E5E09E23B4D13A37C5982225413DDD6CA5913F4F4673662732059AD381DF191A01C72CB6D5D").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("D1000000000000005E9808E11624AAABD2826CF7F464B93F309B8F15506BE6FC7E8C1E5E09E23B4D13A37C5982225413DDD6CA5913F4F4673662732059AD381DF191A01C72CB6D5D506A1C9364164E40422BDAC9135F19F6E3D186068F883FD9AF4C677D433C39A30000000001985241E0FEEEEFFEEEEFFEE0711EE7711EE7719841E5B8E40781CF74DABF592817DE48711D778648DEAFB23FC8BA10229AB5778D05D9C4B7F56676A88BF9295C185ACFC0F961DB5408CAFE44B262C46CEABB858096980000000000640000000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = SecretLockTransactionV1::deserialize(&payload).unwrap().0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn SecretLockTransactionV1_secret_lock_single_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = SecretLockTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.recipient_address =
                UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap();
            tmp_struct.mosaic = {
                let mut tmp_struct = UnresolvedMosaic::default();
                tmp_struct.mosaic_id = UnresolvedMosaicId(9636553580561478212);
                tmp_struct.amount = Amount(1311768467294899695);
                tmp_struct
            };
            tmp_struct.duration = BlockDuration(100);
            tmp_struct.secret = Hash256::from_str(
                "59CC35F8C8D91867717CE4290B40EA636E86CE5C000000000000000000000000",
            )
            .unwrap();
            tmp_struct.hash_algorithm = LockHashAlgorithm::HASH_160;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "5D61CF2A251A2ADFD0788134F115C91734401A4F390941A5AE9D0B92AA2EA752",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("1F97199EAA5A7B3D956DE51DB9E93490A72123ECDC7C2931ED4B3EA9D02FD9443F9F5028B92D5CF5A32DD1F9802D0D5B703BE5FFFDB3480D0915C8BE7ABE62FD").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("D1000000000000001F97199EAA5A7B3D956DE51DB9E93490A72123ECDC7C2931ED4B3EA9D02FD9443F9F5028B92D5CF5A32DD1F9802D0D5B703BE5FFFDB3480D0915C8BE7ABE62FD5D61CF2A251A2ADFD0788134F115C91734401A4F390941A5AE9D0B92AA2EA7520000000001985241E0FEEEEFFEEEEFFEE0711EE7711EE7719841E5B8E40781CF74DABF592817DE48711D778648DEAFB259CC35F8C8D91867717CE4290B40EA636E86CE5C00000000000000000000000044B262C46CEABB85EFCDAB9078563412640000000000000001").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = SecretLockTransactionV1::deserialize(&payload).unwrap().0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn SecretLockTransactionV1_secret_lock_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedSecretLockTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.recipient_address =
                            UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ")
                                .unwrap();
                        tmp_struct.secret = Hash256::from_str(
                            "3FC8BA10229AB5778D05D9C4B7F56676A88BF9295C185ACFC0F961DB5408CAFE",
                        )
                        .unwrap();
                        tmp_struct.mosaic = {
                            let mut tmp_struct = UnresolvedMosaic::default();
                            tmp_struct.mosaic_id = UnresolvedMosaicId(9636553580561478212);
                            tmp_struct.amount = Amount(10000000);
                            tmp_struct
                        };
                        tmp_struct.duration = BlockDuration(100);
                        tmp_struct.hash_algorithm = LockHashAlgorithm::SHA3_256;
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "4191AD7B9C099B32159D5595EBF1A1274F0464690A4533FCA12379B9BD4E8795",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("D2CECD95BAA2B1F170D2EA70E7EC4A32C6DF0813CCE37C900262BDF1A13E16EE9F54F1A9F31E80DC488D43EDBE3072103AA74B7E064EAEDEF5BAB348B45541E1").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "36927A7B0987EB9A13129BA53AC0597E96F9D8F2C8306EA3F750518ACD15529A",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("3001000000000000D2CECD95BAA2B1F170D2EA70E7EC4A32C6DF0813CCE37C900262BDF1A13E16EE9F54F1A9F31E80DC488D43EDBE3072103AA74B7E064EAEDEF5BAB348B45541E14191AD7B9C099B32159D5595EBF1A1274F0464690A4533FCA12379B9BD4E87950000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE77136927A7B0987EB9A13129BA53AC0597E96F9D8F2C8306EA3F750518ACD15529A88000000000000008100000000000000000000000000000000000000000000000000000000000000000000000000000000000000019852419841E5B8E40781CF74DABF592817DE48711D778648DEAFB23FC8BA10229AB5778D05D9C4B7F56676A88BF9295C185ACFC0F961DB5408CAFE44B262C46CEABB85809698000000000064000000000000000000000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn SecretLockTransactionV1_secret_lock_aggregate_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedSecretLockTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.recipient_address =
                            UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ")
                                .unwrap();
                        tmp_struct.mosaic = {
                            let mut tmp_struct = UnresolvedMosaic::default();
                            tmp_struct.mosaic_id = UnresolvedMosaicId(9636553580561478212);
                            tmp_struct.amount = Amount(1311768467294899695);
                            tmp_struct
                        };
                        tmp_struct.duration = BlockDuration(100);
                        tmp_struct.secret = Hash256::from_str(
                            "59CC35F8C8D91867717CE4290B40EA636E86CE5C000000000000000000000000",
                        )
                        .unwrap();
                        tmp_struct.hash_algorithm = LockHashAlgorithm::HASH_160;
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "246716997D7A0A6D612A718705908579A2E9C9D53179328DD7BD38D2B10B3DF8",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("3F55F0B895EDBC04AA511CB828326E32FBCD57D5A2688BF1F9FC3A2C604F7BF8B55929FB0B18624A58641CF010A628DC7D64FDFEA1EEC5315950E2559BD5458B").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "81F7349CC9785016A1435572751389F02926573244D7F97E14F811D60627750A",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("30010000000000003F55F0B895EDBC04AA511CB828326E32FBCD57D5A2688BF1F9FC3A2C604F7BF8B55929FB0B18624A58641CF010A628DC7D64FDFEA1EEC5315950E2559BD5458B246716997D7A0A6D612A718705908579A2E9C9D53179328DD7BD38D2B10B3DF80000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE77181F7349CC9785016A1435572751389F02926573244D7F97E14F811D60627750A88000000000000008100000000000000000000000000000000000000000000000000000000000000000000000000000000000000019852419841E5B8E40781CF74DABF592817DE48711D778648DEAFB259CC35F8C8D91867717CE4290B40EA636E86CE5C00000000000000000000000044B262C46CEABB85EFCDAB907856341264000000000000000100000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn SecretProofTransactionV1_secret_proof_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = SecretProofTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.recipient_address =
                UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I").unwrap();
            tmp_struct.secret = Hash256::from_str(
                "3FC8BA10229AB5778D05D9C4B7F56676A88BF9295C185ACFC0F961DB5408CAFE",
            )
            .unwrap();
            tmp_struct.hash_algorithm = LockHashAlgorithm::SHA3_256;
            tmp_struct.proof = "9A493664".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "E2E9B5CF15554F11A98A250167E936F731865F3318B41E9B18753BFE31A8E4D6",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("AF3DC7E901D3DBA59F26DB495339E55466C70DDAFD4993CFA437CC260C5829774A3A8891758C20D1E4432D53C9B23FD500972FB212325CC0160300BEE521B444").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("BF00000000000000AF3DC7E901D3DBA59F26DB495339E55466C70DDAFD4993CFA437CC260C5829774A3A8891758C20D1E4432D53C9B23FD500972FB212325CC0160300BEE521B444E2E9B5CF15554F11A98A250167E936F731865F3318B41E9B18753BFE31A8E4D60000000001985242E0FEEEEFFEEEEFFEE0711EE7711EE771989059321905F681BCF47EA33BBF5E6F8298B5440854FDED3FC8BA10229AB5778D05D9C4B7F56676A88BF9295C185ACFC0F961DB5408CAFE0400009A493664").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = SecretProofTransactionV1::deserialize(&payload).unwrap().0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn SecretProofTransactionV1_secret_proof_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedSecretProofTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.recipient_address =
                            UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I")
                                .unwrap();
                        tmp_struct.secret = Hash256::from_str(
                            "3FC8BA10229AB5778D05D9C4B7F56676A88BF9295C185ACFC0F961DB5408CAFE",
                        )
                        .unwrap();
                        tmp_struct.hash_algorithm = LockHashAlgorithm::SHA3_256;
                        tmp_struct.proof = "9A493664".as_bytes().to_vec();
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "5D84E89527FF5DBA39EA1B00B8230BB96AC13682FDD336DABD3BDA745F17C2CB",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("C7A226BE39161700A4EAAA38663DE7FD9A3ECDDB6D8AE5BE745FA97CAB9A994CD3B6AD7199C586EE62FB3A1860888B5306F34D6AA31D856B8DDBFBAA51E9A924").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "2082780E43D0C6AB646FF178295F5B7CE48B9DE845A3DA98EF595433BDA184E9",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("1801000000000000C7A226BE39161700A4EAAA38663DE7FD9A3ECDDB6D8AE5BE745FA97CAB9A994CD3B6AD7199C586EE62FB3A1860888B5306F34D6AA31D856B8DDBFBAA51E9A9245D84E89527FF5DBA39EA1B00B8230BB96AC13682FDD336DABD3BDA745F17C2CB0000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE7712082780E43D0C6AB646FF178295F5B7CE48B9DE845A3DA98EF595433BDA184E970000000000000006F0000000000000000000000000000000000000000000000000000000000000000000000000000000000000001985242989059321905F681BCF47EA33BBF5E6F8298B5440854FDED3FC8BA10229AB5778D05D9C4B7F56676A88BF9295C185ACFC0F961DB5408CAFE0400009A49366400").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = TransferTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.recipient_address =
                UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I").unwrap();
            tmp_struct.mosaics = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push({
                    let mut tmp_struct = UnresolvedMosaic::default();
                    tmp_struct.mosaic_id = UnresolvedMosaicId(95442763262823);
                    tmp_struct.amount = Amount(100);
                    tmp_struct
                });
                tmp_vec.sort_unstable();
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "B9E7DC522F51BEA8AEF69B11BA0BE6461ED6877B2F7E82B45B7C28CE79738101",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("2396B87D65DDDCF52F527CC4C8E2C413C52DA4E2D2D951E5EB1370941D86068688099761AD473A3D124650B823C39078B9326EC8CD050FE2EB6ABC9FE61C0212").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("B0000000000000002396B87D65DDDCF52F527CC4C8E2C413C52DA4E2D2D951E5EB1370941D86068688099761AD473A3D124650B823C39078B9326EC8CD050FE2EB6ABC9FE61C0212B9E7DC522F51BEA8AEF69B11BA0BE6461ED6877B2F7E82B45B7C28CE797381010000000001985441E0FEEEEFFEEEEFFEE0711EE7711EE771989059321905F681BCF47EA33BBF5E6F8298B5440854FDED0000010000000000672B0000CE5600006400000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = TransferTransactionV1::deserialize(&payload).unwrap().0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = TransferTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.recipient_address =
                UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I").unwrap();
            tmp_struct.mosaics = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push({
                    let mut tmp_struct = UnresolvedMosaic::default();
                    tmp_struct.mosaic_id = UnresolvedMosaicId(100);
                    tmp_struct.amount = Amount(2);
                    tmp_struct
                });
                tmp_vec.push({
                    let mut tmp_struct = UnresolvedMosaic::default();
                    tmp_struct.mosaic_id = UnresolvedMosaicId(200);
                    tmp_struct.amount = Amount(1);
                    tmp_struct
                });
                tmp_vec.sort_unstable();
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "BDEDB7F83716B5AC6B022AF175C00C011B42286618EF06BD3B1C35D2781286AA",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("42D030CD0166DA62C1DF1FF0945752475FBD2B4B975E9991EFF57BCD742C235787433B8AF428C3852009C8C63B632572057945118F393F4187FF51DFD77CAC6D").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("C00000000000000042D030CD0166DA62C1DF1FF0945752475FBD2B4B975E9991EFF57BCD742C235787433B8AF428C3852009C8C63B632572057945118F393F4187FF51DFD77CAC6DBDEDB7F83716B5AC6B022AF175C00C011B42286618EF06BD3B1C35D2781286AA0000000001985441E0FEEEEFFEEEEFFEE0711EE7711EE771989059321905F681BCF47EA33BBF5E6F8298B5440854FDED000002000000000064000000000000000200000000000000C8000000000000000100000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = TransferTransactionV1::deserialize(&payload).unwrap().0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_3() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = TransferTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.recipient_address =
                UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I").unwrap();
            tmp_struct.mosaics = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push({
                    let mut tmp_struct = UnresolvedMosaic::default();
                    tmp_struct.mosaic_id = UnresolvedMosaicId(7490250818323297978);
                    tmp_struct.amount = Amount(3);
                    tmp_struct
                });
                tmp_vec.push({
                    let mut tmp_struct = UnresolvedMosaic::default();
                    tmp_struct.mosaic_id = UnresolvedMosaicId(8620336746491119575);
                    tmp_struct.amount = Amount(2);
                    tmp_struct
                });
                tmp_vec.push({
                    let mut tmp_struct = UnresolvedMosaic::default();
                    tmp_struct.mosaic_id = UnresolvedMosaicId(15358872602548358953);
                    tmp_struct.amount = Amount(1);
                    tmp_struct
                });
                tmp_vec.sort_unstable();
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "8290EA0BC248DB72174B55D359861DFF4401DF075508FBC0C0C35515E4E3CD48",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("C1DD9E45551CF35D8F058C73A8E3813B107A5D6EC6393F60B8B2F294E1C831FF96F30CB71D18EBEE2C96146D97DF1CFA252B8B3988697015150D7CDFEF884463").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("D000000000000000C1DD9E45551CF35D8F058C73A8E3813B107A5D6EC6393F60B8B2F294E1C831FF96F30CB71D18EBEE2C96146D97DF1CFA252B8B3988697015150D7CDFEF8844638290EA0BC248DB72174B55D359861DFF4401DF075508FBC0C0C35515E4E3CD480000000001985441E0FEEEEFFEEEEFFEE0711EE7711EE771989059321905F681BCF47EA33BBF5E6F8298B5440854FDED0000030000000000BA36BD286FB7F2670300000000000000D787D9329996A177020000000000000029CF5FD941AD25D50100000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = TransferTransactionV1::deserialize(&payload).unwrap().0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_4() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = TransferTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.recipient_address =
                UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I").unwrap();
            tmp_struct.mosaics = {
                let mut tmp_vec = Vec::new();
                tmp_vec.sort_unstable();
                tmp_vec
            };
            tmp_struct.message = "D600000300504C5445000000FBAF93F7".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "7FB9B855A5A52B10C3FAF4670DCA953F10EFC223F0CC2483E1E69571889BF30C",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("5A5763BD9CE487F745C0A5F4D2D2F4167778878C9C119B03C549F915ED471B6AD05F51A76C4CE9CC7BCF58958A6DC64B3C43584D1651B64FBBFCD42FCAD1DEBF").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("B0000000000000005A5763BD9CE487F745C0A5F4D2D2F4167778878C9C119B03C549F915ED471B6AD05F51A76C4CE9CC7BCF58958A6DC64B3C43584D1651B64FBBFCD42FCAD1DEBF7FB9B855A5A52B10C3FAF4670DCA953F10EFC223F0CC2483E1E69571889BF30C0000000001985441E0FEEEEFFEEEEFFEE0711EE7711EE771989059321905F681BCF47EA33BBF5E6F8298B5440854FDED1000000000000000D600000300504C5445000000FBAF93F7").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = TransferTransactionV1::deserialize(&payload).unwrap().0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_5() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = TransferTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.recipient_address =
                UnresolvedAddress::from_str("SGEN27LSEJ7MVZYAAAAAAAAAAAAAAAAAAAAAAAA").unwrap();
            tmp_struct.mosaics = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push({
                    let mut tmp_struct = UnresolvedMosaic::default();
                    tmp_struct.mosaic_id = UnresolvedMosaicId(9636553580561478212);
                    tmp_struct.amount = Amount(1);
                    tmp_struct
                });
                tmp_vec.sort_unstable();
                tmp_vec
            };
            tmp_struct.message = "It's some kind of magic, magic".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "7772B114A30ABE8A0CFF2BDEDDB022DD781E0B8503B28CD9B2EA4684EA9CB553",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("3CA4BBA1CFF24DEA27FD659AA48334DB71FF2E377F641E52773959C58B8A3F77E1255762A39097716FCA94CD55FFED106B8B4EFE69701484E05A184A4FEFFD03").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("CE000000000000003CA4BBA1CFF24DEA27FD659AA48334DB71FF2E377F641E52773959C58B8A3F77E1255762A39097716FCA94CD55FFED106B8B4EFE69701484E05A184A4FEFFD037772B114A30ABE8A0CFF2BDEDDB022DD781E0B8503B28CD9B2EA4684EA9CB5530000000001985441E0FEEEEFFEEEEFFEE0711EE7711EE7719188DD7D72227ECAE70000000000000000000000000000001E0001000000000044B262C46CEABB8501000000000000004974277320736F6D65206B696E64206F66206D616769632C206D61676963").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = TransferTransactionV1::deserialize(&payload).unwrap().0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_6() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = TransferTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.recipient_address =
                UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I").unwrap();
            tmp_struct.mosaics = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push({
                    let mut tmp_struct = UnresolvedMosaic::default();
                    tmp_struct.mosaic_id = UnresolvedMosaicId(12342763262823);
                    tmp_struct.amount = Amount(300);
                    tmp_struct
                });
                tmp_vec.push({
                    let mut tmp_struct = UnresolvedMosaic::default();
                    tmp_struct.mosaic_id = UnresolvedMosaicId(95442763262823);
                    tmp_struct.amount = Amount(100);
                    tmp_struct
                });
                tmp_vec.sort_unstable();
                tmp_vec
            };
            tmp_struct.message = "Hello 👋".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "31F9D9D1465206B24E37FCE4F8D6DAC56A0C5B2A3C08A904F8D196442081485B",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("70E5416292032C453B628E6D8D8EFE8EF81C19AA054AD1C270B17E98B0993352B9A2627F5C944E49F01D479F3BB1B263D4516E6C63117DFA35EBBA9D30432EDE").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("CA0000000000000070E5416292032C453B628E6D8D8EFE8EF81C19AA054AD1C270B17E98B0993352B9A2627F5C944E49F01D479F3BB1B263D4516E6C63117DFA35EBBA9D30432EDE31F9D9D1465206B24E37FCE4F8D6DAC56A0C5B2A3C08A904F8D196442081485B0000000001985441E0FEEEEFFEEEEFFEE0711EE7711EE771989059321905F681BCF47EA33BBF5E6F8298B5440854FDED0A00020000000000671305C6390B00002C01000000000000672B0000CE560000640000000000000048656C6C6F20F09F918B").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = TransferTransactionV1::deserialize(&payload).unwrap().0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_7() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = TransferTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.recipient_address =
                UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I").unwrap();
            tmp_struct.mosaics = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push({
                    let mut tmp_struct = UnresolvedMosaic::default();
                    tmp_struct.mosaic_id = UnresolvedMosaicId(8620336746491119575);
                    tmp_struct.amount = Amount(2);
                    tmp_struct
                });
                tmp_vec.push({
                    let mut tmp_struct = UnresolvedMosaic::default();
                    tmp_struct.mosaic_id = UnresolvedMosaicId(15358872602548358953);
                    tmp_struct.amount = Amount(1);
                    tmp_struct
                });
                tmp_vec.push({
                    let mut tmp_struct = UnresolvedMosaic::default();
                    tmp_struct.mosaic_id = UnresolvedMosaicId(7490250818323297978);
                    tmp_struct.amount = Amount(3);
                    tmp_struct
                });
                tmp_vec.sort_unstable();
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "C25CFB2BC786BC391E6930112DE0347796FBFE0636712A6D6E5F0B94850B0085",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("D62C87F5719E3D2AAACB0ADA00678E0FBD040AB7B3D05C30DE7DC613834C45F3C491D61574DF3E368A27895FD494C0F0D83C6D32FA5916E6A7EE1466F4E6E4C6").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("D000000000000000D62C87F5719E3D2AAACB0ADA00678E0FBD040AB7B3D05C30DE7DC613834C45F3C491D61574DF3E368A27895FD494C0F0D83C6D32FA5916E6A7EE1466F4E6E4C6C25CFB2BC786BC391E6930112DE0347796FBFE0636712A6D6E5F0B94850B00850000000001985441E0FEEEEFFEEEEFFEE0711EE7711EE771989059321905F681BCF47EA33BBF5E6F8298B5440854FDED0000030000000000BA36BD286FB7F2670300000000000000D787D9329996A177020000000000000029CF5FD941AD25D50100000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = TransferTransactionV1::deserialize(&payload).unwrap().0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.recipient_address =
                            UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I")
                                .unwrap();
                        tmp_struct.mosaics = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push({
                                let mut tmp_struct = UnresolvedMosaic::default();
                                tmp_struct.mosaic_id = UnresolvedMosaicId(95442763262823);
                                tmp_struct.amount = Amount(100);
                                tmp_struct
                            });
                            tmp_vec.sort_unstable();
                            tmp_vec
                        };
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "3D74E9679725C6117F5DD26B8809D445E4F9A8F193D20D984E2DB674177CB786",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("1DD497AAE2AF93C3E7402AA3623F0578A14646FDA848EF82EB8D14033AB515CECC22B0AD6B9A3C6277B0DC3451C93A534301FFEDF49958E9AC36BE8B684F92A5").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "708124B1E5E63878225B38A343BDB300A1A06150343BA85DFC608331265D0DA5",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("08010000000000001DD497AAE2AF93C3E7402AA3623F0578A14646FDA848EF82EB8D14033AB515CECC22B0AD6B9A3C6277B0DC3451C93A534301FFEDF49958E9AC36BE8B684F92A53D74E9679725C6117F5DD26B8809D445E4F9A8F193D20D984E2DB674177CB7860000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771708124B1E5E63878225B38A343BDB300A1A06150343BA85DFC608331265D0DA56000000000000000600000000000000000000000000000000000000000000000000000000000000000000000000000000000000001985441989059321905F681BCF47EA33BBF5E6F8298B5440854FDED0000010000000000672B0000CE5600006400000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.recipient_address =
                            UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I")
                                .unwrap();
                        tmp_struct.mosaics = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push({
                                let mut tmp_struct = UnresolvedMosaic::default();
                                tmp_struct.mosaic_id = UnresolvedMosaicId(100);
                                tmp_struct.amount = Amount(2);
                                tmp_struct
                            });
                            tmp_vec.push({
                                let mut tmp_struct = UnresolvedMosaic::default();
                                tmp_struct.mosaic_id = UnresolvedMosaicId(200);
                                tmp_struct.amount = Amount(1);
                                tmp_struct
                            });
                            tmp_vec.sort_unstable();
                            tmp_vec
                        };
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "39954C74173AE36309721EC47B4DB9856B91819D78C4F6D7E8F28400B3971866",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("C3CB1685625F7B50CB6CE902EB01C6B87EA219EAA24C4B8C99470228725ACAA913838F8C526ABD8E73C3D07FAE029C7F2952DA60D2C45CAF2CEC06FE6086DB1D").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "F34F69D90B202FC2752058059E3BC49A8CA4BE331D5F49C2C13B8F2A9A3BC331",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("1801000000000000C3CB1685625F7B50CB6CE902EB01C6B87EA219EAA24C4B8C99470228725ACAA913838F8C526ABD8E73C3D07FAE029C7F2952DA60D2C45CAF2CEC06FE6086DB1D39954C74173AE36309721EC47B4DB9856B91819D78C4F6D7E8F28400B39718660000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771F34F69D90B202FC2752058059E3BC49A8CA4BE331D5F49C2C13B8F2A9A3BC3317000000000000000700000000000000000000000000000000000000000000000000000000000000000000000000000000000000001985441989059321905F681BCF47EA33BBF5E6F8298B5440854FDED000002000000000064000000000000000200000000000000C8000000000000000100000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_3() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.recipient_address =
                            UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I")
                                .unwrap();
                        tmp_struct.mosaics = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push({
                                let mut tmp_struct = UnresolvedMosaic::default();
                                tmp_struct.mosaic_id = UnresolvedMosaicId(7490250818323297978);
                                tmp_struct.amount = Amount(3);
                                tmp_struct
                            });
                            tmp_vec.push({
                                let mut tmp_struct = UnresolvedMosaic::default();
                                tmp_struct.mosaic_id = UnresolvedMosaicId(8620336746491119575);
                                tmp_struct.amount = Amount(2);
                                tmp_struct
                            });
                            tmp_vec.push({
                                let mut tmp_struct = UnresolvedMosaic::default();
                                tmp_struct.mosaic_id = UnresolvedMosaicId(15358872602548358953);
                                tmp_struct.amount = Amount(1);
                                tmp_struct
                            });
                            tmp_vec.sort_unstable();
                            tmp_vec
                        };
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "F2FB235F76C92992E81D098A5E09384DDA2107DC44AD09DF2B12985622866D1E",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("C9F0161425259D1D2984B589F7C2C6F2F0B00E1233103F204AD082152C0E1DD549993443C85FDB91C130F8A4CABDF445852E9B2B0AECEB355E57D2BC83B28D84").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "611B90A6E05EE33D30D87DE5B58505B8B9807E54BB8B9229EAF95DBBD43819BC",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("2801000000000000C9F0161425259D1D2984B589F7C2C6F2F0B00E1233103F204AD082152C0E1DD549993443C85FDB91C130F8A4CABDF445852E9B2B0AECEB355E57D2BC83B28D84F2FB235F76C92992E81D098A5E09384DDA2107DC44AD09DF2B12985622866D1E0000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771611B90A6E05EE33D30D87DE5B58505B8B9807E54BB8B9229EAF95DBBD43819BC8000000000000000800000000000000000000000000000000000000000000000000000000000000000000000000000000000000001985441989059321905F681BCF47EA33BBF5E6F8298B5440854FDED0000030000000000BA36BD286FB7F2670300000000000000D787D9329996A177020000000000000029CF5FD941AD25D50100000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_4() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.recipient_address =
                            UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I")
                                .unwrap();
                        tmp_struct.mosaics = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.sort_unstable();
                            tmp_vec
                        };
                        tmp_struct.message = "D600000300504C5445000000FBAF93F7".as_bytes().to_vec();
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "F95B111572CA29B0D1F16445B28506DE9A4E044A5825DFFD1EE8CF2BE1DB35DF",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("E9255429A2B8253E6FBC0FFBBE00222DB8B5FBB5606EE98811B763CE3EE63C81D24365B2C4392389172356B16C43F633A1D76C9F594257AA017C3EEA028F2D2C").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "AB122F570B57922F4B25A37E13EC53E14BE4A6A3F38C06CE4AF510060633667D",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("0801000000000000E9255429A2B8253E6FBC0FFBBE00222DB8B5FBB5606EE98811B763CE3EE63C81D24365B2C4392389172356B16C43F633A1D76C9F594257AA017C3EEA028F2D2CF95B111572CA29B0D1F16445B28506DE9A4E044A5825DFFD1EE8CF2BE1DB35DF0000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771AB122F570B57922F4B25A37E13EC53E14BE4A6A3F38C06CE4AF510060633667D6000000000000000600000000000000000000000000000000000000000000000000000000000000000000000000000000000000001985441989059321905F681BCF47EA33BBF5E6F8298B5440854FDED1000000000000000D600000300504C5445000000FBAF93F7").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_5() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.recipient_address =
                            UnresolvedAddress::from_str("SGEN27LSEJ7MVZYAAAAAAAAAAAAAAAAAAAAAAAA")
                                .unwrap();
                        tmp_struct.mosaics = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push({
                                let mut tmp_struct = UnresolvedMosaic::default();
                                tmp_struct.mosaic_id = UnresolvedMosaicId(9636553580561478212);
                                tmp_struct.amount = Amount(1);
                                tmp_struct
                            });
                            tmp_vec.sort_unstable();
                            tmp_vec
                        };
                        tmp_struct.message = "It's some kind of magic, magic".as_bytes().to_vec();
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "43F974F7DC96445CF6CA73D5644BFA1036F18ADDC1BDC7D17428C601115BBAD0",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("DC057B241AFA2AD72F522AB22299AF9B76AC3C20C8C29278CC1CD15F2F0CD4FA54A7500F66904858C3ABC7C258EA8130F67140C9F6C85EA73502FCDD409F6830").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "9ACF4807E95D6989038C5FCFEA053C55077439DFB93C06C98237C73815CABE87",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("2801000000000000DC057B241AFA2AD72F522AB22299AF9B76AC3C20C8C29278CC1CD15F2F0CD4FA54A7500F66904858C3ABC7C258EA8130F67140C9F6C85EA73502FCDD409F683043F974F7DC96445CF6CA73D5644BFA1036F18ADDC1BDC7D17428C601115BBAD00000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE7719ACF4807E95D6989038C5FCFEA053C55077439DFB93C06C98237C73815CABE8780000000000000007E00000000000000000000000000000000000000000000000000000000000000000000000000000000000000019854419188DD7D72227ECAE70000000000000000000000000000001E0001000000000044B262C46CEABB8501000000000000004974277320736F6D65206B696E64206F66206D616769632C206D616769630000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_6() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.recipient_address =
                            UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I")
                                .unwrap();
                        tmp_struct.mosaics = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push({
                                let mut tmp_struct = UnresolvedMosaic::default();
                                tmp_struct.mosaic_id = UnresolvedMosaicId(12342763262823);
                                tmp_struct.amount = Amount(300);
                                tmp_struct
                            });
                            tmp_vec.push({
                                let mut tmp_struct = UnresolvedMosaic::default();
                                tmp_struct.mosaic_id = UnresolvedMosaicId(95442763262823);
                                tmp_struct.amount = Amount(100);
                                tmp_struct
                            });
                            tmp_vec.sort_unstable();
                            tmp_vec
                        };
                        tmp_struct.message = "Hello 👋".as_bytes().to_vec();
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "A1577B50CA2B7BD0B86E800349547ADB7AC8C43A437317E13944E64AB29C772F",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("287E9D267A8560FC82BCABEB6E161DB461AA585344739B738CA4474F450B3B8A6DA715AC41AF6BEDE4D8F0C3C7C0104F9E8C7FC91AD441E09ACDE33A15DC8A5C").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "754207E883B1237A94D2892613D382C17B0F0A2EC93042871724F6AE0D991ABA",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("2801000000000000287E9D267A8560FC82BCABEB6E161DB461AA585344739B738CA4474F450B3B8A6DA715AC41AF6BEDE4D8F0C3C7C0104F9E8C7FC91AD441E09ACDE33A15DC8A5CA1577B50CA2B7BD0B86E800349547ADB7AC8C43A437317E13944E64AB29C772F0000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771754207E883B1237A94D2892613D382C17B0F0A2EC93042871724F6AE0D991ABA80000000000000007A0000000000000000000000000000000000000000000000000000000000000000000000000000000000000001985441989059321905F681BCF47EA33BBF5E6F8298B5440854FDED0A00020000000000671305C6390B00002C01000000000000672B0000CE560000640000000000000048656C6C6F20F09F918B000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_7() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.recipient_address =
                            UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I")
                                .unwrap();
                        tmp_struct.mosaics = {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push({
                                let mut tmp_struct = UnresolvedMosaic::default();
                                tmp_struct.mosaic_id = UnresolvedMosaicId(8620336746491119575);
                                tmp_struct.amount = Amount(2);
                                tmp_struct
                            });
                            tmp_vec.push({
                                let mut tmp_struct = UnresolvedMosaic::default();
                                tmp_struct.mosaic_id = UnresolvedMosaicId(15358872602548358953);
                                tmp_struct.amount = Amount(1);
                                tmp_struct
                            });
                            tmp_vec.push({
                                let mut tmp_struct = UnresolvedMosaic::default();
                                tmp_struct.mosaic_id = UnresolvedMosaicId(7490250818323297978);
                                tmp_struct.amount = Amount(3);
                                tmp_struct
                            });
                            tmp_vec.sort_unstable();
                            tmp_vec
                        };
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "844D462BE18D5F44D3A6335560C546467E152B71E8FCF892384BCABB1407FFF7",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("A0893317E373F4009C20654644B74253746799124BA671531C1DBBE26DFED167860641CEA467B53139EBDBB4313BD929C0A3838363D10CD71CD882B42DA5EC92").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "611B90A6E05EE33D30D87DE5B58505B8B9807E54BB8B9229EAF95DBBD43819BC",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("2801000000000000A0893317E373F4009C20654644B74253746799124BA671531C1DBBE26DFED167860641CEA467B53139EBDBB4313BD929C0A3838363D10CD71CD882B42DA5EC92844D462BE18D5F44D3A6335560C546467E152B71E8FCF892384BCABB1407FFF70000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771611B90A6E05EE33D30D87DE5B58505B8B9807E54BB8B9229EAF95DBBD43819BC8000000000000000800000000000000000000000000000000000000000000000000000000000000000000000000000000000000001985441989059321905F681BCF47EA33BBF5E6F8298B5440854FDED0000030000000000BA36BD286FB7F2670300000000000000D787D9329996A177020000000000000029CF5FD941AD25D50100000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn VotingKeyLinkTransactionV1_voting_key_link_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = VotingKeyLinkTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.linked_public_key = VotingPublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.linked_public_key = VotingPublicKey::from_str(
                "2132AD40276EDCA2CEC3948326B672F6FC873B9AA49FB22A49AFEA44612C4C96",
            )
            .unwrap();
            tmp_struct.start_epoch = FinalizationEpoch(1);
            tmp_struct.end_epoch = FinalizationEpoch(3);
            tmp_struct.link_action = LinkAction::LINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "11FDBA708E5AC33B56F0F1BCB26095B88F26796CD422DAEFDC5DDF95C35B9D99",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("2E2DA14AA2ED5E08B2BC7636A3F45E84B84C6968B70BB4064E4C8BE04971FBE4A87B64561B4F378D08FB60F24F2DF28932913364D7CFDF09BDDE75C635EB16B1").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("A9000000000000002E2DA14AA2ED5E08B2BC7636A3F45E84B84C6968B70BB4064E4C8BE04971FBE4A87B64561B4F378D08FB60F24F2DF28932913364D7CFDF09BDDE75C635EB16B111FDBA708E5AC33B56F0F1BCB26095B88F26796CD422DAEFDC5DDF95C35B9D990000000001984341E0FEEEEFFEEEEFFEE0711EE7711EE7712132AD40276EDCA2CEC3948326B672F6FC873B9AA49FB22A49AFEA44612C4C96010000000300000001").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = VotingKeyLinkTransactionV1::deserialize(&payload).unwrap().0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn VotingKeyLinkTransactionV1_voting_key_link_single_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = VotingKeyLinkTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.linked_public_key = VotingPublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.linked_public_key = VotingPublicKey::from_str(
                "E787EDC111A3A8E3E8C94DF66755065D66A34563ED09C83755C38C24D3B7F08A",
            )
            .unwrap();
            tmp_struct.start_epoch = FinalizationEpoch(205);
            tmp_struct.end_epoch = FinalizationEpoch(272);
            tmp_struct.link_action = LinkAction::UNLINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "3B7057FDCC87575F139CED264ABA3A9AD0842990CE35D25C5FF84C86E6BC9835",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("4B8726EC5C6F5875707C8CE094880AD4CE0882B34AE4BEFE244C33C2A8FD8A4B4A6A2BDE2B56C84471A69160B1A24B1AD328F86876F39FB4B7D1A2CDB55CA494").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("A9000000000000004B8726EC5C6F5875707C8CE094880AD4CE0882B34AE4BEFE244C33C2A8FD8A4B4A6A2BDE2B56C84471A69160B1A24B1AD328F86876F39FB4B7D1A2CDB55CA4943B7057FDCC87575F139CED264ABA3A9AD0842990CE35D25C5FF84C86E6BC98350000000001984341E0FEEEEFFEEEEFFEE0711EE7711EE771E787EDC111A3A8E3E8C94DF66755065D66A34563ED09C83755C38C24D3B7F08ACD0000001001000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = VotingKeyLinkTransactionV1::deserialize(&payload).unwrap().0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn VotingKeyLinkTransactionV1_voting_key_link_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedVotingKeyLinkTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.linked_public_key =
                            VotingPublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.linked_public_key = VotingPublicKey::from_str(
                            "2132AD40276EDCA2CEC3948326B672F6FC873B9AA49FB22A49AFEA44612C4C96",
                        )
                        .unwrap();
                        tmp_struct.start_epoch = FinalizationEpoch(1);
                        tmp_struct.end_epoch = FinalizationEpoch(3);
                        tmp_struct.link_action = LinkAction::LINK;
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "F2BB1FB10A9560A2A316DEF3920A1BD837235814F652EC6AFB0E3EA9C1FB0EE9",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("D595FEF4F50F3CC47B4D684B1DD9DB0352574D18492BE938E364073B00E4091A1E60354A10753AE92DA6B6935663137B946A82D5B15E8A01F2DD647BC3463017").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "56C4DAA441CE9528C6F0F1431E6FDD78AD33943E568964DF3AADAA9023B97F26",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("0801000000000000D595FEF4F50F3CC47B4D684B1DD9DB0352574D18492BE938E364073B00E4091A1E60354A10753AE92DA6B6935663137B946A82D5B15E8A01F2DD647BC3463017F2BB1FB10A9560A2A316DEF3920A1BD837235814F652EC6AFB0E3EA9C1FB0EE90000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE77156C4DAA441CE9528C6F0F1431E6FDD78AD33943E568964DF3AADAA9023B97F2660000000000000005900000000000000000000000000000000000000000000000000000000000000000000000000000000000000019843412132AD40276EDCA2CEC3948326B672F6FC873B9AA49FB22A49AFEA44612C4C9601000000030000000100000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn VotingKeyLinkTransactionV1_voting_key_link_aggregate_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedVotingKeyLinkTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.linked_public_key =
                            VotingPublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.linked_public_key = VotingPublicKey::from_str(
                            "E787EDC111A3A8E3E8C94DF66755065D66A34563ED09C83755C38C24D3B7F08A",
                        )
                        .unwrap();
                        tmp_struct.start_epoch = FinalizationEpoch(205);
                        tmp_struct.end_epoch = FinalizationEpoch(272);
                        tmp_struct.link_action = LinkAction::UNLINK;
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "E3FDC02C34AF20D6E93DEB083D2150501A2A1B87FC4A412DCEC234884187B936",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("CB5E0C56862D705FDB3AFBDA7399365F0259ECC36377D19FBC154D94C18E337C9183A2B911BE690AF2FAC22C1C65821749E37CF475EFE6C14DEAB991ACBAFE7C").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "7BEFAC9DE1ED91FF6A7F9252CBDF9825C5DEF3D65EBC9CE6D07475854D69978C",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("0801000000000000CB5E0C56862D705FDB3AFBDA7399365F0259ECC36377D19FBC154D94C18E337C9183A2B911BE690AF2FAC22C1C65821749E37CF475EFE6C14DEAB991ACBAFE7CE3FDC02C34AF20D6E93DEB083D2150501A2A1B87FC4A412DCEC234884187B9360000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE7717BEFAC9DE1ED91FF6A7F9252CBDF9825C5DEF3D65EBC9CE6D07475854D69978C6000000000000000590000000000000000000000000000000000000000000000000000000000000000000000000000000000000001984341E787EDC111A3A8E3E8C94DF66755065D66A34563ED09C83755C38C24D3B7F08ACD000000100100000000000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn VrfKeyLinkTransactionV1_vrf_key_link_single_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = VrfKeyLinkTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.linked_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.linked_public_key = PublicKey::from_str(
                "E787EDC111A3A8E3E8C94DF66755065D66A34563ED09C83755C38C24D3B7F08A",
            )
            .unwrap();
            tmp_struct.link_action = LinkAction::LINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "A00FF1F8249F64D7E3A55DA214E5D47BE7A9A906CC4E9D47A934FB8328FA2C06",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("38F64BB69857DF898DF5D551032AA4BBFA454B3235F5915CEBED82C85BE69E7C7D06443763551A4E68CDA17AEC2BF9A74CB5F85A6D0474E7CA7B804F55AF8EDB").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("A10000000000000038F64BB69857DF898DF5D551032AA4BBFA454B3235F5915CEBED82C85BE69E7C7D06443763551A4E68CDA17AEC2BF9A74CB5F85A6D0474E7CA7B804F55AF8EDBA00FF1F8249F64D7E3A55DA214E5D47BE7A9A906CC4E9D47A934FB8328FA2C060000000001984342E0FEEEEFFEEEEFFEE0711EE7711EE771E787EDC111A3A8E3E8C94DF66755065D66A34563ED09C83755C38C24D3B7F08A01").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = VrfKeyLinkTransactionV1::deserialize(&payload).unwrap().0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn VrfKeyLinkTransactionV1_vrf_key_link_single_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = VrfKeyLinkTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.linked_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.linked_public_key = PublicKey::from_str(
                "2132AD40276EDCA2CEC3948326B672F6FC873B9AA49FB22A49AFEA44612C4C96",
            )
            .unwrap();
            tmp_struct.link_action = LinkAction::UNLINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "D1B135019A0C6B06AF89045337CA791B8D0FB8B4AEE6C16331D9EA259C069FDA",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("83FC771045460B0545CA4C27C00A595D21418F34E056F299732CD759C9C0A268D0395222D79F0EEB392D3F5AC0A0FAEAFE231CC0C5F7187F99CAAF74DECC13E3").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
        let payload = decode("A10000000000000083FC771045460B0545CA4C27C00A595D21418F34E056F299732CD759C9C0A268D0395222D79F0EEB392D3F5AC0A0FAEAFE231CC0C5F7187F99CAAF74DECC13E3D1B135019A0C6B06AF89045337CA791B8D0FB8B4AEE6C16331D9EA259C069FDA0000000001984342E0FEEEEFFEEEEFFEE0711EE7711EE7712132AD40276EDCA2CEC3948326B672F6FC873B9AA49FB22A49AFEA44612C4C9600").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = VrfKeyLinkTransactionV1::deserialize(&payload).unwrap().0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn VrfKeyLinkTransactionV1_vrf_key_link_aggregate_1() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedVrfKeyLinkTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.linked_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.linked_public_key = PublicKey::from_str(
                            "E787EDC111A3A8E3E8C94DF66755065D66A34563ED09C83755C38C24D3B7F08A",
                        )
                        .unwrap();
                        tmp_struct.link_action = LinkAction::LINK;
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "DF24940A7CA5734E548A7E2F1F6240CB851B4CC41D96843419F6EFCCAE727767",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("931F2E428A63C16A98CFC5A3EDEC34DE7014133316645C99E1A13C5EF30006D22D255C52C0529286CFD234CF69E9F8DE369F34B45A894649F7595E035977BB39").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "DFED773D9A8101C3DEE6A0F1B8F2D2414FAA3EA509980ED2A6A68DD1F11C32B5",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("0001000000000000931F2E428A63C16A98CFC5A3EDEC34DE7014133316645C99E1A13C5EF30006D22D255C52C0529286CFD234CF69E9F8DE369F34B45A894649F7595E035977BB39DF24940A7CA5734E548A7E2F1F6240CB851B4CC41D96843419F6EFCCAE7277670000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE771DFED773D9A8101C3DEE6A0F1B8F2D2414FAA3EA509980ED2A6A68DD1F11C32B55800000000000000510000000000000000000000000000000000000000000000000000000000000000000000000000000000000001984342E787EDC111A3A8E3E8C94DF66755065D66A34563ED09C83755C38C24D3B7F08A0100000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
    #[test]
    #[allow(non_snake_case)]
    fn VrfKeyLinkTransactionV1_vrf_key_link_aggregate_2() {
        // Serialize Test
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedVrfKeyLinkTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.signer_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.linked_public_key = PublicKey::from_bytes(&[0; 32]).unwrap();
                        tmp_struct.linked_public_key = PublicKey::from_str(
                            "2132AD40276EDCA2CEC3948326B672F6FC873B9AA49FB22A49AFEA44612C4C96",
                        )
                        .unwrap();
                        tmp_struct.link_action = LinkAction::UNLINK;
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "53127C30841F06013376670C302954E74F7170EC871E7E957857A53248C523CA",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("B6D02A1F73CB1F1E5DCF024763E33A837CBC3E36AA8C5CD982620F360B60E05BD9549B1B139E51725BE39DB878F7044CA9AC786EB97163B1D9E81CB74BC39EEF").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "20DBC70A82354FC46E727F9925707398981300E40BE3778FF9EFBE86D722AF59",
            )
            .unwrap();
            tmp_struct
        };
        let payload = decode("0001000000000000B6D02A1F73CB1F1E5DCF024763E33A837CBC3E36AA8C5CD982620F360B60E05BD9549B1B139E51725BE39DB878F7044CA9AC786EB97163B1D9E81CB74BC39EEF53127C30841F06013376670C302954E74F7170EC871E7E957857A53248C523CA0000000002984142E0FEEEEFFEEEEFFEE0711EE7711EE77120DBC70A82354FC46E727F9925707398981300E40BE3778FF9EFBE86D722AF5958000000000000005100000000000000000000000000000000000000000000000000000000000000000000000000000000000000019843422132AD40276EDCA2CEC3948326B672F6FC873B9AA49FB22A49AFEA44612C4C960000000000000000").unwrap();
        assert_eq!(
            (payload.len(), payload.clone()),
            (tx.serialize().len(), tx.serialize())
        );

        // Deserialize Test
        let _ = AggregateBondedTransactionV2::deserialize(&payload)
            .unwrap()
            .0;
    }
}
