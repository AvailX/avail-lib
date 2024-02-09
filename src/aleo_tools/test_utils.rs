use snarkvm::file::Manifest;
use snarkvm::prelude::*;

use snarkvm::synthesizer::Program;
use std::{fs, fs::File, io::Write, ops::Add, panic::catch_unwind, path::PathBuf, str::FromStr};

use crate::errors::{AvailError, AvailErrorType, AvailResult};

pub const IMPORT_PROGRAM: &str = "
import credits.aleo;
program aleo_test.aleo;

function test:
    input r0 as u32.public;
    input r1 as u32.private;
    add r0 r1 into r2;
    output r2 as u32.private;
";

pub const FINALIZE_TEST_PROGRAM: &str = "program finalize_test.aleo;

mapping monotonic_counter:
    // Counter key
    key id as u32.public;
    // Counter value
    value counter as u32.public;

function increase_counter:
    // Counter index
    input r0 as u32.public;
    // Value to increment by
    input r1 as u32.public;
    finalize r0 r1;

finalize increase_counter:
    // Counter index
    input r0 as u32.public;
    // Value to increment by
    input r1 as u32.public;
    // Get or initialize counter key
    get.or_use monotonic_counter[r0] 0u32 into r2;
    // Add r1 to into the existing counter value
    add r1 r2 into r3;
    // Set r3 into account[r0];
    set r3 into monotonic_counter[r0];
";

pub const CREDITS_IMPORT_TEST_PROGRAM: &str = "import credits.aleo;
program credits_import_test.aleo;

function test:
    input r0 as u32.public;
    input r1 as u32.private;
    add r0 r1 into r2;
    output r2 as u32.private;
";

pub const HELLO_PROGRAM: &str = "program hello.aleo;

function hello:
    input r0 as u32.public;
    input r1 as u32.private;
    add r0 r1 into r2;
    output r2 as u32.private;
";

pub const HELLO_PROGRAM_2: &str = "program hello.aleo;

function hello:
    input r0 as u32.public;
    input r1 as u32.private;
    mul r0 r1 into r2;
    output r2 as u32.private;
";

pub const GENERIC_PROGRAM_BODY: &str = "

function fabulous:
    input r0 as u32.public;
    input r1 as u32.private;
    add r0 r1 into r2;
    output r2 as u32.private;
";

pub const MULTIPLY_PROGRAM: &str =
    "// The 'multiply_test.aleo' program which is imported by the 'double_test.aleo' program.
program multiply_test.aleo;

function multiply:
    input r0 as u32.public;
    input r1 as u32.private;
    mul r0 r1 into r2;
    output r2 as u32.private;
";

pub const AVAIL_NFT_TEST: &str = "
program avail_nft_0.aleo;

struct TokenId:
    data1 as u128;
    data2 as u128;

struct BaseURI:
    data0 as u128;
    data1 as u128;
    data2 as u128;
    data3 as u128;

struct SymbolBits:
    data as u128;

record NFT:
    owner as address.private;
    data as TokenId.private;
    edition as scalar.private;

record NFT_mint:
    owner as address.private;
    amount as u8.private;

record NFT_claim:
    owner as address.private;
    claim as field.private;

record NFT_ownership:
    owner as address.private;
    nft_owner as address.private;
    data as TokenId.private;
    edition as scalar.private;


mapping nft_owners:
	key as field.public;
	value as address.public;


mapping general_settings:
	key as u8.public;
	value as u128.public;


mapping nfts_to_mint:
	key as u128.public;
	value as field.public;


mapping claims_to_nfts:
	key as field.public;
	value as field.public;


mapping toggle_settings:
	key as u8.public;
	value as u32.public;

function initialize_collection:
    input r0 as u128.public;
    input r1 as u128.public;
    input r2 as BaseURI.public;
    assert.eq self.caller aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px;
    async initialize_collection r0 r1 r2 into r3;
    output r3 as avail_nft_0.aleo/initialize_collection.future;

finalize initialize_collection:
    input r0 as u128.public;
    input r1 as u128.public;
    input r2 as BaseURI.public;
    get.or_use toggle_settings[0u8] 0u32 into r3;
    and r3 1u32 into r4;
    assert.eq r4 0u32;
    set 0u128 into general_settings[0u8];
    set r0 into general_settings[1u8];
    set r1 into general_settings[2u8];
    set r2.data0 into general_settings[3u8];
    set r2.data1 into general_settings[4u8];
    set r2.data2 into general_settings[5u8];
    set r2.data3 into general_settings[6u8];
    set 5u32 into toggle_settings[0u8];
    set 0u32 into toggle_settings[1u8];


