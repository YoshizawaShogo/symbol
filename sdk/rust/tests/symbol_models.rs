#[cfg(not(feature = "private_network"))]
mod symbol_models_test {
    use std::str::FromStr;
    use symbol::symbol::prelude::*;
    #[test]
    #[allow(non_snake_case)]
    fn AccountAddressRestrictionTransactionV1_account_address_restriction_single_1() {
        let tx = {
            let mut tmp_struct = AccountAddressRestrictionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
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
                "D294E5E650ACC2A911B548BE2A1806FF4717621BCE3EC1007295219AFFC17B82",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("7695D855CBB6CB83D5BD08E9D76145674F805D741301883387B7101FD8CA84329BB14DBF2F0B4CD58AA84CF31AC6899D134FC38FAB0E7A76F6216ACD60914ACB").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountAddressRestrictionTransactionV1_account_address_restriction_single_2() {
        let tx = {
            let mut tmp_struct = AccountAddressRestrictionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
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
                "6A4E5B14BEDA025A0F12D76FA4391E96FA26D85BE24B3E8C4A08F336ABA1C6F4",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("4E1E910A55162EA9D5E9B17EA6AB357290E97030969C2FAFC18BCF3D73E08827F0CC9A304088742D170E8B3CE1EC4AAAC813F0F7BB6C6BBAFAEBCAE9C23D4327").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountAddressRestrictionTransactionV1_account_address_restriction_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct =
                            EmbeddedAccountAddressRestrictionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "83D1CD2DA160075F016CC04B51397186FEF67006364D851DA9EB0CF3E886E372",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountAddressRestrictionTransactionV1_account_address_restriction_aggregate_2() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct =
                            EmbeddedAccountAddressRestrictionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "1CBF41A4BFD2355911130C4A1CF08AA3A2E4849E5DA5A273545A9D1D1E084AA4",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountKeyLinkTransactionV1_account_key_link_single_1() {
        let tx = {
            let mut tmp_struct = AccountKeyLinkTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.linked_public_key = PublicKey::from_str(
                "F6503F78FBF99544B906872DDB392F4BE707180D285E7919DBACEF2E9573B1E6",
            )
            .unwrap();
            tmp_struct.link_action = LinkAction::LINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "09DF26A84FD347A711C50CEF23C5094F33E4B52435365EA460A894B7785F2483",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("F9197D11A025101D4396A3475EEBD790DC62DC63859C4FEA5DA57BE16D7BF3771AB705D063E05356AD3FBFA344425CAAB47B0831AEBB2D65A08C0C014B110C62").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountKeyLinkTransactionV1_account_key_link_single_2() {
        let tx = {
            let mut tmp_struct = AccountKeyLinkTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.linked_public_key = PublicKey::from_str(
                "9801508C58666C746F471538E43002B85B1CD542F9874B2861183919BA8787B6",
            )
            .unwrap();
            tmp_struct.link_action = LinkAction::UNLINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "8D8B630E0A3086A427DC187452878840ADA021A6D8CACFCF023B7CD172F1B1AB",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("7E87B96BC9C9481B675BF1C0DB8618E4F5AFE5E95EEF7FB37231C252BA76534A1EE392CC2210350F35E7096A43003049ADC71F48556621896014227BC1DDF54F").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountKeyLinkTransactionV1_account_key_link_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedAccountKeyLinkTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.linked_public_key = PublicKey::from_str(
                            "F6503F78FBF99544B906872DDB392F4BE707180D285E7919DBACEF2E9573B1E6",
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
                "6C3AD86C294BCC0244EF2F68D47BA6426FF48C42CB5FF1978139E0256142BACC",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountKeyLinkTransactionV1_account_key_link_aggregate_2() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedAccountKeyLinkTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.linked_public_key = PublicKey::from_str(
                            "9801508C58666C746F471538E43002B85B1CD542F9874B2861183919BA8787B6",
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
                "D571DBBC3B5948FBC7239B34964484AA478046F8BB09B3F3805F853935125E5D",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMetadataTransactionV1_account_metadata_single_1() {
        let tx = {
            let mut tmp_struct = AccountMetadataTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.target_address =
                UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap();
            tmp_struct.scoped_metadata_key = 10;
            tmp_struct.value_size_delta = 10;
            tmp_struct.value = "313233424143".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "FA182D113CBFB48881D3EF7F4CC5BDB29069087E2A7E903093A02D09684A4F94",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("A3204BB3BDBCBFEF5BA954DAB9D6AE784A84B492AA9911B533C381BBB2BBD06A36B4F623A00CA60F7BAF93CCB46441506F469EBBAF4C18352AF548E8315F4B3D").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMetadataTransactionV1_account_metadata_single_2() {
        let tx = {
            let mut tmp_struct = AccountMetadataTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.target_address =
                UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap();
            tmp_struct.scoped_metadata_key = 11258607;
            tmp_struct.value_size_delta = -6;
            tmp_struct.value = "313233424143".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "A679C078A6514E8DC0CA28B1A943D8BA9373AC8B14CCA6B07FEA07ABF9052913",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("03B8387DAA75186536106B847E4AE26213EADCC166A70EAA20C2AF66646D9243D54413EBFA4BB0B614E0ADCAF2417EA350198A26F3DCDBB8B4DACCECC8B1D418").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMetadataTransactionV1_account_metadata_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedAccountMetadataTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "05785D97A5BEB8B77E1F567DA9C2D18048515A01F34E9040EA604C06CF0E6FEC",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMetadataTransactionV1_account_metadata_aggregate_2() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedAccountMetadataTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "8501AD37DE64117F3F9DBCC5751F62FDF60FE3B7BBAD8BF77F94E7F9DAD4438C",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMosaicRestrictionTransactionV1_account_mosaic_restriction_single_1() {
        let tx = {
            let mut tmp_struct = AccountMosaicRestrictionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
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
                "2636536393B22D8EC6DC9E1E2A3F4266839DB16634821789BDFCD5FD51C43B22",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("FD9028F3F1A77147A0A41A0159DC0AD8B13FDA38F7076684F769C1B0BB1CEBED212AA9D6590CE68FB976998D263A2B9C86A744215B35A2EAE02E492E4B788A74").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMosaicRestrictionTransactionV1_account_mosaic_restriction_single_2() {
        let tx = {
            let mut tmp_struct = AccountMosaicRestrictionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
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
                "109184E25B10CD680042331EFCDE6C5BE55659DDD747F83CAA729CAD575C17F3",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("1EB35E0C52602BF150054BDB7B938335A7BB30311C66CEEA869F98CB8808AE214A004AFBEE92B091138C9C7969D08E7B12476C30E182644C3C2A9590BE206F7B").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMosaicRestrictionTransactionV1_account_mosaic_restriction_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct =
                            EmbeddedAccountMosaicRestrictionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "47AE9AFBF35F72617BEC0A990D5BCFCD3651D5E6E4DF51A29E900595A1AF7D1E",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMosaicRestrictionTransactionV1_account_mosaic_restriction_aggregate_2() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct =
                            EmbeddedAccountMosaicRestrictionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "9F088F7A876138C11646E2ECA8ED00FC57AC81F3DC37DEC9991A6959700325ED",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountOperationRestrictionTransactionV1_account_operation_restriction_single_1() {
        let tx = {
            let mut tmp_struct = AccountOperationRestrictionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
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
                "357B615F58B325C42286A51C3E7C797B92135F871D338312D6FCC23BE78FBE13",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("DD7BC1EEFC484BB258024BF0CCEA65E49A83805A63948A60E52F0FD0349C731D1A9F4070FB21C1456FC8C265743BAE84D2D97A9EA3F9A2E4577B5A383C58642D").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountOperationRestrictionTransactionV1_account_operation_restriction_single_2() {
        let tx = {
            let mut tmp_struct = AccountOperationRestrictionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
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
                "20C98B818D42B45707812D9AEDD8DF76D575EEA2C35480D45F1BC7104D4E25CE",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("3BB0589E7608A5BB4A6FC071A1CBD604DBCC4B34AFC46C97674C1AB287192DB41BF3BD7EB77DC7E68F310D4A62B81CB23511834E6BCB21048F4EA9883284D97E").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountOperationRestrictionTransactionV1_account_operation_restriction_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct =
                            EmbeddedAccountOperationRestrictionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "5D500E7BE219C910C07C33560769AD7D1025DA3FB845C646E43D017D201CF800",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountOperationRestrictionTransactionV1_account_operation_restriction_aggregate_2() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct =
                            EmbeddedAccountOperationRestrictionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "896F8F32730913961A94EBB0ED6959434BE790FA8810D86E7FF91774BCB180A5",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn AddressAliasTransactionV1_address_alias_single_1() {
        let tx = {
            let mut tmp_struct = AddressAliasTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.namespace_id = NamespaceId(9562080086528621131);
            tmp_struct.address =
                Address::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap();
            tmp_struct.alias_action = AliasAction::LINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "F1C440FF40F3B9E8E768B9B306E2231B0E349FF83F327E1824A1E5AF333EC2DA",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("A2B62B8383199C1030E1231E9BDB9FA0DA44646E7ADD17C91F9136438DF16D7C629C9B6F017DD47FC0AD066C05E2E71747C7834D188665FE2B1ACC474A27741B").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn AddressAliasTransactionV1_address_alias_single_2() {
        let tx = {
            let mut tmp_struct = AddressAliasTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.namespace_id = NamespaceId(9562080086528621131);
            tmp_struct.address =
                Address::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap();
            tmp_struct.alias_action = AliasAction::UNLINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "61C09C365DE9685B3E93420019F9F4BC853013E1AD95C67E7BBA32DB6C44D1C9",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("128FA50612DF89B1A99A9D2624BEC9957408CCBB0149D82B7F3EB9A7EAC05EB964CE554CA36B86C3776F1B8E584AB6431EC2A1B848B7479A5CBB53049B622186").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn AddressAliasTransactionV1_address_alias_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedAddressAliasTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "BC3361A2FE27D9DF789C5D126475267E37B4F06CC30356E53C55A5FC9933E104",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn AddressAliasTransactionV1_address_alias_aggregate_2() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedAddressAliasTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "9EB78F164D02FE97FE29259E2678586A7BC08E402D14C1444A64A97B394D1F48",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn AggregateBondedTransactionV1_aggregate_bonded_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "0D60B282D0F1A7630D165972F424CDEA90441D5B14497E1333B7F39592532ADC",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("B75CCB8D780D5CEF69C8D0B4F60959DD28537B54EED68588B29483D2871A6D78D988D2684EEF974D04BEDA0BFEE310A9EB4210F65F0552FC79EE1BAAA7E3228E").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push({let mut tmp_struct = Cosignature::default();tmp_struct.signer_public_key = PublicKey::from_str("67fa12789f80766d329c7f687c5c5f889a82f5e9c3e7996ae4ffe48c34299de7").unwrap();tmp_struct.signature = Signature::from_str("622c0ca6cc2ec0c48776fc24bf34fb7f4912b3718457a44d41a32dfcd3dbcedd7d2aa65325ed925e86edeae6ab6ca54ed8b4c0dd090ed9db3860d295da9820ed").unwrap();tmp_struct });
                tmp_vec.push({let mut tmp_struct = Cosignature::default();tmp_struct.signer_public_key = PublicKey::from_str("549676227a2a84f8a555f69892b49a3be02a3b2c71e031e2e8968ebab867c491").unwrap();tmp_struct.signature = Signature::from_str("b3895f21837f76df15b3a6d97fd7ba1dc625011619a5542194ee4220ae34e50c510d942c2c306bc0637ecfc9d9befa907819c6477254fbad11c7a0dddc71b913").unwrap();tmp_struct });
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "61E0F8B9AB2FE3E008DCE1380FECDAF5BCFB1851247BF990771154177A0B7E78",
            )
            .unwrap();
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn AggregateBondedTransactionV2_aggregate_bonded_aggregate_2() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "FDDDB26B9750C36F0C0F06914658E6DC86AE0C323ADBB3520D42DD85138616EB",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("3E080DCE5B32319CA6899808CA664C3961C77A85BB42B192F36394D7B46C79FE4EC2AD6DA50E38836D4ADCDD992C020137F047C1228C351B9533176AB18CE0AF").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push({let mut tmp_struct = Cosignature::default();tmp_struct.signer_public_key = PublicKey::from_str("bd6072e843df052681fe12fcb825cc873c670bec51e73f5b460043677d6b1ebb").unwrap();tmp_struct.signature = Signature::from_str("119db71f2916e710bc2195251d422af0cb2cb378c2f0f2521907f8102912ea38ad3bed2820f6aea6656b0d89e5bda7b2533409864b8a6c961dca9d173ae39979").unwrap();tmp_struct });
                tmp_vec.push({let mut tmp_struct = Cosignature::default();tmp_struct.signer_public_key = PublicKey::from_str("062f6371fd45c2adb840d85b3e7afcb22678365733264291705210a1661c6dc8").unwrap();tmp_struct.signature = Signature::from_str("f55d9667e12f30c7cec0280a51f09f02c26f28e435e1ca1617765fb792c3aaa3350bc8ecd2116b8bdd3fc26e779c2a05dd788f0e59502e92c92dada6c25c6a90").unwrap();tmp_struct });
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "3F2BE873F569828C88CD0DE37BB31C998FA0AAEB3308A1FFBF3D01CE49E8E9F7",
            )
            .unwrap();
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn AggregateCompleteTransactionV1_aggregate_complete_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateCompleteTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.recipient_address =
                            UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ")
                                .unwrap();
                        tmp_struct.mosaics = {
                            let mut tmp_vec = Vec::new();
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
                "E76E5A82B9C5148C740F52913F63DC987028FAECC90046B177B02EF55272419A",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("6ABA6C43AE8B33939CF3452DBFE04525EA9628D0B254A59766AF70B5497BBD82E1859E64570D6441F31AA8BD77693581F42CA59E67B8B86D944D7EBD8D05FAC4").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push({let mut tmp_struct = Cosignature::default();tmp_struct.signer_public_key = PublicKey::from_str("bbc9173f90bdcd2f8f83a387fc24f34f723c359ff422e519669b241cbe0945da").unwrap();tmp_struct.signature = Signature::from_str("af3135255497c49c647ee1021f691d28221d47854708034f7939666a8fb8ad4560f8d734355151bb610be08c9f5d01ea977f1e6b56841ab085dca25effa4a37e").unwrap();tmp_struct });
                tmp_vec.push({let mut tmp_struct = Cosignature::default();tmp_struct.signer_public_key = PublicKey::from_str("58a079ce59f839e4a2a02432ef80f746314a38244fb17566222f7ef3ab6f42b8").unwrap();tmp_struct.signature = Signature::from_str("e17df12fc341ead43586126508d09311a237436f3b2eec79111843b9a50bda9e22d2accafe5dfa67789a15e7be246861b218c8339adb7480f367ed354a523aba").unwrap();tmp_struct });
                tmp_vec.push({let mut tmp_struct = Cosignature::default();tmp_struct.signer_public_key = PublicKey::from_str("34de86b4c43f3a8fae4495fc0832fb156f358f7fb9aa801fb77814229c745e81").unwrap();tmp_struct.signature = Signature::from_str("6b1998d483fc9e20fbc420e197b4213d7f2c14f7e5fc4ba0874a88eef263fcb4f14abcb5d3e144ca61d4fae67b9d13c3c3ae6c4715c0e1faf34df7f9f8c4c9ae").unwrap();tmp_struct });
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "6655F5FCF2290442DD1B3AEBB649A49249E32EBAF259403183A9A847EA22E0B6",
            )
            .unwrap();
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn AggregateCompleteTransactionV2_aggregate_complete_aggregate_2() {
        let tx = {
            let mut tmp_struct = AggregateCompleteTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.recipient_address =
                            UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ")
                                .unwrap();
                        tmp_struct.mosaics = {
                            let mut tmp_vec = Vec::new();
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
                "38CF4625210D1D38D6E9A8DC901D49D659ADC793C0AE5F488E52DC29A292B5A7",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("BA5888AF968D00F57A9019A58421897EFC4863BEB8EF9B20C765B50B839DCFE4501FB0CD839CEF8EF43F4DD59CEB78BD8A80B011D9E8B577418C6415FFC7FA3D").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct.cosignatures = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push({let mut tmp_struct = Cosignature::default();tmp_struct.signer_public_key = PublicKey::from_str("264e45b83fcf538b9b58ccf252bd39486a6d1b139300efd2db357ce4ec225cb4").unwrap();tmp_struct.signature = Signature::from_str("7ce0e4577928b9f42f83453d13819a0e762adb37bda14ca6c7b773acb4dce3912b332087402cab4e5c52c3c8f58ff56c09af15fdf6592492aa5720852f8546e9").unwrap();tmp_struct });
                tmp_vec.push({let mut tmp_struct = Cosignature::default();tmp_struct.signer_public_key = PublicKey::from_str("00a0437049f578c2c64b9bea3e6d19bd2a5b521f8447749b2d6006b188e32a04").unwrap();tmp_struct.signature = Signature::from_str("cd18f8c52d0bfb1cc1c35d359ba2856dc956681e4ff1d72d2e1eeae280c1f8ccb2b8d1ce44f9760ee0985c5ff32e49e6159b7a249056d2f8549f31bd0477141f").unwrap();tmp_struct });
                tmp_vec.push({let mut tmp_struct = Cosignature::default();tmp_struct.signer_public_key = PublicKey::from_str("188cb4361e1e76f98cf3e4d313f5eaa202582f2823eb8a92aec3ef71e792090f").unwrap();tmp_struct.signature = Signature::from_str("5ba52f0194c2b04fbb92ef1ee80feec0cf2a4d02fc0bd39817f3b8228343797c88493416fe316460124f89eee6f32a047f59b2937c85f3f3a4b5973465fb71f4").unwrap();tmp_struct });
                tmp_vec
            };
            tmp_struct.transactions_hash = Hash256::from_str(
                "DCE7DC355A58AEDC834B89C2E3D42DD07DBB8C9167A046856CA56EBE4EEE5AC2",
            )
            .unwrap();
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn HashLockTransactionV1_hash_lock_single_1() {
        let tx = {
            let mut tmp_struct = HashLockTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
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
                "AC09CC13AF31045A0AF5D0427B219E6336D29375A4ACB5365ECDDE434F3E5795",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("DCE85092A4AA448260E9C849FBC5FA51CA92BF90DFD1831FBFBE44D0B7FB4973E243B0D651CD5DC0EE35EC60472C1598C0BF182B344FD80D26938E3DFF5F9491").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn HashLockTransactionV1_hash_lock_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedHashLockTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "0A56922477C082CCB79B3994BCB6639B952167882A129905C81E0262B49450A5",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicAddressRestrictionTransactionV1_mosaic_address_restriction_single_1() {
        let tx = {
            let mut tmp_struct = MosaicAddressRestrictionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.mosaic_id = UnresolvedMosaicId(1);
            tmp_struct.restriction_key = 1311768467294898927;
            tmp_struct.previous_restriction_value = 9;
            tmp_struct.new_restriction_value = 8;
            tmp_struct.target_address =
                UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "13F9FD5838B8F8AC48E7526D508250E581D7ADA7E304162F334CB03A0D556E04",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("D540747095A39055383EA6A199959BE21A43DC6324DFD215EBE2888904D6F5D6F61D259D84456DC6D731DABBCFD26C747E4A80970D56C1741D82FFE9CDB0E540").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicAddressRestrictionTransactionV1_mosaic_address_restriction_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct =
                            EmbeddedMosaicAddressRestrictionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "BBAA2C0D6489F346888202F8860674C70858ED2ED33B6CCC4C16865543520CFF",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicAliasTransactionV1_mosaic_alias_single_1() {
        let tx = {
            let mut tmp_struct = MosaicAliasTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.namespace_id = NamespaceId(13182596108967839652);
            tmp_struct.mosaic_id = MosaicId(10);
            tmp_struct.alias_action = AliasAction::LINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "64C9E56A3D00C0AE070BF06A400BDFEB829B00CCAB9F0ADF1A229A308EC5EB4A",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("8D5BDBFC1344DA6738E928C2547B8422D34AAB8AA80E77E9657AEC80937DA19D782F837545CFF48DD4880D08C35B7C39119B9F75F3E50DFAB0D917D4D2598BF0").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicAliasTransactionV1_mosaic_alias_single_2() {
        let tx = {
            let mut tmp_struct = MosaicAliasTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.namespace_id = NamespaceId(16233676262248077354);
            tmp_struct.mosaic_id = MosaicId(14624838436596993100);
            tmp_struct.alias_action = AliasAction::UNLINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "DD5CFC65BCEDBF0E91E4DFE20B5E1598106DD30EC59997C6E8028DBAE9910D63",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("6B1750C3D6F272C316EBB3916E177F2DAF2F6837CF43201A631059D02EC0FFCC554C2C64E9AB10F6B154EFE152DAAA04CCA11082DB6E81EA411E7E416A298142").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicAliasTransactionV1_mosaic_alias_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedMosaicAliasTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "114DDC0523930DC9F7B360CECF4DFDD5A5DF9C2B06709C577EAACCC58118AEA3",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicAliasTransactionV1_mosaic_alias_aggregate_2() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedMosaicAliasTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "A2A8877BEA72600C4C83B18067F721CA8C09370ADF3445A289FD6FA69DE0B999",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicDefinitionTransactionV1_mosaic_definition_single_1() {
        let tx = {
            let mut tmp_struct = MosaicDefinitionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.duration = BlockDuration(10000);
            tmp_struct.nonce = MosaicNonce(0);
            tmp_struct.flags = MosaicFlags::RESTRICTABLE | MosaicFlags::SUPPLY_MUTABLE;
            tmp_struct.divisibility = 4;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "70F08E77C31D9816C0A63009137A9BADBE5F42431EFBC3994822A25CED9D8282",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("5D3116D285D4ED8883DBBBC8E59FED08A888DAB21C6E4B918434BE2B3AF1105EE1B94EAA9C4BB54428F4A71C711964F00848B9A9E00D8F55670991AADC16119F").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicDefinitionTransactionV1_mosaic_definition_single_2() {
        let tx = {
            let mut tmp_struct = MosaicDefinitionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.duration = BlockDuration(1000);
            tmp_struct.nonce = MosaicNonce(3095715558);
            tmp_struct.flags = MosaicFlags::NONE;
            tmp_struct.divisibility = 3;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "FA59F3C0267DA5F9A11BBC9714B19402172CD1834F42CC4D2699301437B6BF0D",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("D3B6BEF55F6D99281079B8A138EECE9A4CACC052BC3E84D83D72C3FCF0CFA85DEA390B8FCD50F1A6A6E196DDDED52CB92FC3C216C6B5F06F96E89B23FA62B4BE").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicDefinitionTransactionV1_mosaic_definition_single_3() {
        let tx = {
            let mut tmp_struct = MosaicDefinitionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.duration = BlockDuration(0);
            tmp_struct.nonce = MosaicNonce(3095715558);
            tmp_struct.flags = MosaicFlags::REVOKABLE | MosaicFlags::TRANSFERABLE;
            tmp_struct.divisibility = 2;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "1138798330AB1EDF113867A0A03285FBFC8D433F7F688B41B33D01C7939086D5",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("3712BC4F3932457AD1A7CC967CC45C3D5F04A52F6B802AEC7D377E504432F1DA40DD1EDAFE9F5899BD04DFBFB1324B198CCEE3344883DEA75DCCE2D1778B6529").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicDefinitionTransactionV1_mosaic_definition_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedMosaicDefinitionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "9408A21018F5FA6205D0A5D1A99DC3BF7295D686460569F6FC7BCD9D392F2E9E",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicDefinitionTransactionV1_mosaic_definition_aggregate_2() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedMosaicDefinitionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "D962535DE9E97E0CB8E878AA0B68B2D601D20D0BB2A46AE06C086FFED4342DFE",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicDefinitionTransactionV1_mosaic_definition_aggregate_3() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedMosaicDefinitionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "C1F2E14CBF743FE0A0FC27482BA97B4734B5F80BED8C6642B075EB421F9E2F81",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicGlobalRestrictionTransactionV1_mosaic_global_restriction_single_1() {
        let tx = {
            let mut tmp_struct = MosaicGlobalRestrictionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.mosaic_id = UnresolvedMosaicId(1875072453572000775);
            tmp_struct.reference_mosaic_id = UnresolvedMosaicId(7706325451784159270);
            tmp_struct.restriction_key = 1;
            tmp_struct.previous_restriction_value = 9;
            tmp_struct.new_restriction_value = 8;
            tmp_struct.previous_restriction_type = MosaicRestrictionType::EQ;
            tmp_struct.new_restriction_type = MosaicRestrictionType::GE;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "0A72FF40C5C259ACCDFA578E3409242DE4DED94C84C43A11A8D3F9EC8FD773DE",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("A70ECFC4FAD876EDD481D02AF560DBC319E6AAB21DD33A9095BD45B1A5994844527F5DDBE7C10AE28D960436ACD0D6076D3D9F7ABE9473832F2839FB3370B95A").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicGlobalRestrictionTransactionV1_mosaic_global_restriction_single_2() {
        let tx = {
            let mut tmp_struct = MosaicGlobalRestrictionTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.mosaic_id = UnresolvedMosaicId(6207017352306769745);
            tmp_struct.reference_mosaic_id = UnresolvedMosaicId(0);
            tmp_struct.restriction_key = 4444;
            tmp_struct.previous_restriction_value = 0;
            tmp_struct.new_restriction_value = 0;
            tmp_struct.previous_restriction_type = MosaicRestrictionType::NONE;
            tmp_struct.new_restriction_type = MosaicRestrictionType::GE;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "AB12A93BCCCDB042E487A653D7E975EA1DA739FA886B8EFF7833C2CA878C96FB",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("BB9436193DB00910693878E9530966643BAB80AF00C026FD3FD85327422707AD2E5F21B890C22220BC510301F5DC8DE7FAC2445F7022B4B8DEDC5D751E95ADF1").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicGlobalRestrictionTransactionV1_mosaic_global_restriction_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct =
                            EmbeddedMosaicGlobalRestrictionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "5B47A3C1ECE8210626CA7638F00C38DAC6697E3FAC0461D7C9ECDF5BEB4C4D85",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicGlobalRestrictionTransactionV1_mosaic_global_restriction_aggregate_2() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct =
                            EmbeddedMosaicGlobalRestrictionTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "26646FC63C7D7EA9EAA0B678CA7BDB1ECD77C7D210ACD997A8713BAB88D6A8C1",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicMetadataTransactionV1_mosaic_metadata_single_1() {
        let tx = {
            let mut tmp_struct = MosaicMetadataTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.target_address =
                UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap();
            tmp_struct.scoped_metadata_key = 10;
            tmp_struct.target_mosaic_id = UnresolvedMosaicId(1000);
            tmp_struct.value_size_delta = 10;
            tmp_struct.value = "313233414243".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "D7E3F96B4A418E8E2FDF8A45B7C34B3A354ECC07C0175763727C1FBA0B1736AC",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("21F6DF84B68468A19A0E204EFC45A826C02991737D0C3334F42CB64928D9537886359B83316B16060A859A5A2C1819CBC36FF520DF5F17D1529240F256CEA94C").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicMetadataTransactionV1_mosaic_metadata_single_2() {
        let tx = {
            let mut tmp_struct = MosaicMetadataTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.target_address =
                UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap();
            tmp_struct.scoped_metadata_key = 10;
            tmp_struct.target_mosaic_id = UnresolvedMosaicId(1000);
            tmp_struct.value_size_delta = -5;
            tmp_struct.value = "313233414243".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "A7FB98F0A1BE958F2D70BC8609A3539DCBD702978FC872C2D388A634FBEA074C",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("EE4682A5FBE4BA1C8F8131DF3C0DF7BE4E8BAF0E3A2B2D288101F2C5261932F03E02FDC4207B5FD7E44A4771E6D3895388213C48789982B42AF05CDEB7F88E26").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicMetadataTransactionV1_mosaic_metadata_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedMosaicMetadataTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "1D00C6B79C08776387DBFAB93D0DEBB09D6F2E1BE8E08BD206A35594CD153172",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicMetadataTransactionV1_mosaic_metadata_aggregate_2() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedMosaicMetadataTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "9F96414C1542444D215F3C971002101B031334A54DED6843C59B5CB86E164F01",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicSupplyChangeTransactionV1_mosaic_supply_change_single_1() {
        let tx = {
            let mut tmp_struct = MosaicSupplyChangeTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.mosaic_id = UnresolvedMosaicId(6300565133566699912);
            tmp_struct.action = MosaicSupplyChangeAction::INCREASE;
            tmp_struct.delta = Amount(10);
            tmp_struct.signer_public_key = PublicKey::from_str(
                "F37F9FB1AFC9F6737DE4652755E6251E66ACB1FCE0A767B62F6E9DA4235E6960",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("E2170899C9BFFDB63EA730C1EE0AA60A9AB086C9127242101ACF0DEFCEA8A31D9B4CA37B6644AC2B6928527338C1CB2C87EA4ADBD98A9EFAC34430B9245C6F93").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicSupplyChangeTransactionV1_mosaic_supply_change_single_2() {
        let tx = {
            let mut tmp_struct = MosaicSupplyChangeTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.mosaic_id = UnresolvedMosaicId(14624838436596993100);
            tmp_struct.action = MosaicSupplyChangeAction::DECREASE;
            tmp_struct.delta = Amount(10);
            tmp_struct.signer_public_key = PublicKey::from_str(
                "1412C49C8BB197DBDEDAB0D12AF4C24A653707369B24C995F924E78D55964BB9",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("575F393E380C091DCC5729454D7B839D52158AD00CFD07A735F385DBC0574266EAD33478F15B0F38788437B0F9249A4732808002E23ADC95B9BA1F3F1B86A222").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicSupplyChangeTransactionV1_mosaic_supply_change_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedMosaicSupplyChangeTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "95D4073C8B7C43F263AAE6E1C8615C41019E2B4FC4AA2693191FC8E693CB4D5D",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicSupplyChangeTransactionV1_mosaic_supply_change_aggregate_2() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedMosaicSupplyChangeTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "8661A3B2F2D23C954942249029CA29E0ABBDDB19B15F73D8560472FCD797EAD4",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn MultisigAccountModificationTransactionV1_multisig_account_modification_single_1() {
        let tx = {
            let mut tmp_struct = MultisigAccountModificationTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
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
                "0499B08886DED0962C696C707C0964588FE4B6C3BC82BECF9F1F63257ED4CB87",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("4E30A6E6477467C8314BEFF4922D58C33ED32AF351DD88640AF200EB4EE9C6FAD92B42D7FA236485F99D4D2C253993A66B2B00454A1159E71CBB3EB51394AC67").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn MultisigAccountModificationTransactionV1_multisig_account_modification_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct =
                            EmbeddedMultisigAccountModificationTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "3AFC562C7601C865F7F473063BDA62E72D9EA4802B111F302D016A699DFB4B08",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceMetadataTransactionV1_namespace_metadata_single_1() {
        let tx = {
            let mut tmp_struct = NamespaceMetadataTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.target_address =
                UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I").unwrap();
            tmp_struct.scoped_metadata_key = 10;
            tmp_struct.target_namespace_id = NamespaceId(1000);
            tmp_struct.value_size_delta = 10;
            tmp_struct.value = "ABC123".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "2D64F92F75B37994557B14A982218F4A8B7D6B9CFFBBADFB259BAEC0F9434F0F",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("067D7153D66ED03E696208DAFB698C1EC0ECD92DA3AFEC180E082FA84133F1E5B9B0F0ACD14CEBE867DBA15DD37CB9CC413AAB3EF73E9929977337E6A8F2AB44").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceMetadataTransactionV1_namespace_metadata_single_2() {
        let tx = {
            let mut tmp_struct = NamespaceMetadataTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.target_address =
                UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I").unwrap();
            tmp_struct.scoped_metadata_key = 10;
            tmp_struct.target_namespace_id = NamespaceId(1000);
            tmp_struct.value_size_delta = -3;
            tmp_struct.value = "ABC123".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "72D6ECEC68081903BB300BC0C033139CDB18D41EA98F9922CF20A5E1FB6B02D6",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("FD0DDFDC372E15AE261A4AF8C61EFE37FAE5E4D6D8B6E53AA83ED616BD002C1700D2B594C841472C3DC24E4B74DE5E01968A943F8AE7BC34B9C59C9918DA2A46").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceMetadataTransactionV1_namespace_metadata_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedNamespaceMetadataTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "DB14CF9C0521579EB44C4FCFC30BFB92CCEC6BFD25FE506E0A8BB8144B8D73B6",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceMetadataTransactionV1_namespace_metadata_aggregate_2() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedNamespaceMetadataTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "F786C045005CD01DE3ABC781B29FE97E0DC7C863AEF29BB5BB16D9D309678E8B",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceRegistrationTransactionV1_namespace_registration_single_1() {
        let tx = {
            let mut tmp_struct = NamespaceRegistrationTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.duration = BlockDuration(10000);
            tmp_struct.id = NamespaceId(13858666424160217470);
            tmp_struct.registration_type = NamespaceRegistrationType::ROOT;
            tmp_struct.name = "newnamespace".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "15A62A582DA8A52B13BB59EBE39FF2E4155FA2C822CBB0268BDDE5FA00F4F8FF",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("6F81F080720F6F641386F1320BCD4B641345CA1D3FF4D7DE302B0EA28D0E8869F3FCC0BACD72C3FF897CB620ED6B713B07F68B6312428A3C6C09B88FCAD0789A").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceRegistrationTransactionV1_namespace_registration_single_2() {
        let tx = {
            let mut tmp_struct = NamespaceRegistrationTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.parent_id = NamespaceId(4635294387305441662);
            tmp_struct.id = NamespaceId(17411894141110456835);
            tmp_struct.registration_type = NamespaceRegistrationType::CHILD;
            tmp_struct.name = "subnamespace".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "00C8921E7F8A214345AC3A2E15FA9651622A4FA7E609FC6BDE2E79063DCBD336",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("59C951AD8691705F1EB49D80B78B850B4114F38E0FCC64DAC404E9AA44DCBAA8A3DCFE82DF1275E278F8B8C98D3B83FB6328F257937AD4490B944C4AE27904B3").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceRegistrationTransactionV1_namespace_registration_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedNamespaceRegistrationTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "C3327284E6AC67B1A558F95797CF2EFC994BCECA4EBBCCB4592CB6B8F645DC2D",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceRegistrationTransactionV1_namespace_registration_aggregate_2() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedNamespaceRegistrationTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "2491954E840A79E330BD295F8FF0A15863384734583B9AB6E83815AF9438C086",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn NodeKeyLinkTransactionV1_node_key_link_single_1() {
        let tx = {
            let mut tmp_struct = NodeKeyLinkTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.linked_public_key = PublicKey::from_str(
                "9801508C58666C746F471538E43002B85B1CD542F9874B2861183919BA8787B6",
            )
            .unwrap();
            tmp_struct.link_action = LinkAction::LINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "63F4B6B964EE70A2980C5BB57DB2C184F8DDBF9B0BD9E72F89EA55AC9C5BA6BF",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("3CCE9BCD544BFF665A3400F7337A5115307ABB490AD821B6EE8F2906805B4B4C7D525EC20B52B9F6D7FEAA0CC6C20E6A613F2395916AC07F4ACC34FAD57F177D").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn NodeKeyLinkTransactionV1_node_key_link_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedNodeKeyLinkTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.linked_public_key = PublicKey::from_str(
                            "9801508C58666C746F471538E43002B85B1CD542F9874B2861183919BA8787B6",
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
                "F13328C38A0C8B9DE194EAB1609872BAA51FF677395513A528A707795E3C2F59",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn SecretLockTransactionV1_secret_lock_single_1() {
        let tx = {
            let mut tmp_struct = SecretLockTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
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
                "21EDDC0F3547F2FE431C1B2BAA531E7C299CD1CBE3F410112C74F43DDAD2A578",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("5E9808E11624AAABD2826CF7F464B93F309B8F15506BE6FC7E8C1E5E09E23B4D13A37C5982225413DDD6CA5913F4F4673662732059AD381DF191A01C72CB6D5D").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn SecretLockTransactionV1_secret_lock_single_2() {
        let tx = {
            let mut tmp_struct = SecretLockTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
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
                "ED5139E46E6BECCD9E762EBFCA2A534DE476087621E2EA3A0BD9E42743A2B4AF",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("1F97199EAA5A7B3D956DE51DB9E93490A72123ECDC7C2931ED4B3EA9D02FD9443F9F5028B92D5CF5A32DD1F9802D0D5B703BE5FFFDB3480D0915C8BE7ABE62FD").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn SecretLockTransactionV1_secret_lock_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedSecretLockTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "8B23AE138D05DFC8CCEC32BA748D55766D1053859E2F403EAA30D692C3CB822F",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn SecretLockTransactionV1_secret_lock_aggregate_2() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedSecretLockTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "EC851E578B1AEF681A6FBE5CD8682D12517F69142EF25B27F9CE75F0087E1F01",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn SecretProofTransactionV1_secret_proof_single_1() {
        let tx = {
            let mut tmp_struct = SecretProofTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.recipient_address =
                UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I").unwrap();
            tmp_struct.secret = Hash256::from_str(
                "3FC8BA10229AB5778D05D9C4B7F56676A88BF9295C185ACFC0F961DB5408CAFE",
            )
            .unwrap();
            tmp_struct.hash_algorithm = LockHashAlgorithm::SHA3_256;
            tmp_struct.proof = "9A493664".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "CC619039AF06C01EE2082F40F9D0F4626B8EDB093303FAAC9B15AE5D9797FAE5",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("AF3DC7E901D3DBA59F26DB495339E55466C70DDAFD4993CFA437CC260C5829774A3A8891758C20D1E4432D53C9B23FD500972FB212325CC0160300BEE521B444").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn SecretProofTransactionV1_secret_proof_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedSecretProofTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "0F6E0AF656247EBE417B0BF2A910BCB879429D9167B274833DA2BCC7526097F0",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_1() {
        let tx = {
            let mut tmp_struct = TransferTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
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
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "A75027E4F32570A79B8A5A8641AB91ED48360074AE2AAE055CE3BD48D3BE2233",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("2396B87D65DDDCF52F527CC4C8E2C413C52DA4E2D2D951E5EB1370941D86068688099761AD473A3D124650B823C39078B9326EC8CD050FE2EB6ABC9FE61C0212").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_2() {
        let tx = {
            let mut tmp_struct = TransferTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
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
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "B0A48186B2D8C168DBAF2395AD3BF421F9E44D7AD8E616C5E981ABD1DB5190F2",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("42D030CD0166DA62C1DF1FF0945752475FBD2B4B975E9991EFF57BCD742C235787433B8AF428C3852009C8C63B632572057945118F393F4187FF51DFD77CAC6D").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_3() {
        let tx = {
            let mut tmp_struct = TransferTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
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
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "5A76A15971385920F91E666BD0698687C0A5C50D6FCEE82E9F1FDC4D8BC7F518",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("C1DD9E45551CF35D8F058C73A8E3813B107A5D6EC6393F60B8B2F294E1C831FF96F30CB71D18EBEE2C96146D97DF1CFA252B8B3988697015150D7CDFEF884463").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_4() {
        let tx = {
            let mut tmp_struct = TransferTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.recipient_address =
                UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I").unwrap();
            tmp_struct.mosaics = {
                let mut tmp_vec = Vec::new();
                tmp_vec
            };
            tmp_struct.message = "D600000300504C5445000000FBAF93F7".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "2558851FBDECC3CFAD26DE0050EAA6661B81F2CCE59F3A6672766CD8EC8EE3AA",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("5A5763BD9CE487F745C0A5F4D2D2F4167778878C9C119B03C549F915ED471B6AD05F51A76C4CE9CC7BCF58958A6DC64B3C43584D1651B64FBBFCD42FCAD1DEBF").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_5() {
        let tx = {
            let mut tmp_struct = TransferTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
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
                tmp_vec
            };
            tmp_struct.message = "It's some kind of magic, magic".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "11EB0CF6770160DFED4C943B9B691930D3F138141FF4D02B7CB8B383A3AE2BDA",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("3CA4BBA1CFF24DEA27FD659AA48334DB71FF2E377F641E52773959C58B8A3F77E1255762A39097716FCA94CD55FFED106B8B4EFE69701484E05A184A4FEFFD03").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_6() {
        let tx = {
            let mut tmp_struct = TransferTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
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
                tmp_vec
            };
            tmp_struct.message = "Hello 👋".as_bytes().to_vec();
            tmp_struct.signer_public_key = PublicKey::from_str(
                "0E24F92F57FF40F89B2F28B0A89B1F1A55408A653C1864502F29744237EFA2B6",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("70E5416292032C453B628E6D8D8EFE8EF81C19AA054AD1C270B17E98B0993352B9A2627F5C944E49F01D479F3BB1B263D4516E6C63117DFA35EBBA9D30432EDE").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_7() {
        let tx = {
            let mut tmp_struct = TransferTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
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
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "E66203800A937D2CDF45D3C62C30DEE4A0FEA810B958DD870EBB05CC97BCC382",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("D62C87F5719E3D2AAACB0ADA00678E0FBD040AB7B3D05C30DE7DC613834C45F3C491D61574DF3E368A27895FD494C0F0D83C6D32FA5916E6A7EE1466F4E6E4C6").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                            tmp_vec
                        };
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "8881DFC77FE896CB10A98B2CD52660C243C81185BAE7C54187E5BCF84CE849E8",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_2() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                            tmp_vec
                        };
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "20D7B516D60461DDBDBF41E29F111DAB28951B5E1E257B560889766C7F210D62",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_3() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                            tmp_vec
                        };
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "A311444F2A64E4E1D9864C117076FA4AA1629FC011AD6103B2FB7FC614F60EDE",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_4() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.recipient_address =
                            UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I")
                                .unwrap();
                        tmp_struct.mosaics = {
                            let mut tmp_vec = Vec::new();
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
                "7C4A1D083B98CCA7D7A7F467B1ECCE4CD8ED1B3D4615724B590B07E852B4A89C",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_5() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "6AA821C54CAC407F32028BABE0D2B29DC3EE7F89078B2EC40705C7EAA123C4E7",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_6() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                "3A347999434B7D11999E74880393A38AA0EAD0D5290B16B665E9408F7DC7CABA",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_7() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedTransferTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
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
                            tmp_vec
                        };
                        tmp_struct
                    }
                    .into(),
                );
                tmp_vec
            };
            tmp_struct.signer_public_key = PublicKey::from_str(
                "9FC9DA6EB5B2E47C0AB55B8B7DEBEC331DFDD4C4A7E748ED35A59D69E29B639B",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn VotingKeyLinkTransactionV1_voting_key_link_single_1() {
        let tx = {
            let mut tmp_struct = VotingKeyLinkTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.linked_public_key = VotingPublicKey::from_str(
                "C614558647D02037384A2FECA80ACE95B235D9B9D90035FA46102FE79ECCBA75",
            )
            .unwrap();
            tmp_struct.start_epoch = FinalizationEpoch(1);
            tmp_struct.end_epoch = FinalizationEpoch(3);
            tmp_struct.link_action = LinkAction::LINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "7163EBCDA4CACDCAE3A1145D22FA73E0AA8AFFB46F5E14D9205DCF98951FFA48",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("2E2DA14AA2ED5E08B2BC7636A3F45E84B84C6968B70BB4064E4C8BE04971FBE4A87B64561B4F378D08FB60F24F2DF28932913364D7CFDF09BDDE75C635EB16B1").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn VotingKeyLinkTransactionV1_voting_key_link_single_2() {
        let tx = {
            let mut tmp_struct = VotingKeyLinkTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.linked_public_key = VotingPublicKey::from_str(
                "9801508C58666C746F471538E43002B85B1CD542F9874B2861183919BA8787B6",
            )
            .unwrap();
            tmp_struct.start_epoch = FinalizationEpoch(205);
            tmp_struct.end_epoch = FinalizationEpoch(272);
            tmp_struct.link_action = LinkAction::UNLINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "880B6E4974E521B5B448C41BFD54F6F316D7707CCC6D07006DB993A350B83DAF",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("4B8726EC5C6F5875707C8CE094880AD4CE0882B34AE4BEFE244C33C2A8FD8A4B4A6A2BDE2B56C84471A69160B1A24B1AD328F86876F39FB4B7D1A2CDB55CA494").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn VotingKeyLinkTransactionV1_voting_key_link_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedVotingKeyLinkTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.linked_public_key = VotingPublicKey::from_str(
                            "C614558647D02037384A2FECA80ACE95B235D9B9D90035FA46102FE79ECCBA75",
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
                "E31C85F00E72BB7537B1D055E2F8B1563143080AA60C3A9653FA2A94AD5958A9",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn VotingKeyLinkTransactionV1_voting_key_link_aggregate_2() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedVotingKeyLinkTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.linked_public_key = VotingPublicKey::from_str(
                            "9801508C58666C746F471538E43002B85B1CD542F9874B2861183919BA8787B6",
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
                "06EE91EA9B49139264D5AD874CC71D53EA5010A6033A951A163C46CB00718C8C",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn VrfKeyLinkTransactionV1_vrf_key_link_single_1() {
        let tx = {
            let mut tmp_struct = VrfKeyLinkTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.linked_public_key = PublicKey::from_str(
                "9801508C58666C746F471538E43002B85B1CD542F9874B2861183919BA8787B6",
            )
            .unwrap();
            tmp_struct.link_action = LinkAction::LINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "F338E25ED94FC0376305E1B337BF00D56F8B012A88D5E9E18DF13815A1AB1C89",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("38F64BB69857DF898DF5D551032AA4BBFA454B3235F5915CEBED82C85BE69E7C7D06443763551A4E68CDA17AEC2BF9A74CB5F85A6D0474E7CA7B804F55AF8EDB").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn VrfKeyLinkTransactionV1_vrf_key_link_single_2() {
        let tx = {
            let mut tmp_struct = VrfKeyLinkTransactionV1::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.linked_public_key = PublicKey::from_str(
                "C614558647D02037384A2FECA80ACE95B235D9B9D90035FA46102FE79ECCBA75",
            )
            .unwrap();
            tmp_struct.link_action = LinkAction::UNLINK;
            tmp_struct.signer_public_key = PublicKey::from_str(
                "41F56C0A08B535092BDFF114360B5563B729B9BB82B727238D857D9FFF705B1E",
            )
            .unwrap();
            tmp_struct.signature = Signature::from_str("83FC771045460B0545CA4C27C00A595D21418F34E056F299732CD759C9C0A268D0395222D79F0EEB392D3F5AC0A0FAEAFE231CC0C5F7187F99CAAF74DECC13E3").unwrap();
            tmp_struct.fee = Amount(18370164183782063840);
            tmp_struct.deadline = Timestamp(8207562320463688160);
            tmp_struct
        };
    }
    #[test]
    #[allow(non_snake_case)]
    fn VrfKeyLinkTransactionV1_vrf_key_link_aggregate_1() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedVrfKeyLinkTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.linked_public_key = PublicKey::from_str(
                            "9801508C58666C746F471538E43002B85B1CD542F9874B2861183919BA8787B6",
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
                "633071BA1815E88BE471CE8C8B930BE7771C637A7209B6D04523BD35ABF08952",
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
    }
    #[test]
    #[allow(non_snake_case)]
    fn VrfKeyLinkTransactionV1_vrf_key_link_aggregate_2() {
        let tx = {
            let mut tmp_struct = AggregateBondedTransactionV2::default();
            tmp_struct.network = NetworkType::TESTNET;
            tmp_struct.transactions = {
                let mut tmp_vec = Vec::new();
                tmp_vec.push(
                    {
                        let mut tmp_struct = EmbeddedVrfKeyLinkTransactionV1::default();
                        tmp_struct.network = NetworkType::TESTNET;
                        tmp_struct.linked_public_key = PublicKey::from_str(
                            "C614558647D02037384A2FECA80ACE95B235D9B9D90035FA46102FE79ECCBA75",
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
                "C28DC781CBA32E6D357869E20D90BD00B29729341414E5ECE5FBF8B93492A880",
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
    }
}
