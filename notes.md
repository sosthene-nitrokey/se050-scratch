Sessions
========

Sessions require nested APDUs.
Solution: Writer interface for Command builder, instead of taking bytes for the data, it takes a 

```rust
pub trait DataSource {
    /// Total length of the containinc data
    fn bytes(&self) -> usize;
    fn to_writer<W: iso7816::command::Writer>(&self, writer: &mut W) -> Result<(), W::Error>;
}
```

```rust
pub trait Writer {
    type Error;
    fn write(&mut self, data: &[u8]) -> Result<usize, Self::Error>;
}
```

```rust
impl CommandBulder {
    fn new(
        class: class::Class,
        instruction: instruction::Instruction,
        p1: u8,
        p2: u8,
        data: &impl DataSource,
        le: u16,
      )
}
```

With `DataSource` being implemented for `&[u8]`, but also other structures like tagged slices, and `CommandBuilder` itself for recursive implementations.

PINs
====

Pins are stored in UserIDs

Workflow:

### Session (pin) creation

1. Create Authentication Object (UserID with WriteUserID, with the pin in the Value field (First hashed because limited to 16 bytes?))
2. Create session with `CreateSession`, pass in the object identifier of the correspondinc UserID


### Session usage

1. `VerifySessionUserId` with the Pin as value and the session ID from `CreateSession`
2. All commands are wrapped in a `ProcessSessionCmd`


Policies
========

Policy set contains multiple policies.
Policy contains and Auth Object ID and an access rule, the access rule applies to the auth object ID.
Access rules are defined by bitflags + extensions.


Bitflags and extensions defined in 4.3.34.2 - AN12413

Serialization of the whole thing?

Looking a [se050-nano-packange](https://github.com/NXPPlugNTrust/nano-package/blob/master/examples/se05x_crypto/src/ex_se05x_crypto.c#L565):

```c
policyBuf[policyBuflen++] = 0; //Update At end.

policyBuf[policyBuflen++] = (uint8_t)((AuthObjId >> 3 * 8) & 0xFF);
policyBuf[policyBuflen++] = (uint8_t)((AuthObjId >> 2 * 8) & 0xFF);
policyBuf[policyBuflen++] = (uint8_t)((AuthObjId >> 1 * 8) & 0xFF);
policyBuf[policyBuflen++] = (uint8_t)((AuthObjId >> 0 * 8) & 0xFF);

policyBuf[policyBuflen++] = (uint8_t)((policyHeader >> 3 * 8) & 0xFF);
policyBuf[policyBuflen++] = (uint8_t)((policyHeader >> 2 * 8) & 0xFF);
policyBuf[policyBuflen++] = (uint8_t)((policyHeader >> 1 * 8) & 0xFF);
policyBuf[policyBuflen++] = (uint8_t)((policyHeader >> 0 * 8) & 0xFF);

policyBuf[0] = policyBuflen - 1;
```

My assumption:

Policy = concatenation of policysets
Policyset = total length (1byte) || AuthObjId || AccessRuleHeader (bitflags) || Access Rule extension


Writing stuff
=============

WriteBinary doesn't work for data longer than 255 bytes?
TLV serialization!


Factory reset
=============

Need restricted authentication object `RESERVED_ID_FACTORY_RESET`.
Created with user id 0001020304

Created object `01020304` with userid `01020304`