function add_nft:
    input r0 as TokenId.public;
    input r1 as scalar.public;
    assert.eq self.caller aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px;
    hash.bhp256 r0 into r2 as field;
    commit.bhp256 r2 r1 into r3 as field;
    async add_nft r3 into r4;
    output r4 as avail_nft_0.aleo/add_nft.future;

finalize add_nft:
    input r0 as field.public;
    get toggle_settings[0u8] into r1;
    and r1 9u32 into r2;
    assert.eq r2 1u32;
    get general_settings[1u8] into r3;
    sub r3 1u128 into r4;
    set r4 into general_settings[1u8];
    get general_settings[0u8] into r5;
    set r0 into nfts_to_mint[r5];
    add r5 1u128 into r6;
    set r6 into general_settings[0u8];


function add_minter:
    input r0 as address.private;
    input r1 as u8.public;
    assert.eq self.caller aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px;
    cast r0 r1 into r2 as NFT_mint.record;
    async add_minter into r3;
    output r2 as NFT_mint.record;
    output r3 as avail_nft_0.aleo/add_minter.future;

finalize add_minter:
    get toggle_settings[0u8] into r0;
    and r0 9u32 into r1;
    assert.eq r1 1u32;


function update_toggle_settings:
    input r0 as u32.public;
    assert.eq self.caller aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px;
    async update_toggle_settings r0 into r1;
    output r1 as avail_nft_0.aleo/update_toggle_settings.future;

finalize update_toggle_settings:
    input r0 as u32.public;
    get toggle_settings[0u8] into r1;
    and r1 9u32 into r2;
    assert.eq r2 1u32;
    and r0 1u32 into r3;
    assert.eq r3 1u32;
    set r0 into toggle_settings[0u8];


function set_mint_block:
    input r0 as u32.public;
    assert.eq self.caller aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px;
    async set_mint_block r0 into r1;
    output r1 as avail_nft_0.aleo/set_mint_block.future;

finalize set_mint_block:
    input r0 as u32.public;
    get toggle_settings[0u8] into r1;
    and r1 9u32 into r2;
    assert.eq r2 1u32;
    set r0 into toggle_settings[1u8];


function update_symbol:
    input r0 as u128.public;
    assert.eq self.caller aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px;
    async update_symbol r0 into r1;
    output r1 as avail_nft_0.aleo/update_symbol.future;

finalize update_symbol:
    input r0 as u128.public;
    get toggle_settings[0u8] into r1;
    and r1 9u32 into r2;
    assert.eq r2 1u32;
    set r0 into general_settings[2u8];


function update_base_uri:
    input r0 as BaseURI.public;
    assert.eq self.caller aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px;
    async update_base_uri r0 into r1;
    output r1 as avail_nft_0.aleo/update_base_uri.future;

finalize update_base_uri:
    input r0 as BaseURI.public;
    get toggle_settings[0u8] into r1;
    and r1 9u32 into r2;
    assert.eq r2 1u32;
    set r0.data0 into general_settings[3u8];
    set r0.data1 into general_settings[4u8];
    set r0.data2 into general_settings[5u8];
    set r0.data3 into general_settings[6u8];


function open_mint:
    input r0 as scalar.private;
    hash.bhp256 self.caller into r1 as field;
    commit.bhp256 r1 r0 into r2 as field;
    cast self.caller r2 into r3 as NFT_claim.record;
    async open_mint r2 into r4;
    output r3 as NFT_claim.record;
    output r4 as avail_nft_0.aleo/open_mint.future;

finalize open_mint:
    input r0 as field.public;
    get toggle_settings[1u8] into r1;
    lte r1 block.height into r2;
    assert.eq r2 true;
    get toggle_settings[0u8] into r3;
    and r3 15u32 into r4;
    assert.eq r4 3u32;
    get.or_use claims_to_nfts[r0] 0field into r5;
    assert.eq r5 0field;
    rand.chacha into r6 as u128;
    get.or_use general_settings[0u8] 0u128 into r7;
    rem r6 r7 into r8;
    get nfts_to_mint[r8] into r9;
    set r9 into claims_to_nfts[r0];
    sub r7 1u128 into r10;
    set r10 into general_settings[0u8];
    get nfts_to_mint[r10] into r11;
    set r11 into nfts_to_mint[r8];


