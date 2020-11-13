import "entity.cats"

using ProofGamma = binary_fixed(32)
using ProofVerificationHash = binary_fixed(16)
using ProofScalar = binary_fixed(32)

# Verfiable random function proof
struct VrfProof
	# gamma
	gamma = ProofGamma

	# verification hash
	verificationHash = ProofVerificationHash

	# scalar
	scalar = ProofScalar

# binary layout for a block header
struct BlockHeader
	inline SizePrefixedEntity
	inline VerifiableEntity
	inline EntityBody

	# block height
	height = Height

	# number of milliseconds elapsed since creation of nemesis block
	timestamp = Timestamp

	# block difficulty
	difficulty = Difficulty

	# generation hash proof
	generationHashProof = VrfProof

	# previous block hash
	previousBlockHash = Hash256

	# hash of the transactions in this block
	transactionsHash = Hash256

	# hash of the receipts generated by this block
	receiptsHash = Hash256

	# hash of the global chain state at this block
	stateHash = Hash256

	# beneficiary address designated by harvester
	beneficiaryAddress = Address

	# fee multiplier applied to block transactions
	feeMultiplier = BlockFeeMultiplier

# binary layout for an importance block footer
struct ImportanceBlockFooter
	# number of voting eligible accounts
	votingEligibleAccountsCount = uint32

	# number of harvesting eligible accounts
	harvestingEligibleAccountsCount = uint64

	# total balance eligible for voting
	totalVotingBalance = Amount

	# previous importance block hash
	previousImportanceBlockHash = Hash256

# binary layout for a nemesis block header
struct NemesisBlockHeader
	const uint8 version = 1
	const EntityType entityType = 0x8043

	inline BlockHeader
	inline ImportanceBlockFooter

# binary layout for a normal block header
struct NormalBlockHeader
	const uint8 version = 1
	const EntityType entityType = 0x8143

	inline BlockHeader

	# reserved padding to align end of BlockHeader on 8-byte boundary
	blockHeader_Reserved1 = uint32

# binary layout for an importance block header
struct ImportanceBlockHeader
	const uint8 version = 1
	const EntityType entityType = 0x8243

	inline BlockHeader
	inline ImportanceBlockFooter
