use crate::block_header::BlockHeader;

pub fn create_test_block_header_shapella() -> BlockHeader {
    BlockHeader {
        block_hash: "0xb49fa930183f0fa8ca46244e946a635637b90c374d845e9c68dd7fd66cb6e0a1"
            .to_string(),
        number: 17034871,
        gas_limit: 30000000,
        gas_used: 29970493,
        nonce: "0x0000000000000000".to_string(),
        transaction_root: Some(
            "0xd3deb43136a95fac91dc5a451caf62b18f0d3ffa5d8744b769ac5df6d8ef40bc".to_string(),
        ),
        receipts_root: Some(
            "0xe4001611ca993adcb8948b4f114bc0985948bee0fef32c4e2292ad87356afd58".to_string(),
        ),
        state_root: Some(
            "0x1d49150d23cc59f06491e138b7a9bf0d307bcd7bb2bab6e38754740931d6c3ef".to_string(),
        ),
        base_fee_per_gas: Some("0x4b5b025b0".to_string()),
        parent_hash: Some(
            "0xe22c56f211f03baadcc91e4eb9a24344e6848c5df4473988f893b58223f5216c".to_string(),
        ),
        miner: Some("0x0b70b578abd96aab5e80d24d1f3c28dbde14356a".to_string()),
        logs_bloom: Some("0x9479b5dce9a45ebc5af9eee4f1a9cb73dfb3437b2b7edff2a5f36addcfa4319e74212ff4d469c756dfddbeb6db79d5ab5effee57ac57fdfdbbe30a1fd8ffa9fdb9ddfb19771ffdbd7eff4bdbd3d5f97fdf571edeeafefe5a6eb55eceffd739b27ffcff7f16a38dbff45ddd25fb9d7b9bef9bae37e0bb6f7dfbf5fbf3cdfd5dbdbe57ffdfe7ef77ddb7fef267af53587bfb7ffcb3f9f673fc7eababe5fbfe77eddff945e347a6fffdfbfbe8fd7ffee79efcfcce5d77a66db3f9fabe9f3b8b1d79f9a99bb7af11f93f6f9b736f2cbf5697d57f4b6de478bdfef6759dfbbe5bebee7cfff7e9df8acdd46f9777fb47edfaf0b5f5ba74393acff5d24bfcf389fb9eff".to_string()),
        difficulty: Some("0x0".to_string()),
        totaldifficulty: Some("0xd8fb2...".to_string()),
        sha3_uncles: Some(
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string(),
        ),
        timestamp: Some("0x64373087".to_string()),
        extra_data: Some("0x4e65746865726d696e64".to_string()),
        mix_hash: Some(
            "0xd7a4a06e28abcc8ac4e0bab5f0a7e60ea7a0c3de93b2f7e7a4cc3c9a79e60186".to_string(),
        ),
        ommers_hash: Some(
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string(),
        ),
        withdrawals_root: Some(
            "0xc32381c919dad80afe8fe0df79460418e350725a63f67c55b27ee168ef464e5d".to_string(),
        ),
        blob_gas_used: Some("0xa0000".to_string()),
        excess_blob_gas: Some("0x20000".to_string()),
        parent_beacon_block_root: Some(
            "0x2b5b8c2d329148bc5d724cac0a7abc557f93471c24ea027b119ecedb937c0045".to_string(),
        ),
    }
}