function mint:
    input r0 as NFT_mint.record;
    input r1 as scalar.private;
    hash.bhp256 self.caller into r2 as field;
    commit.bhp256 r2 r1 into r3 as field;
    sub r0.amount 1u8 into r4;
    cast r0.owner r4 into r5 as NFT_mint.record;
    cast r0.owner r3 into r6 as NFT_claim.record;
    async mint r3 into r7;
    output r5 as NFT_mint.record;
    output r6 as NFT_claim.record;
    output r7 as avail_nft_0.aleo/mint.future;

finalize mint:
    input r0 as field.public;
    get toggle_settings[1u8] into r1;
    lte r1 block.height into r2;
    assert.eq r2 true;
    get toggle_settings[0u8] into r3;
    and r3 11u32 into r4;
    assert.eq r4 3u32;
    get.or_use claims_to_nfts[r0] 0field into r5;
    assert.eq r5 0field;
    rand.chacha into r6 as u128;
    get.or_use general_settings[0u8] 0u128 into r7;
    rem r6 r7 into r8;
    get nfts_to_mint[r8] into r9;
    set r9 into claims_to_nfts[r0];
    sub r7 1u128 into r10;
    set r10 into general_settings[0u8];
    get nfts_to_mint[r10] into r11;
    set r11 into nfts_to_mint[r8];


function claim_nft:
    input r0 as NFT_claim.record;
    input r1 as TokenId.private;
    input r2 as scalar.private;
    hash.bhp256 r1 into r3 as field;
    commit.bhp256 r3 r2 into r4 as field;
    cast r0.owner r1 r2 into r5 as NFT.record;
    async claim_nft r0.claim r4 into r6;
    output r5 as NFT.record;
    output r6 as avail_nft_0.aleo/claim_nft.future;

finalize claim_nft:
    input r0 as field.public;
    input r1 as field.public;
    get claims_to_nfts[r0] into r2;
    assert.eq r2 r1;
    set 0field into claims_to_nfts[r0];


function authorize:
    input r0 as NFT.record;
    input r1 as u64.public;
    async authorize into r2;
    output r2 as avail_nft_0.aleo/authorize.future;

finalize authorize:
    assert.eq 0u8 1u8;


function transfer_private:
    input r0 as NFT.record;
    input r1 as address.private;
    cast r1 r0.data r0.edition into r2 as NFT.record;
    output r2 as NFT.record;


function transfer_public:
    input r0 as address.private;
    input r1 as TokenId.private;
    input r2 as scalar.private;
    hash.bhp256 r1 into r3 as field;
    commit.bhp256 r3 r2 into r4 as field;
    async transfer_public r0 r4 self.caller into r5;
    output r5 as avail_nft_0.aleo/transfer_public.future;

finalize transfer_public:
    input r0 as address.public;
    input r1 as field.public;
    input r2 as address.public;
    get nft_owners[r1] into r3;
    assert.eq r2 r3;
    set r0 into nft_owners[r1];


function convert_private_to_public:
    input r0 as NFT.record;
    hash.bhp256 r0.data into r1 as field;
    commit.bhp256 r1 r0.edition into r2 as field;
    async convert_private_to_public r0.owner r2 into r3;
    output r3 as avail_nft_0.aleo/convert_private_to_public.future;

finalize convert_private_to_public:
    input r0 as address.public;
    input r1 as field.public;
    set r0 into nft_owners[r1];


function convert_public_to_private:
    input r0 as address.private;
    input r1 as TokenId.private;
    input r2 as scalar.private;
    assert.eq r0 self.caller;
    hash.bhp256 r1 into r3 as field;
    commit.bhp256 r3 r2 into r4 as field;
    cast r0 r1 r2 into r5 as NFT.record;
    async convert_public_to_private r0 r4 into r6;
    output r5 as NFT.record;
    output r6 as avail_nft_0.aleo/convert_public_to_private.future;

finalize convert_public_to_private:
    input r0 as address.public;
    input r1 as field.public;
    get nft_owners[r1] into r2;
    assert.eq r0 r2;
    set aleo1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq3ljyzc into nft_owners[r1];";

pub const TOKEN_MINT: &str = "

program token_avl_4.aleo;

mapping account:
	key as address.public;
	value as u64.public;

record token_avl_4:
    owner as address.private;
    amount as u64.private;

struct approval:
    approver as address;
    spender as address;

mapping approvals:
    key as field.public;
    value as u64.public;

