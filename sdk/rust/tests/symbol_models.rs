#[cfg(not(feature = "private_network"))]
mod symbol_models_test {
    use hex::decode;
    use symbol::symbol::prelude::*;
    #[test]
    #[allow(non_snake_case)]
    fn AccountAddressRestrictionTransactionV1_account_address_restriction_single_1() {
        let mut tx = AccountAddressRestrictionTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.restriction_flags = AccountRestrictionFlags::ADDRESS;
        tx.restriction_additions
            .push(UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap());
        tx.restriction_additions
            .push(UnresolvedAddress::from_str("TD2ASJ2LKL5LX66PPZ67PYQN4HIMH5SX7OCZLQI").unwrap());
        tx.restriction_deletions
            .push(UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I").unwrap());
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountAddressRestrictionTransactionV1_account_address_restriction_single_2() {
        let mut tx = AccountAddressRestrictionTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.restriction_flags = AccountRestrictionFlags::ADDRESS
            | AccountRestrictionFlags::OUTGOING
            | AccountRestrictionFlags::BLOCK;
        tx.restriction_additions
            .push(UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I").unwrap());
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountAddressRestrictionTransactionV1_account_address_restriction_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountAddressRestrictionTransactionV1_account_address_restriction_aggregate_2() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountKeyLinkTransactionV1_account_key_link_single_1() {
        let mut tx = AccountKeyLinkTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.link_action = LinkAction::LINK;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountKeyLinkTransactionV1_account_key_link_single_2() {
        let mut tx = AccountKeyLinkTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.link_action = LinkAction::UNLINK;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountKeyLinkTransactionV1_account_key_link_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountKeyLinkTransactionV1_account_key_link_aggregate_2() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMetadataTransactionV1_account_metadata_single_1() {
        let mut tx = AccountMetadataTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.scoped_metadata_key = 10;
        tx.value_size_delta = 10;
        tx.value.extend_from_slice("313233424143".as_bytes());
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMetadataTransactionV1_account_metadata_single_2() {
        let mut tx = AccountMetadataTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.scoped_metadata_key = 11258607;
        tx.value_size_delta = -6;
        tx.value.extend_from_slice("313233424143".as_bytes());
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMetadataTransactionV1_account_metadata_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMetadataTransactionV1_account_metadata_aggregate_2() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMosaicRestrictionTransactionV1_account_mosaic_restriction_single_1() {
        let mut tx = AccountMosaicRestrictionTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.restriction_flags = AccountRestrictionFlags::MOSAIC_ID;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMosaicRestrictionTransactionV1_account_mosaic_restriction_single_2() {
        let mut tx = AccountMosaicRestrictionTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.restriction_flags = AccountRestrictionFlags::MOSAIC_ID | AccountRestrictionFlags::BLOCK;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMosaicRestrictionTransactionV1_account_mosaic_restriction_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountMosaicRestrictionTransactionV1_account_mosaic_restriction_aggregate_2() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountOperationRestrictionTransactionV1_account_operation_restriction_single_1() {
        let mut tx = AccountOperationRestrictionTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.restriction_flags =
            AccountRestrictionFlags::OUTGOING | AccountRestrictionFlags::TRANSACTION_TYPE;
        tx.restriction_additions.push(TransactionType::SECRET_PROOF);
        tx.restriction_deletions.push(TransactionType::TRANSFER);
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountOperationRestrictionTransactionV1_account_operation_restriction_single_2() {
        let mut tx = AccountOperationRestrictionTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.restriction_flags = AccountRestrictionFlags::OUTGOING
            | AccountRestrictionFlags::TRANSACTION_TYPE
            | AccountRestrictionFlags::BLOCK;
        tx.restriction_additions
            .push(TransactionType::ADDRESS_ALIAS);
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountOperationRestrictionTransactionV1_account_operation_restriction_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AccountOperationRestrictionTransactionV1_account_operation_restriction_aggregate_2() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AddressAliasTransactionV1_address_alias_single_1() {
        let mut tx = AddressAliasTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.namespace_id = NamespaceId(9562080086528621131);
        tx.alias_action = AliasAction::LINK;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AddressAliasTransactionV1_address_alias_single_2() {
        let mut tx = AddressAliasTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.namespace_id = NamespaceId(9562080086528621131);
        tx.alias_action = AliasAction::UNLINK;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AddressAliasTransactionV1_address_alias_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AddressAliasTransactionV1_address_alias_aggregate_2() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AggregateBondedTransactionV1_aggregate_bonded_aggregate_1() {
        let mut tx = AggregateBondedTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AggregateBondedTransactionV2_aggregate_bonded_aggregate_2() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AggregateCompleteTransactionV1_aggregate_complete_aggregate_1() {
        let mut tx = AggregateCompleteTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn AggregateCompleteTransactionV2_aggregate_complete_aggregate_2() {
        let mut tx = AggregateCompleteTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn HashLockTransactionV1_hash_lock_single_1() {
        let mut tx = HashLockTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.mosaic.mosaic_id = UnresolvedMosaic(9636553580561478212);
        tx.mosaic.amount = UnresolvedMosaic(10000000);
        tx.duration = BlockDuration(100);
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn HashLockTransactionV1_hash_lock_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicAddressRestrictionTransactionV1_mosaic_address_restriction_single_1() {
        let mut tx = MosaicAddressRestrictionTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.mosaic_id = UnresolvedMosaicId(1);
        tx.restriction_key = 1311768467294898927;
        tx.previous_restriction_value = 9;
        tx.new_restriction_value = 8;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicAddressRestrictionTransactionV1_mosaic_address_restriction_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicAliasTransactionV1_mosaic_alias_single_1() {
        let mut tx = MosaicAliasTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.namespace_id = NamespaceId(13182596108967839652);
        tx.mosaic_id = MosaicId(10);
        tx.alias_action = AliasAction::LINK;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicAliasTransactionV1_mosaic_alias_single_2() {
        let mut tx = MosaicAliasTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.namespace_id = NamespaceId(16233676262248077354);
        tx.mosaic_id = MosaicId(14624838436596993100);
        tx.alias_action = AliasAction::UNLINK;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicAliasTransactionV1_mosaic_alias_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicAliasTransactionV1_mosaic_alias_aggregate_2() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicDefinitionTransactionV1_mosaic_definition_single_1() {
        let mut tx = MosaicDefinitionTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.duration = BlockDuration(10000);
        tx.nonce = MosaicNonce(0);
        tx.flags = MosaicFlags::RESTRICTABLE | MosaicFlags::SUPPLY_MUTABLE;
        tx.divisibility = 4;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicDefinitionTransactionV1_mosaic_definition_single_2() {
        let mut tx = MosaicDefinitionTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.duration = BlockDuration(1000);
        tx.nonce = MosaicNonce(3095715558);
        tx.flags = MosaicFlags::NONE;
        tx.divisibility = 3;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicDefinitionTransactionV1_mosaic_definition_single_3() {
        let mut tx = MosaicDefinitionTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.duration = BlockDuration(0);
        tx.nonce = MosaicNonce(3095715558);
        tx.flags = MosaicFlags::REVOKABLE | MosaicFlags::TRANSFERABLE;
        tx.divisibility = 2;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicDefinitionTransactionV1_mosaic_definition_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicDefinitionTransactionV1_mosaic_definition_aggregate_2() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicDefinitionTransactionV1_mosaic_definition_aggregate_3() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicGlobalRestrictionTransactionV1_mosaic_global_restriction_single_1() {
        let mut tx = MosaicGlobalRestrictionTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.mosaic_id = UnresolvedMosaicId(1875072453572000775);
        tx.reference_mosaic_id = UnresolvedMosaicId(7706325451784159270);
        tx.restriction_key = 1;
        tx.previous_restriction_value = 9;
        tx.new_restriction_value = 8;
        tx.previous_restriction_type = MosaicRestrictionType::EQ;
        tx.new_restriction_type = MosaicRestrictionType::GE;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicGlobalRestrictionTransactionV1_mosaic_global_restriction_single_2() {
        let mut tx = MosaicGlobalRestrictionTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.mosaic_id = UnresolvedMosaicId(6207017352306769745);
        tx.reference_mosaic_id = UnresolvedMosaicId(0);
        tx.restriction_key = 4444;
        tx.previous_restriction_value = 0;
        tx.new_restriction_value = 0;
        tx.previous_restriction_type = MosaicRestrictionType::NONE;
        tx.new_restriction_type = MosaicRestrictionType::GE;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicGlobalRestrictionTransactionV1_mosaic_global_restriction_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicGlobalRestrictionTransactionV1_mosaic_global_restriction_aggregate_2() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicMetadataTransactionV1_mosaic_metadata_single_1() {
        let mut tx = MosaicMetadataTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.scoped_metadata_key = 10;
        tx.target_mosaic_id = UnresolvedMosaicId(1000);
        tx.value_size_delta = 10;
        tx.value.extend_from_slice("313233414243".as_bytes());
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicMetadataTransactionV1_mosaic_metadata_single_2() {
        let mut tx = MosaicMetadataTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.scoped_metadata_key = 10;
        tx.target_mosaic_id = UnresolvedMosaicId(1000);
        tx.value_size_delta = -5;
        tx.value.extend_from_slice("313233414243".as_bytes());
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicMetadataTransactionV1_mosaic_metadata_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicMetadataTransactionV1_mosaic_metadata_aggregate_2() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicSupplyChangeTransactionV1_mosaic_supply_change_single_1() {
        let mut tx = MosaicSupplyChangeTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.mosaic_id = UnresolvedMosaicId(6300565133566699912);
        tx.action = MosaicSupplyChangeAction::INCREASE;
        tx.delta = Amount(10);
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicSupplyChangeTransactionV1_mosaic_supply_change_single_2() {
        let mut tx = MosaicSupplyChangeTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.mosaic_id = UnresolvedMosaicId(14624838436596993100);
        tx.action = MosaicSupplyChangeAction::DECREASE;
        tx.delta = Amount(10);
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicSupplyChangeTransactionV1_mosaic_supply_change_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MosaicSupplyChangeTransactionV1_mosaic_supply_change_aggregate_2() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MultisigAccountModificationTransactionV1_multisig_account_modification_single_1() {
        let mut tx = MultisigAccountModificationTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.min_removal_delta = 1;
        tx.min_approval_delta = 2;
        tx.address_additions
            .push(UnresolvedAddress::from_str("TBA6LOHEA6A465G2X5MSQF66JBYR254GJDPK7MQ").unwrap());
        tx.address_additions
            .push(UnresolvedAddress::from_str("TD2ASJ2LKL5LX66PPZ67PYQN4HIMH5SX7OCZLQI").unwrap());
        tx.address_deletions
            .push(UnresolvedAddress::from_str("TCIFSMQZAX3IDPHUP2RTXP26N6BJRNKEBBKP33I").unwrap());
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn MultisigAccountModificationTransactionV1_multisig_account_modification_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceMetadataTransactionV1_namespace_metadata_single_1() {
        let mut tx = NamespaceMetadataTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.scoped_metadata_key = 10;
        tx.target_namespace_id = NamespaceId(1000);
        tx.value_size_delta = 10;
        tx.value.extend_from_slice("ABC123".as_bytes());
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceMetadataTransactionV1_namespace_metadata_single_2() {
        let mut tx = NamespaceMetadataTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.scoped_metadata_key = 10;
        tx.target_namespace_id = NamespaceId(1000);
        tx.value_size_delta = -3;
        tx.value.extend_from_slice("ABC123".as_bytes());
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceMetadataTransactionV1_namespace_metadata_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceMetadataTransactionV1_namespace_metadata_aggregate_2() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceRegistrationTransactionV1_namespace_registration_single_1() {
        let mut tx = NamespaceRegistrationTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.duration = BlockDuration(10000);
        tx.id = NamespaceId(13858666424160217470);
        tx.registration_type = NamespaceRegistrationType::ROOT;
        tx.name.extend_from_slice("newnamespace".as_bytes());
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceRegistrationTransactionV1_namespace_registration_single_2() {
        let mut tx = NamespaceRegistrationTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.parent_id = NamespaceId(4635294387305441662);
        tx.id = NamespaceId(17411894141110456835);
        tx.registration_type = NamespaceRegistrationType::CHILD;
        tx.name.extend_from_slice("subnamespace".as_bytes());
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceRegistrationTransactionV1_namespace_registration_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn NamespaceRegistrationTransactionV1_namespace_registration_aggregate_2() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn NodeKeyLinkTransactionV1_node_key_link_single_1() {
        let mut tx = NodeKeyLinkTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.link_action = LinkAction::LINK;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn NodeKeyLinkTransactionV1_node_key_link_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn SecretLockTransactionV1_secret_lock_single_1() {
        let mut tx = SecretLockTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.mosaic.mosaic_id = UnresolvedMosaic(9636553580561478212);
        tx.mosaic.amount = UnresolvedMosaic(10000000);
        tx.duration = BlockDuration(100);
        tx.hash_algorithm = LockHashAlgorithm::SHA3_256;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn SecretLockTransactionV1_secret_lock_single_2() {
        let mut tx = SecretLockTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.mosaic.mosaic_id = UnresolvedMosaic(9636553580561478212);
        tx.mosaic.amount = UnresolvedMosaic(1311768467294899695);
        tx.duration = BlockDuration(100);
        tx.hash_algorithm = LockHashAlgorithm::HASH_160;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn SecretLockTransactionV1_secret_lock_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn SecretLockTransactionV1_secret_lock_aggregate_2() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn SecretProofTransactionV1_secret_proof_single_1() {
        let mut tx = SecretProofTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.hash_algorithm = LockHashAlgorithm::SHA3_256;
        tx.proof.extend_from_slice("9A493664".as_bytes());
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn SecretProofTransactionV1_secret_proof_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_1() {
        let mut tx = TransferTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_2() {
        let mut tx = TransferTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_3() {
        let mut tx = TransferTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_4() {
        let mut tx = TransferTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.message
            .extend_from_slice("D600000300504C5445000000FBAF93F7".as_bytes());
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_5() {
        let mut tx = TransferTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.message
            .extend_from_slice("It's some kind of magic, magic".as_bytes());
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_6() {
        let mut tx = TransferTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.message.extend_from_slice("Hello 👋".as_bytes());
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_single_7() {
        let mut tx = TransferTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_2() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_3() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_4() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_5() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_6() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn TransferTransactionV1_transfer_aggregate_7() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn VotingKeyLinkTransactionV1_voting_key_link_single_1() {
        let mut tx = VotingKeyLinkTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.start_epoch = FinalizationEpoch(1);
        tx.end_epoch = FinalizationEpoch(3);
        tx.link_action = LinkAction::LINK;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn VotingKeyLinkTransactionV1_voting_key_link_single_2() {
        let mut tx = VotingKeyLinkTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.start_epoch = FinalizationEpoch(205);
        tx.end_epoch = FinalizationEpoch(272);
        tx.link_action = LinkAction::UNLINK;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn VotingKeyLinkTransactionV1_voting_key_link_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn VotingKeyLinkTransactionV1_voting_key_link_aggregate_2() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn VrfKeyLinkTransactionV1_vrf_key_link_single_1() {
        let mut tx = VrfKeyLinkTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.link_action = LinkAction::LINK;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn VrfKeyLinkTransactionV1_vrf_key_link_single_2() {
        let mut tx = VrfKeyLinkTransactionV1::default();
        tx.network = NetworkType::TESTNET;
        tx.link_action = LinkAction::UNLINK;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn VrfKeyLinkTransactionV1_vrf_key_link_aggregate_1() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
    #[test]
    #[allow(non_snake_case)]
    fn VrfKeyLinkTransactionV1_vrf_key_link_aggregate_2() {
        let mut tx = AggregateBondedTransactionV2::default();
        tx.network = NetworkType::TESTNET;
        tx.fee = Amount(18370164183782063840);
        tx.deadline = Timestamp(8207562320463688160);
    }
}