pub fn create_test_block_header_paris() -> BlockHeader {
    BlockHeader {
        block_hash: "0xa49fa930183f0fa8ca46244e946a635637b90c374d845e9c68dd7fd66cb6e051"
            .to_string(),
        number: 15537395,
        gas_limit: 30000000,
        gas_used: 29982083,
        nonce: "0x0000000000000000".to_string(),
        transaction_root: Some(
            "0x5c56184fbce74e9c98d2a51aa2110963396047d84e8ce1ae1785337255cd11e0".to_string(),
        ),
        receipts_root: Some(
            "0x1707e457973ce280debe93f5d478663d97ad192beea1f2fc65f391db80226e5e".to_string(),
        ),
        state_root: Some(
            "0x2ca38a39c5517f658d107c19550334a9820a7393d14857f2c0e0458292668d64".to_string(),
        ),
        base_fee_per_gas: Some("0xcc8ac8283".to_string()),
        parent_hash: Some(
            "0x56a9bb0302da44b8c0b3df540781424684c3af04d0b7a38d72842b762076a664".to_string(),
        ),
        miner: Some("0x0b3b161b8abeb6b04cb95c3e6047f80c120a0292".to_string()),
        logs_bloom: Some(
            "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
                .to_string(),
        ),
        difficulty: Some("0x0".to_string()),
        totaldifficulty: Some("0xc8fb2...".to_string()),
        sha3_uncles: Some(
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string(),
        ),
        timestamp: Some("0x6322c97f".to_string()),
        extra_data: Some("0x".to_string()),
        mix_hash: Some(
            "0xfd7be062e87b2193dff12ca89e90bbf61684e6676df4e86cca6730237863dd23".to_string(),
        ),
        ommers_hash: Some(
            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string(),
        ),
        withdrawals_root: None,
        blob_gas_used: None,
        excess_blob_gas: None,
        parent_beacon_block_root: None,
    }
}

pub fn create_test_block_header_london() -> BlockHeader {
    BlockHeader {
        block_hash: "0x9b83c12c69edb74f6c8dd5d052765c1adf940e320bd1291696e6fa07829eee71".to_string(),
        number: 12965001,
        gas_limit: 29999798,
        gas_used: 29985144,
        nonce: "0x0956e895d988798e".to_string(),
        transaction_root: Some("0x03c97f958cc4db3cc60def5ce1e83aaf1490837f5f57c529a6ccffef0d201edb".to_string()),
        receipts_root: Some("0x2335850563dbf51f65a37508f2fdd9da1780f70cfa46734107a2e86a9fde46d7".to_string()),
        state_root: Some("0x0180d59eb0855ef6dbca806fbe81491bea252ab2e0d3a8bb8786326d598e3cd9".to_string()),
        base_fee_per_gas: Some("0x430da58e".to_string()),
        parent_hash: Some("0x9b83c12c69edb74f6c8dd5d052765c1adf940e320bd1291696e6fa07829eee71".to_string()),
        ommers_hash: Some("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string()),
        miner: Some("0x829bd824b016326a401d083b33d092293333a830".to_string()),
        logs_bloom: Some("0x74adf8cfdd0a1ddf12f3d6d5bbd79cab73a19b6986fc007932d9acffafebb747debf512456c87e9afffa5f40fd21ad403b97f3b38e86e9e9db62433eb2b6f8547ad677fdab07f1adcb83686fb37db9ea7acb113f0d74b397324d9cfbf8f33cb3dbfb0d256bcbdaf608dd7b1ac168ee40e322b69bf675a6f4fbbbbe72dccbdd88fab28e7d94685c34bffc9bd1ff98ef777af7ff9793de951d336a1b75acbc7f11ce9dac7e9942ab6a363b4fbebbc3d738dbee5a993fa7c87adce26cbeddfdfcf4d59bba977fb7514a3da550c0b21f399e8bf56778c7dfdcfeeb2457abef1fe63eaf38ecbabdae6c237afd34378163feb6ccdb42f56782cd474bdf9ee9fadb94b4".to_string()),
        difficulty: Some("0x1b81c23e05b218".to_string()),
        totaldifficulty: Some("0x608af5dd578251af429".to_string()),
        sha3_uncles: Some("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_string()),
        timestamp: Some("0x610bdab3".to_string()),
        extra_data: Some("0xe4b883e5bda9e7a59ee4bb99e9b1bc030521".to_string()),
        mix_hash: Some("0xcb3166ebb1888430069b769145b20ba5e3a55f32fd2fa39f0ebdc08d60b4557e".to_string()),
        withdrawals_root: None,
        blob_gas_used: None,
        excess_blob_gas: None,
        parent_beacon_block_root: None,
    }
}