function approve_public:
    input r0 as address.public; // spender
    input r1 as u64.public; // amount spender is allowed to withdraw from approver

    // hash approval
    cast self.caller r0 into r2 as approval;
    hash.bhp256 r2 into r3 as field;

    async approve_public r3 r1 into r4;
    output r4 as token_avl_4.aleo/approve_public.future;

finalize approve_public:
    input r0 as field.public;
    input r1 as u64.public; // increase in amount spender is allowed to withdraw from approver

    // if approvals for approval field exists, the approved amount is increased.
    // otherwise, the approved allowance is created.
    get.or_use approvals[r0] 0u64 into r2;
    add r1 r2 into r3;
    set r3 into approvals[r0];

function unapprove_public:
    input r0 as address.public; // spender
    input r1 as u64.public; // amount spender's allowance is decreasing by

    // hash approval
    cast self.caller r0 into r2 as approval;
    hash.bhp256 r2 into r3 as field;

    async unapprove_public r3 r1 into r4;
    output r4 as token_avl_4.aleo/unapprove_public.future;

finalize unapprove_public:
    input r0 as field.public;
    input r1 as u64.public; // decrease in amount spender is allowed to withdraw from approver

    get approvals[r0] into r2;
    sub r2 r1 into r3;
    set r3 into approvals[r0];

/* Transfer From */

function transfer_from_public:
    input r0 as address.public; // from the approver
    input r1 as address.public; // to the receiver
    input r2 as u64.public; // amount to transfer

    cast r0 self.caller into r3 as approval;
    hash.bhp256 r3 into r4 as field; // hash approval

    async transfer_from_public r4 r0 r1 r2 into r5;
    output r5 as token_avl_4.aleo/transfer_from_public.future;

finalize transfer_from_public:
    input r0 as field.public; // approval
    input r1 as address.public; // from the approver
    input r2 as address.public; // to the receiver
    input r3 as u64.public; // amount to transfer

    get approvals[r0] into r4;
    sub r4 r3 into r5;
    set r5 into approvals[r0];
    get account[r1] into r6;
    sub r6 r3 into r7;
    set r7 into account[r1];
    get.or_use account[r2] 0u64 into r8;
    add r8 r3 into r9;
    set r9 into account[r2];

function transfer_public:
    input r0 as address.public;
    input r1 as u64.public;
    async transfer_public self.caller r0 r1 into r2;
    output r2 as token_avl_4.aleo/transfer_public.future;

finalize transfer_public:
    input r0 as address.public;
    input r1 as address.public;
    input r2 as u64.public;
    get.or_use account[r0] 0u64 into r3;
    sub r3 r2 into r4;
    set r4 into account[r0];
    get.or_use account[r1] 0u64 into r5;
    add r5 r2 into r6;
    set r6 into account[r1];


function transfer_private:
    input r0 as token_avl_4.record;
    input r1 as address.private;
    input r2 as u64.private;
    sub r0.amount r2 into r3;
    cast r0.owner r3 into r4 as token_avl_4.record;
    cast r1 r2 into r5 as token_avl_4.record;
    output r4 as token_avl_4.record;
    output r5 as token_avl_4.record;


function transfer_private_to_public:
    input r0 as token_avl_4.record;
    input r1 as address.public;
    input r2 as u64.public;
    sub r0.amount r2 into r3;
    cast r0.owner r3 into r4 as token_avl_4.record;
    async transfer_private_to_public r1 r2 into r5;
    output r4 as token_avl_4.record;
    output r5 as token_avl_4.aleo/transfer_private_to_public.future;

finalize transfer_private_to_public:
    input r0 as address.public;
    input r1 as u64.public;
    get.or_use account[r0] 0u64 into r2;
    add r2 r1 into r3;
    set r3 into account[r0];


function transfer_public_to_private:
    input r0 as address.public;
    input r1 as u64.public;
    cast r0 r1 into r2 as token_avl_4.record;
    async transfer_public_to_private self.caller r1 into r3;
    output r2 as token_avl_4.record;
    output r3 as token_avl_4.aleo/transfer_public_to_private.future;

finalize transfer_public_to_private:
    input r0 as address.public;
    input r1 as u64.public;
    get.or_use account[r0] 0u64 into r2;
    sub r2 r1 into r3;
    set r3 into account[r0];

function mint_public:
    input r0 as address.public;
    input r1 as u64.public;
    async mint_public r0 r1 into r2;
    output r2 as token_avl_4.aleo/mint_public.future;

finalize mint_public:
    input r0 as address.public;
    input r1 as u64.public;
    get.or_use account[r0] 0u64 into r2;
    add r2 r1 into r3;
    set r3 into account[r0];
