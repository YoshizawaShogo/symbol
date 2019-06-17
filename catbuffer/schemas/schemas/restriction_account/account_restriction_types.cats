# account restriction types
enum AccountRestrictionType : uint8
	# account restriction type is an address
	address = 0x01

	# account restriction type is a mosaic id
	mosaicId = 0x02

	# account restriction type is a transaction type
	transactionType = 0x04

	# account restriction type sentinel
	sentinel = 0x05

	# account restriction is interpreted as blocking operation
	block = 0x80

# account restriction modification type
enum AccountRestrictionModificationType : uint8
	# add account restriction value
	add = 0x00

	# remove account restriction value
	del = 0x01

# account restriction basic modification
struct AccountRestrictionModification
	modificationType = AccountRestrictionModificationType