";

pub const MULTIPLY_IMPORT_PROGRAM: &str =
    "// The 'double_test.aleo' program that uses a single import from another program to perform doubling.
import multiply_test.aleo;

program double_test.aleo;

function double_it:
    input r0 as u32.private;
    call multiply_test.aleo/multiply 2u32 r0 into r1;
    output r1 as u32.private;
";

pub const FEE_ESTIMATION_PROGRAM: &str = "program feeestimation.aleo;

struct function_size:
    function_id as field;
    size as u64;


mapping size_records:
	key as field.public;
	value as function_size.public;

function main:
    input r0 as u32.public;
    input r1 as u32.private;
    add r0 r1 into r2;
    output r2 as u32.private;


function add_size_record:
    input r0 as field.public;
    input r1 as field.public;
    input r2 as u64.public;
    gt r2 0u64 into r3;
    assert.eq r3 true;
    cast r1 r2 into r4 as function_size;
    async add_size_record r0 r4 into r5;
    output r5 as feeestimation.aleo/add_size_record.future;

finalize add_size_record:
    input r0 as field.public;
    input r1 as function_size.public;
    set r1 into size_records[r0];


function remove_size_record:
    input r0 as field.public;
    input r1 as field.public;
    async remove_size_record r0 into r2;
    output r2 as feeestimation.aleo/remove_size_record.future;

finalize remove_size_record:
    input r0 as field.public;
    remove size_records[r0];
";

pub const RECORD_NFT_MINT: &str = r"{
    owner: aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px.private,
    amount: 1u8.private,
    _nonce: ";

pub const RECORD_NFT_CLAIM: &str = r"{
    owner: aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px.private,
    claim: 4479947346975967561932201916893705366415023848624848951360213155145823719753field.private,
    _nonce: ";

pub const RECORD_2000000001_MICROCREDITS: &str = r"{
  owner: aleo1j7qxyunfldj2lp8hsvy7mw5k8zaqgjfyr72x2gh3x4ewgae8v5gscf5jh3.private,
  microcredits: 2000000001u64.private,
  _nonce: 440655410641037118713377218645355605135385337348439127168929531052605977026group.public
}";

pub const RECORD_5_MICROCREDITS: &str = r"{
  owner: aleo1j7qxyunfldj2lp8hsvy7mw5k8zaqgjfyr72x2gh3x4ewgae8v5gscf5jh3.private,
  microcredits: 5u64.private,
  _nonce: 3700202890700295811197086261814785945731964545546334348117582517467189701159group.public
}";

/// Get a random program id
pub fn random_program_id(len: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    let mut rng = rand::thread_rng();

    let program: String = (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    program.add(".aleo")
}

/// Get a random program
pub fn random_program() -> Program<Testnet3> {
    let random_program = String::from("program ")
        .add(&random_program_id(15))
        .add(";")
        .add(GENERIC_PROGRAM_BODY);
    Program::<Testnet3>::from_str(&random_program).unwrap()
}

/// Create temp directory with test data
pub fn setup_directory(
    directory_name: &str,
    main_program: &str,
    imports: Vec<(&str, &str)>,
) -> AvailResult<PathBuf> {
    // Crate a temporary directory for the test.
    let directory = std::env::temp_dir().join(directory_name);

    catch_unwind(|| {
        let _ = &directory
            .exists()
            .then(|| fs::remove_dir_all(&directory).unwrap());
        fs::create_dir(&directory).unwrap();

        let imports_directory = directory.join("imports");
        fs::create_dir(directory.join("imports")).unwrap();
        let program = Program::<Testnet3>::from_str(main_program).unwrap();
        let program_id = program.id();

        // Create the manifest file.
        Manifest::create(&directory, program_id).unwrap();

        let mut main = File::create(directory.join("main.aleo")).unwrap();
        main.write_all(main_program.as_bytes()).unwrap();

        imports.into_iter().for_each(|(name, program)| {
            let mut file = File::create(imports_directory.join(name)).unwrap();
            file.write_all(program.as_bytes()).unwrap();
        });
    })
    .map_err(|_| {
        AvailError::new(
            AvailErrorType::Internal,
            "Failed to create test directory".to_string(),
            "".to_string(),
        )
    })?;
    Ok(directory)
}

/// Teardown temp directory
pub fn teardown_directory(directory: &PathBuf) {
    if directory.exists() {
        fs::remove_dir_all(directory).unwrap();
    }
}
