#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct endpoint(pub Bitfield<u64, 2usize>);
pub type endpoint_t = endpoint;
impl endpoint {
    pub fn new(epQueue_head: u64, epQueue_tail: u64, state: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_epQueue_head(epQueue_head);
        this.set_epQueue_tail(epQueue_tail);
        this.set_state(state);
        this
    }
    pub fn unpack(&self) -> endpoint_Unpacked {
        endpoint_Unpacked {
            epQueue_head: self.get_epQueue_head(),
            epQueue_tail: self.get_epQueue_tail(),
            state: self.get_state(),
        }
    }
    #[allow(dead_code)]
    pub fn get_epQueue_head(&self) -> u64 {
        self.0.get_bits(64usize..128usize)
    }
    pub fn set_epQueue_head(&mut self, epQueue_head: u64) {
        self.0.set_bits(64usize..128usize, epQueue_head)
    }
    #[allow(dead_code)]
    pub const fn width_of_epQueue_head() -> usize {
        128usize - 64usize
    }
    #[allow(dead_code)]
    pub fn get_epQueue_tail(&self) -> u64 {
        self.0.get_bits(2usize..48usize)
    }
    pub fn set_epQueue_tail(&mut self, epQueue_tail: u64) {
        self.0.set_bits(2usize..48usize, epQueue_tail)
    }
    #[allow(dead_code)]
    pub const fn width_of_epQueue_tail() -> usize {
        48usize - 2usize
    }
    #[allow(dead_code)]
    pub fn get_state(&self) -> u64 {
        self.0.get_bits(0usize..2usize)
    }
    pub fn set_state(&mut self, state: u64) {
        self.0.set_bits(0usize..2usize, state)
    }
    #[allow(dead_code)]
    pub const fn width_of_state() -> usize {
        2usize - 0usize
    }
}
impl fmt::Debug for endpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct endpoint_Unpacked {
    pub epQueue_head: u64,
    pub epQueue_tail: u64,
    pub state: u64,
}
impl endpoint_Unpacked {
    pub fn pack(self) -> endpoint {
        match self {
            Self {
                epQueue_head,
                epQueue_tail,
                state,
            } => endpoint::new(epQueue_head, epQueue_tail, state),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct notification(pub Bitfield<u64, 4usize>);
pub type notification_t = notification;
impl notification {
    pub fn new(
        ntfnBoundTCB: u64,
        ntfnMsgIdentifier: u64,
        ntfnQueue_head: u64,
        ntfnQueue_tail: u64,
        state: u64,
    ) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_ntfnBoundTCB(ntfnBoundTCB);
        this.set_ntfnMsgIdentifier(ntfnMsgIdentifier);
        this.set_ntfnQueue_head(ntfnQueue_head);
        this.set_ntfnQueue_tail(ntfnQueue_tail);
        this.set_state(state);
        this
    }
    pub fn unpack(&self) -> notification_Unpacked {
        notification_Unpacked {
            ntfnBoundTCB: self.get_ntfnBoundTCB(),
            ntfnMsgIdentifier: self.get_ntfnMsgIdentifier(),
            ntfnQueue_head: self.get_ntfnQueue_head(),
            ntfnQueue_tail: self.get_ntfnQueue_tail(),
            state: self.get_state(),
        }
    }
    #[allow(dead_code)]
    pub fn get_ntfnBoundTCB(&self) -> u64 {
        self.0.get_bits(192usize..240usize)
    }
    pub fn set_ntfnBoundTCB(&mut self, ntfnBoundTCB: u64) {
        self.0.set_bits(192usize..240usize, ntfnBoundTCB)
    }
    #[allow(dead_code)]
    pub const fn width_of_ntfnBoundTCB() -> usize {
        240usize - 192usize
    }
    #[allow(dead_code)]
    pub fn get_ntfnMsgIdentifier(&self) -> u64 {
        self.0.get_bits(128usize..192usize)
    }
    pub fn set_ntfnMsgIdentifier(&mut self, ntfnMsgIdentifier: u64) {
        self.0.set_bits(128usize..192usize, ntfnMsgIdentifier)
    }
    #[allow(dead_code)]
    pub const fn width_of_ntfnMsgIdentifier() -> usize {
        192usize - 128usize
    }
    #[allow(dead_code)]
    pub fn get_ntfnQueue_head(&self) -> u64 {
        self.0.get_bits(64usize..112usize)
    }
    pub fn set_ntfnQueue_head(&mut self, ntfnQueue_head: u64) {
        self.0.set_bits(64usize..112usize, ntfnQueue_head)
    }
    #[allow(dead_code)]
    pub const fn width_of_ntfnQueue_head() -> usize {
        112usize - 64usize
    }
    #[allow(dead_code)]
    pub fn get_ntfnQueue_tail(&self) -> u64 {
        self.0.get_bits(16usize..64usize)
    }
    pub fn set_ntfnQueue_tail(&mut self, ntfnQueue_tail: u64) {
        self.0.set_bits(16usize..64usize, ntfnQueue_tail)
    }
    #[allow(dead_code)]
    pub const fn width_of_ntfnQueue_tail() -> usize {
        64usize - 16usize
    }
    #[allow(dead_code)]
    pub fn get_state(&self) -> u64 {
        self.0.get_bits(0usize..2usize)
    }
    pub fn set_state(&mut self, state: u64) {
        self.0.set_bits(0usize..2usize, state)
    }
    #[allow(dead_code)]
    pub const fn width_of_state() -> usize {
        2usize - 0usize
    }
}
impl fmt::Debug for notification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct notification_Unpacked {
    pub ntfnBoundTCB: u64,
    pub ntfnMsgIdentifier: u64,
    pub ntfnQueue_head: u64,
    pub ntfnQueue_tail: u64,
    pub state: u64,
}
impl notification_Unpacked {
    pub fn pack(self) -> notification {
        match self {
            Self {
                ntfnBoundTCB,
                ntfnMsgIdentifier,
                ntfnQueue_head,
                ntfnQueue_tail,
                state,
            } => notification::new(
                ntfnBoundTCB,
                ntfnMsgIdentifier,
                ntfnQueue_head,
                ntfnQueue_tail,
                state,
            ),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct mdb_node(pub Bitfield<u64, 2usize>);
pub type mdb_node_t = mdb_node;
impl mdb_node {
    pub fn new(mdbNext: u64, mdbRevocable: u64, mdbFirstBadged: u64, mdbPrev: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_mdbNext(mdbNext);
        this.set_mdbRevocable(mdbRevocable);
        this.set_mdbFirstBadged(mdbFirstBadged);
        this.set_mdbPrev(mdbPrev);
        this
    }
    pub fn unpack(&self) -> mdb_node_Unpacked {
        mdb_node_Unpacked {
            mdbNext: self.get_mdbNext(),
            mdbRevocable: self.get_mdbRevocable(),
            mdbFirstBadged: self.get_mdbFirstBadged(),
            mdbPrev: self.get_mdbPrev(),
        }
    }
    #[allow(dead_code)]
    pub fn get_mdbNext(&self) -> u64 {
        self.0.get_bits(66usize..112usize)
    }
    pub fn set_mdbNext(&mut self, mdbNext: u64) {
        self.0.set_bits(66usize..112usize, mdbNext)
    }
    #[allow(dead_code)]
    pub const fn width_of_mdbNext() -> usize {
        112usize - 66usize
    }
    #[allow(dead_code)]
    pub fn get_mdbRevocable(&self) -> u64 {
        self.0.get_bits(65usize..66usize)
    }
    pub fn set_mdbRevocable(&mut self, mdbRevocable: u64) {
        self.0.set_bits(65usize..66usize, mdbRevocable)
    }
    #[allow(dead_code)]
    pub const fn width_of_mdbRevocable() -> usize {
        66usize - 65usize
    }
    #[allow(dead_code)]
    pub fn get_mdbFirstBadged(&self) -> u64 {
        self.0.get_bits(64usize..65usize)
    }
    pub fn set_mdbFirstBadged(&mut self, mdbFirstBadged: u64) {
        self.0.set_bits(64usize..65usize, mdbFirstBadged)
    }
    #[allow(dead_code)]
    pub const fn width_of_mdbFirstBadged() -> usize {
        65usize - 64usize
    }
    #[allow(dead_code)]
    pub fn get_mdbPrev(&self) -> u64 {
        self.0.get_bits(0usize..64usize)
    }
    pub fn set_mdbPrev(&mut self, mdbPrev: u64) {
        self.0.set_bits(0usize..64usize, mdbPrev)
    }
    #[allow(dead_code)]
    pub const fn width_of_mdbPrev() -> usize {
        64usize - 0usize
    }
}
impl fmt::Debug for mdb_node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct mdb_node_Unpacked {
    pub mdbNext: u64,
    pub mdbRevocable: u64,
    pub mdbFirstBadged: u64,
    pub mdbPrev: u64,
}
impl mdb_node_Unpacked {
    pub fn pack(self) -> mdb_node {
        match self {
            Self {
                mdbNext,
                mdbRevocable,
                mdbFirstBadged,
                mdbPrev,
            } => mdb_node::new(mdbNext, mdbRevocable, mdbFirstBadged, mdbPrev),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct thread_state(pub Bitfield<u64, 3usize>);
pub type thread_state_t = thread_state;
impl thread_state {
    pub fn new(
        blockingIPCBadge: u64,
        blockingIPCCanGrant: u64,
        blockingIPCCanGrantReply: u64,
        blockingIPCIsCall: u64,
        tcbQueued: u64,
        blockingObject: u64,
        tsType: u64,
    ) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_blockingIPCBadge(blockingIPCBadge);
        this.set_blockingIPCCanGrant(blockingIPCCanGrant);
        this.set_blockingIPCCanGrantReply(blockingIPCCanGrantReply);
        this.set_blockingIPCIsCall(blockingIPCIsCall);
        this.set_tcbQueued(tcbQueued);
        this.set_blockingObject(blockingObject);
        this.set_tsType(tsType);
        this
    }
    pub fn unpack(&self) -> thread_state_Unpacked {
        thread_state_Unpacked {
            blockingIPCBadge: self.get_blockingIPCBadge(),
            blockingIPCCanGrant: self.get_blockingIPCCanGrant(),
            blockingIPCCanGrantReply: self.get_blockingIPCCanGrantReply(),
            blockingIPCIsCall: self.get_blockingIPCIsCall(),
            tcbQueued: self.get_tcbQueued(),
            blockingObject: self.get_blockingObject(),
            tsType: self.get_tsType(),
        }
    }
    #[allow(dead_code)]
    pub fn get_blockingIPCBadge(&self) -> u64 {
        self.0.get_bits(128usize..192usize)
    }
    pub fn set_blockingIPCBadge(&mut self, blockingIPCBadge: u64) {
        self.0.set_bits(128usize..192usize, blockingIPCBadge)
    }
    #[allow(dead_code)]
    pub const fn width_of_blockingIPCBadge() -> usize {
        192usize - 128usize
    }
    #[allow(dead_code)]
    pub fn get_blockingIPCCanGrant(&self) -> u64 {
        self.0.get_bits(67usize..68usize)
    }
    pub fn set_blockingIPCCanGrant(&mut self, blockingIPCCanGrant: u64) {
        self.0.set_bits(67usize..68usize, blockingIPCCanGrant)
    }
    #[allow(dead_code)]
    pub const fn width_of_blockingIPCCanGrant() -> usize {
        68usize - 67usize
    }
    #[allow(dead_code)]
    pub fn get_blockingIPCCanGrantReply(&self) -> u64 {
        self.0.get_bits(66usize..67usize)
    }
    pub fn set_blockingIPCCanGrantReply(&mut self, blockingIPCCanGrantReply: u64) {
        self.0.set_bits(66usize..67usize, blockingIPCCanGrantReply)
    }
    #[allow(dead_code)]
    pub const fn width_of_blockingIPCCanGrantReply() -> usize {
        67usize - 66usize
    }
    #[allow(dead_code)]
    pub fn get_blockingIPCIsCall(&self) -> u64 {
        self.0.get_bits(65usize..66usize)
    }
    pub fn set_blockingIPCIsCall(&mut self, blockingIPCIsCall: u64) {
        self.0.set_bits(65usize..66usize, blockingIPCIsCall)
    }
    #[allow(dead_code)]
    pub const fn width_of_blockingIPCIsCall() -> usize {
        66usize - 65usize
    }
    #[allow(dead_code)]
    pub fn get_tcbQueued(&self) -> u64 {
        self.0.get_bits(64usize..65usize)
    }
    pub fn set_tcbQueued(&mut self, tcbQueued: u64) {
        self.0.set_bits(64usize..65usize, tcbQueued)
    }
    #[allow(dead_code)]
    pub const fn width_of_tcbQueued() -> usize {
        65usize - 64usize
    }
    #[allow(dead_code)]
    pub fn get_blockingObject(&self) -> u64 {
        self.0.get_bits(4usize..48usize)
    }
    pub fn set_blockingObject(&mut self, blockingObject: u64) {
        self.0.set_bits(4usize..48usize, blockingObject)
    }
    #[allow(dead_code)]
    pub const fn width_of_blockingObject() -> usize {
        48usize - 4usize
    }
    #[allow(dead_code)]
    pub fn get_tsType(&self) -> u64 {
        self.0.get_bits(0usize..4usize)
    }
    pub fn set_tsType(&mut self, tsType: u64) {
        self.0.set_bits(0usize..4usize, tsType)
    }
    #[allow(dead_code)]
    pub const fn width_of_tsType() -> usize {
        4usize - 0usize
    }
}
impl fmt::Debug for thread_state {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct thread_state_Unpacked {
    pub blockingIPCBadge: u64,
    pub blockingIPCCanGrant: u64,
    pub blockingIPCCanGrantReply: u64,
    pub blockingIPCIsCall: u64,
    pub tcbQueued: u64,
    pub blockingObject: u64,
    pub tsType: u64,
}
impl thread_state_Unpacked {
    pub fn pack(self) -> thread_state {
        match self {
            Self {
                blockingIPCBadge,
                blockingIPCCanGrant,
                blockingIPCCanGrantReply,
                blockingIPCIsCall,
                tcbQueued,
                blockingObject,
                tsType,
            } => thread_state::new(
                blockingIPCBadge,
                blockingIPCCanGrant,
                blockingIPCCanGrantReply,
                blockingIPCIsCall,
                tcbQueued,
                blockingObject,
                tsType,
            ),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct vm_attributes(pub Bitfield<u64, 1usize>);
pub type vm_attributes_t = vm_attributes;
impl vm_attributes {
    pub fn new(armExecuteNever: u64, armParityEnabled: u64, armPageCacheable: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_armExecuteNever(armExecuteNever);
        this.set_armParityEnabled(armParityEnabled);
        this.set_armPageCacheable(armPageCacheable);
        this
    }
    pub fn unpack(&self) -> vm_attributes_Unpacked {
        vm_attributes_Unpacked {
            armExecuteNever: self.get_armExecuteNever(),
            armParityEnabled: self.get_armParityEnabled(),
            armPageCacheable: self.get_armPageCacheable(),
        }
    }
    #[allow(dead_code)]
    pub fn get_armExecuteNever(&self) -> u64 {
        self.0.get_bits(2usize..3usize)
    }
    pub fn set_armExecuteNever(&mut self, armExecuteNever: u64) {
        self.0.set_bits(2usize..3usize, armExecuteNever)
    }
    #[allow(dead_code)]
    pub const fn width_of_armExecuteNever() -> usize {
        3usize - 2usize
    }
    #[allow(dead_code)]
    pub fn get_armParityEnabled(&self) -> u64 {
        self.0.get_bits(1usize..2usize)
    }
    pub fn set_armParityEnabled(&mut self, armParityEnabled: u64) {
        self.0.set_bits(1usize..2usize, armParityEnabled)
    }
    #[allow(dead_code)]
    pub const fn width_of_armParityEnabled() -> usize {
        2usize - 1usize
    }
    #[allow(dead_code)]
    pub fn get_armPageCacheable(&self) -> u64 {
        self.0.get_bits(0usize..1usize)
    }
    pub fn set_armPageCacheable(&mut self, armPageCacheable: u64) {
        self.0.set_bits(0usize..1usize, armPageCacheable)
    }
    #[allow(dead_code)]
    pub const fn width_of_armPageCacheable() -> usize {
        1usize - 0usize
    }
}
impl fmt::Debug for vm_attributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct vm_attributes_Unpacked {
    pub armExecuteNever: u64,
    pub armParityEnabled: u64,
    pub armPageCacheable: u64,
}
impl vm_attributes_Unpacked {
    pub fn pack(self) -> vm_attributes {
        match self {
            Self {
                armExecuteNever,
                armParityEnabled,
                armPageCacheable,
            } => vm_attributes::new(armExecuteNever, armParityEnabled, armPageCacheable),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct pte_table(pub Bitfield<u64, 1usize>);
pub type pte_table_t = pte_table;
impl pte_table {
    pub fn new(pte_sw_type: u64, pt_base_address: u64, pte_hw_type: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_pte_sw_type(pte_sw_type);
        this.set_pt_base_address(pt_base_address);
        this.set_pte_hw_type(pte_hw_type);
        this
    }
    pub fn unpack(&self) -> pte_table_Unpacked {
        pte_table_Unpacked {
            pte_sw_type: self.get_pte_sw_type(),
            pt_base_address: self.get_pt_base_address(),
            pte_hw_type: self.get_pte_hw_type(),
        }
    }
    #[allow(dead_code)]
    pub fn get_pte_sw_type(&self) -> u64 {
        self.0.get_bits(58usize..59usize)
    }
    pub fn set_pte_sw_type(&mut self, pte_sw_type: u64) {
        self.0.set_bits(58usize..59usize, pte_sw_type)
    }
    #[allow(dead_code)]
    pub const fn width_of_pte_sw_type() -> usize {
        59usize - 58usize
    }
    #[allow(dead_code)]
    pub fn get_pt_base_address(&self) -> u64 {
        self.0.get_bits(12usize..48usize)
    }
    pub fn set_pt_base_address(&mut self, pt_base_address: u64) {
        self.0.set_bits(12usize..48usize, pt_base_address)
    }
    #[allow(dead_code)]
    pub const fn width_of_pt_base_address() -> usize {
        48usize - 12usize
    }
    #[allow(dead_code)]
    pub fn get_pte_hw_type(&self) -> u64 {
        self.0.get_bits(0usize..2usize)
    }
    pub fn set_pte_hw_type(&mut self, pte_hw_type: u64) {
        self.0.set_bits(0usize..2usize, pte_hw_type)
    }
    #[allow(dead_code)]
    pub const fn width_of_pte_hw_type() -> usize {
        2usize - 0usize
    }
}
impl fmt::Debug for pte_table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct pte_table_Unpacked {
    pub pte_sw_type: u64,
    pub pt_base_address: u64,
    pub pte_hw_type: u64,
}
impl pte_table_Unpacked {
    pub fn pack(self) -> pte_table {
        match self {
            Self {
                pte_sw_type,
                pt_base_address,
                pte_hw_type,
            } => pte_table::new(pte_sw_type, pt_base_address, pte_hw_type),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct pte_page(pub Bitfield<u64, 1usize>);
pub type pte_page_t = pte_page;
impl pte_page {
    pub fn new(
        pte_sw_type: u64,
        UXN: u64,
        page_base_address: u64,
        nG: u64,
        AF: u64,
        SH: u64,
        AP: u64,
        AttrIndx: u64,
        pte_hw_type: u64,
    ) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_pte_sw_type(pte_sw_type);
        this.set_UXN(UXN);
        this.set_page_base_address(page_base_address);
        this.set_nG(nG);
        this.set_AF(AF);
        this.set_SH(SH);
        this.set_AP(AP);
        this.set_AttrIndx(AttrIndx);
        this.set_pte_hw_type(pte_hw_type);
        this
    }
    pub fn unpack(&self) -> pte_page_Unpacked {
        pte_page_Unpacked {
            pte_sw_type: self.get_pte_sw_type(),
            UXN: self.get_UXN(),
            page_base_address: self.get_page_base_address(),
            nG: self.get_nG(),
            AF: self.get_AF(),
            SH: self.get_SH(),
            AP: self.get_AP(),
            AttrIndx: self.get_AttrIndx(),
            pte_hw_type: self.get_pte_hw_type(),
        }
    }
    #[allow(dead_code)]
    pub fn get_pte_sw_type(&self) -> u64 {
        self.0.get_bits(58usize..59usize)
    }
    pub fn set_pte_sw_type(&mut self, pte_sw_type: u64) {
        self.0.set_bits(58usize..59usize, pte_sw_type)
    }
    #[allow(dead_code)]
    pub const fn width_of_pte_sw_type() -> usize {
        59usize - 58usize
    }
    #[allow(dead_code)]
    pub fn get_UXN(&self) -> u64 {
        self.0.get_bits(54usize..55usize)
    }
    pub fn set_UXN(&mut self, UXN: u64) {
        self.0.set_bits(54usize..55usize, UXN)
    }
    #[allow(dead_code)]
    pub const fn width_of_UXN() -> usize {
        55usize - 54usize
    }
    #[allow(dead_code)]
    pub fn get_page_base_address(&self) -> u64 {
        self.0.get_bits(12usize..48usize)
    }
    pub fn set_page_base_address(&mut self, page_base_address: u64) {
        self.0.set_bits(12usize..48usize, page_base_address)
    }
    #[allow(dead_code)]
    pub const fn width_of_page_base_address() -> usize {
        48usize - 12usize
    }
    #[allow(dead_code)]
    pub fn get_nG(&self) -> u64 {
        self.0.get_bits(11usize..12usize)
    }
    pub fn set_nG(&mut self, nG: u64) {
        self.0.set_bits(11usize..12usize, nG)
    }
    #[allow(dead_code)]
    pub const fn width_of_nG() -> usize {
        12usize - 11usize
    }
    #[allow(dead_code)]
    pub fn get_AF(&self) -> u64 {
        self.0.get_bits(10usize..11usize)
    }
    pub fn set_AF(&mut self, AF: u64) {
        self.0.set_bits(10usize..11usize, AF)
    }
    #[allow(dead_code)]
    pub const fn width_of_AF() -> usize {
        11usize - 10usize
    }
    #[allow(dead_code)]
    pub fn get_SH(&self) -> u64 {
        self.0.get_bits(8usize..10usize)
    }
    pub fn set_SH(&mut self, SH: u64) {
        self.0.set_bits(8usize..10usize, SH)
    }
    #[allow(dead_code)]
    pub const fn width_of_SH() -> usize {
        10usize - 8usize
    }
    #[allow(dead_code)]
    pub fn get_AP(&self) -> u64 {
        self.0.get_bits(6usize..8usize)
    }
    pub fn set_AP(&mut self, AP: u64) {
        self.0.set_bits(6usize..8usize, AP)
    }
    #[allow(dead_code)]
    pub const fn width_of_AP() -> usize {
        8usize - 6usize
    }
    #[allow(dead_code)]
    pub fn get_AttrIndx(&self) -> u64 {
        self.0.get_bits(2usize..5usize)
    }
    pub fn set_AttrIndx(&mut self, AttrIndx: u64) {
        self.0.set_bits(2usize..5usize, AttrIndx)
    }
    #[allow(dead_code)]
    pub const fn width_of_AttrIndx() -> usize {
        5usize - 2usize
    }
    #[allow(dead_code)]
    pub fn get_pte_hw_type(&self) -> u64 {
        self.0.get_bits(0usize..2usize)
    }
    pub fn set_pte_hw_type(&mut self, pte_hw_type: u64) {
        self.0.set_bits(0usize..2usize, pte_hw_type)
    }
    #[allow(dead_code)]
    pub const fn width_of_pte_hw_type() -> usize {
        2usize - 0usize
    }
}
impl fmt::Debug for pte_page {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct pte_page_Unpacked {
    pub pte_sw_type: u64,
    pub UXN: u64,
    pub page_base_address: u64,
    pub nG: u64,
    pub AF: u64,
    pub SH: u64,
    pub AP: u64,
    pub AttrIndx: u64,
    pub pte_hw_type: u64,
}
impl pte_page_Unpacked {
    pub fn pack(self) -> pte_page {
        match self {
            Self {
                pte_sw_type,
                UXN,
                page_base_address,
                nG,
                AF,
                SH,
                AP,
                AttrIndx,
                pte_hw_type,
            } => pte_page::new(
                pte_sw_type,
                UXN,
                page_base_address,
                nG,
                AF,
                SH,
                AP,
                AttrIndx,
                pte_hw_type,
            ),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct pte_4k_page(pub Bitfield<u64, 1usize>);
pub type pte_4k_page_t = pte_4k_page;
impl pte_4k_page {
    pub fn new(
        pte_sw_type: u64,
        UXN: u64,
        page_base_address: u64,
        nG: u64,
        AF: u64,
        SH: u64,
        AP: u64,
        AttrIndx: u64,
        pte_hw_type: u64,
    ) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_pte_sw_type(pte_sw_type);
        this.set_UXN(UXN);
        this.set_page_base_address(page_base_address);
        this.set_nG(nG);
        this.set_AF(AF);
        this.set_SH(SH);
        this.set_AP(AP);
        this.set_AttrIndx(AttrIndx);
        this.set_pte_hw_type(pte_hw_type);
        this
    }
    pub fn unpack(&self) -> pte_4k_page_Unpacked {
        pte_4k_page_Unpacked {
            pte_sw_type: self.get_pte_sw_type(),
            UXN: self.get_UXN(),
            page_base_address: self.get_page_base_address(),
            nG: self.get_nG(),
            AF: self.get_AF(),
            SH: self.get_SH(),
            AP: self.get_AP(),
            AttrIndx: self.get_AttrIndx(),
            pte_hw_type: self.get_pte_hw_type(),
        }
    }
    #[allow(dead_code)]
    pub fn get_pte_sw_type(&self) -> u64 {
        self.0.get_bits(58usize..59usize)
    }
    pub fn set_pte_sw_type(&mut self, pte_sw_type: u64) {
        self.0.set_bits(58usize..59usize, pte_sw_type)
    }
    #[allow(dead_code)]
    pub const fn width_of_pte_sw_type() -> usize {
        59usize - 58usize
    }
    #[allow(dead_code)]
    pub fn get_UXN(&self) -> u64 {
        self.0.get_bits(54usize..55usize)
    }
    pub fn set_UXN(&mut self, UXN: u64) {
        self.0.set_bits(54usize..55usize, UXN)
    }
    #[allow(dead_code)]
    pub const fn width_of_UXN() -> usize {
        55usize - 54usize
    }
    #[allow(dead_code)]
    pub fn get_page_base_address(&self) -> u64 {
        self.0.get_bits(12usize..48usize)
    }
    pub fn set_page_base_address(&mut self, page_base_address: u64) {
        self.0.set_bits(12usize..48usize, page_base_address)
    }
    #[allow(dead_code)]
    pub const fn width_of_page_base_address() -> usize {
        48usize - 12usize
    }
    #[allow(dead_code)]
    pub fn get_nG(&self) -> u64 {
        self.0.get_bits(11usize..12usize)
    }
    pub fn set_nG(&mut self, nG: u64) {
        self.0.set_bits(11usize..12usize, nG)
    }
    #[allow(dead_code)]
    pub const fn width_of_nG() -> usize {
        12usize - 11usize
    }
    #[allow(dead_code)]
    pub fn get_AF(&self) -> u64 {
        self.0.get_bits(10usize..11usize)
    }
    pub fn set_AF(&mut self, AF: u64) {
        self.0.set_bits(10usize..11usize, AF)
    }
    #[allow(dead_code)]
    pub const fn width_of_AF() -> usize {
        11usize - 10usize
    }
    #[allow(dead_code)]
    pub fn get_SH(&self) -> u64 {
        self.0.get_bits(8usize..10usize)
    }
    pub fn set_SH(&mut self, SH: u64) {
        self.0.set_bits(8usize..10usize, SH)
    }
    #[allow(dead_code)]
    pub const fn width_of_SH() -> usize {
        10usize - 8usize
    }
    #[allow(dead_code)]
    pub fn get_AP(&self) -> u64 {
        self.0.get_bits(6usize..8usize)
    }
    pub fn set_AP(&mut self, AP: u64) {
        self.0.set_bits(6usize..8usize, AP)
    }
    #[allow(dead_code)]
    pub const fn width_of_AP() -> usize {
        8usize - 6usize
    }
    #[allow(dead_code)]
    pub fn get_AttrIndx(&self) -> u64 {
        self.0.get_bits(2usize..5usize)
    }
    pub fn set_AttrIndx(&mut self, AttrIndx: u64) {
        self.0.set_bits(2usize..5usize, AttrIndx)
    }
    #[allow(dead_code)]
    pub const fn width_of_AttrIndx() -> usize {
        5usize - 2usize
    }
    #[allow(dead_code)]
    pub fn get_pte_hw_type(&self) -> u64 {
        self.0.get_bits(0usize..2usize)
    }
    pub fn set_pte_hw_type(&mut self, pte_hw_type: u64) {
        self.0.set_bits(0usize..2usize, pte_hw_type)
    }
    #[allow(dead_code)]
    pub const fn width_of_pte_hw_type() -> usize {
        2usize - 0usize
    }
}
impl fmt::Debug for pte_4k_page {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct pte_4k_page_Unpacked {
    pub pte_sw_type: u64,
    pub UXN: u64,
    pub page_base_address: u64,
    pub nG: u64,
    pub AF: u64,
    pub SH: u64,
    pub AP: u64,
    pub AttrIndx: u64,
    pub pte_hw_type: u64,
}
impl pte_4k_page_Unpacked {
    pub fn pack(self) -> pte_4k_page {
        match self {
            Self {
                pte_sw_type,
                UXN,
                page_base_address,
                nG,
                AF,
                SH,
                AP,
                AttrIndx,
                pte_hw_type,
            } => pte_4k_page::new(
                pte_sw_type,
                UXN,
                page_base_address,
                nG,
                AF,
                SH,
                AP,
                AttrIndx,
                pte_hw_type,
            ),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct pte_invalid(pub Bitfield<u64, 1usize>);
pub type pte_invalid_t = pte_invalid;
impl pte_invalid {
    pub fn new(pte_sw_type: u64, pte_hw_type: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_pte_sw_type(pte_sw_type);
        this.set_pte_hw_type(pte_hw_type);
        this
    }
    pub fn unpack(&self) -> pte_invalid_Unpacked {
        pte_invalid_Unpacked {
            pte_sw_type: self.get_pte_sw_type(),
            pte_hw_type: self.get_pte_hw_type(),
        }
    }
    #[allow(dead_code)]
    pub fn get_pte_sw_type(&self) -> u64 {
        self.0.get_bits(58usize..59usize)
    }
    pub fn set_pte_sw_type(&mut self, pte_sw_type: u64) {
        self.0.set_bits(58usize..59usize, pte_sw_type)
    }
    #[allow(dead_code)]
    pub const fn width_of_pte_sw_type() -> usize {
        59usize - 58usize
    }
    #[allow(dead_code)]
    pub fn get_pte_hw_type(&self) -> u64 {
        self.0.get_bits(0usize..2usize)
    }
    pub fn set_pte_hw_type(&mut self, pte_hw_type: u64) {
        self.0.set_bits(0usize..2usize, pte_hw_type)
    }
    #[allow(dead_code)]
    pub const fn width_of_pte_hw_type() -> usize {
        2usize - 0usize
    }
}
impl fmt::Debug for pte_invalid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct pte_invalid_Unpacked {
    pub pte_sw_type: u64,
    pub pte_hw_type: u64,
}
impl pte_invalid_Unpacked {
    pub fn pack(self) -> pte_invalid {
        match self {
            Self {
                pte_sw_type,
                pte_hw_type,
            } => pte_invalid::new(pte_sw_type, pte_hw_type),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct ttbr(pub Bitfield<u64, 1usize>);
pub type ttbr_t = ttbr;
impl ttbr {
    pub fn new(asid: u64, base_address: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_asid(asid);
        this.set_base_address(base_address);
        this
    }
    pub fn unpack(&self) -> ttbr_Unpacked {
        ttbr_Unpacked {
            asid: self.get_asid(),
            base_address: self.get_base_address(),
        }
    }
    #[allow(dead_code)]
    pub fn get_asid(&self) -> u64 {
        self.0.get_bits(48usize..64usize)
    }
    pub fn set_asid(&mut self, asid: u64) {
        self.0.set_bits(48usize..64usize, asid)
    }
    #[allow(dead_code)]
    pub const fn width_of_asid() -> usize {
        64usize - 48usize
    }
    #[allow(dead_code)]
    pub fn get_base_address(&self) -> u64 {
        self.0.get_bits(0usize..48usize)
    }
    pub fn set_base_address(&mut self, base_address: u64) {
        self.0.set_bits(0usize..48usize, base_address)
    }
    #[allow(dead_code)]
    pub const fn width_of_base_address() -> usize {
        48usize - 0usize
    }
}
impl fmt::Debug for ttbr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ttbr_Unpacked {
    pub asid: u64,
    pub base_address: u64,
}
impl ttbr_Unpacked {
    pub fn pack(self) -> ttbr {
        match self {
            Self { asid, base_address } => ttbr::new(asid, base_address),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct lookup_fault_invalid_root(pub Bitfield<u64, 2usize>);
impl lookup_fault_invalid_root {
    pub fn new() -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_lufType(lookup_fault_tag::lookup_fault_invalid_root);
        this
    }
    pub fn unpack(&self) -> lookup_fault_invalid_root_Unpacked {
        lookup_fault_invalid_root_Unpacked {}
    }
    #[allow(dead_code)]
    fn get_lufType(&self) -> u64 {
        self.0.get_bits(0usize..2usize)
    }
    fn set_lufType(&mut self, lufType: u64) {
        self.0.set_bits(0usize..2usize, lufType)
    }
    #[allow(dead_code)]
    const fn width_of_lufType() -> usize {
        2usize - 0usize
    }
}
impl fmt::Debug for lookup_fault_invalid_root {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct lookup_fault_invalid_root_Unpacked {}
impl lookup_fault_invalid_root_Unpacked {
    pub fn pack(self) -> lookup_fault_invalid_root {
        match self {
            Self {} => lookup_fault_invalid_root::new(),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct lookup_fault_missing_capability(pub Bitfield<u64, 2usize>);
impl lookup_fault_missing_capability {
    pub fn new(bitsLeft: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_bitsLeft(bitsLeft);
        this.set_lufType(lookup_fault_tag::lookup_fault_missing_capability);
        this
    }
    pub fn unpack(&self) -> lookup_fault_missing_capability_Unpacked {
        lookup_fault_missing_capability_Unpacked {
            bitsLeft: self.get_bitsLeft(),
        }
    }
    #[allow(dead_code)]
    pub fn get_bitsLeft(&self) -> u64 {
        self.0.get_bits(2usize..9usize)
    }
    pub fn set_bitsLeft(&mut self, bitsLeft: u64) {
        self.0.set_bits(2usize..9usize, bitsLeft)
    }
    #[allow(dead_code)]
    pub const fn width_of_bitsLeft() -> usize {
        9usize - 2usize
    }
    #[allow(dead_code)]
    fn get_lufType(&self) -> u64 {
        self.0.get_bits(0usize..2usize)
    }
    fn set_lufType(&mut self, lufType: u64) {
        self.0.set_bits(0usize..2usize, lufType)
    }
    #[allow(dead_code)]
    const fn width_of_lufType() -> usize {
        2usize - 0usize
    }
}
impl fmt::Debug for lookup_fault_missing_capability {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct lookup_fault_missing_capability_Unpacked {
    pub bitsLeft: u64,
}
impl lookup_fault_missing_capability_Unpacked {
    pub fn pack(self) -> lookup_fault_missing_capability {
        match self {
            Self { bitsLeft } => lookup_fault_missing_capability::new(bitsLeft),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct lookup_fault_depth_mismatch(pub Bitfield<u64, 2usize>);
impl lookup_fault_depth_mismatch {
    pub fn new(bitsFound: u64, bitsLeft: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_bitsFound(bitsFound);
        this.set_bitsLeft(bitsLeft);
        this.set_lufType(lookup_fault_tag::lookup_fault_depth_mismatch);
        this
    }
    pub fn unpack(&self) -> lookup_fault_depth_mismatch_Unpacked {
        lookup_fault_depth_mismatch_Unpacked {
            bitsFound: self.get_bitsFound(),
            bitsLeft: self.get_bitsLeft(),
        }
    }
    #[allow(dead_code)]
    pub fn get_bitsFound(&self) -> u64 {
        self.0.get_bits(9usize..16usize)
    }
    pub fn set_bitsFound(&mut self, bitsFound: u64) {
        self.0.set_bits(9usize..16usize, bitsFound)
    }
    #[allow(dead_code)]
    pub const fn width_of_bitsFound() -> usize {
        16usize - 9usize
    }
    #[allow(dead_code)]
    pub fn get_bitsLeft(&self) -> u64 {
        self.0.get_bits(2usize..9usize)
    }
    pub fn set_bitsLeft(&mut self, bitsLeft: u64) {
        self.0.set_bits(2usize..9usize, bitsLeft)
    }
    #[allow(dead_code)]
    pub const fn width_of_bitsLeft() -> usize {
        9usize - 2usize
    }
    #[allow(dead_code)]
    fn get_lufType(&self) -> u64 {
        self.0.get_bits(0usize..2usize)
    }
    fn set_lufType(&mut self, lufType: u64) {
        self.0.set_bits(0usize..2usize, lufType)
    }
    #[allow(dead_code)]
    const fn width_of_lufType() -> usize {
        2usize - 0usize
    }
}
impl fmt::Debug for lookup_fault_depth_mismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct lookup_fault_depth_mismatch_Unpacked {
    pub bitsFound: u64,
    pub bitsLeft: u64,
}
impl lookup_fault_depth_mismatch_Unpacked {
    pub fn pack(self) -> lookup_fault_depth_mismatch {
        match self {
            Self {
                bitsFound,
                bitsLeft,
            } => lookup_fault_depth_mismatch::new(bitsFound, bitsLeft),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct lookup_fault_guard_mismatch(pub Bitfield<u64, 2usize>);
impl lookup_fault_guard_mismatch {
    pub fn new(guardFound: u64, bitsLeft: u64, bitsFound: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_guardFound(guardFound);
        this.set_bitsLeft(bitsLeft);
        this.set_bitsFound(bitsFound);
        this.set_lufType(lookup_fault_tag::lookup_fault_guard_mismatch);
        this
    }
    pub fn unpack(&self) -> lookup_fault_guard_mismatch_Unpacked {
        lookup_fault_guard_mismatch_Unpacked {
            guardFound: self.get_guardFound(),
            bitsLeft: self.get_bitsLeft(),
            bitsFound: self.get_bitsFound(),
        }
    }
    #[allow(dead_code)]
    pub fn get_guardFound(&self) -> u64 {
        self.0.get_bits(64usize..128usize)
    }
    pub fn set_guardFound(&mut self, guardFound: u64) {
        self.0.set_bits(64usize..128usize, guardFound)
    }
    #[allow(dead_code)]
    pub const fn width_of_guardFound() -> usize {
        128usize - 64usize
    }
    #[allow(dead_code)]
    pub fn get_bitsLeft(&self) -> u64 {
        self.0.get_bits(9usize..16usize)
    }
    pub fn set_bitsLeft(&mut self, bitsLeft: u64) {
        self.0.set_bits(9usize..16usize, bitsLeft)
    }
    #[allow(dead_code)]
    pub const fn width_of_bitsLeft() -> usize {
        16usize - 9usize
    }
    #[allow(dead_code)]
    pub fn get_bitsFound(&self) -> u64 {
        self.0.get_bits(2usize..9usize)
    }
    pub fn set_bitsFound(&mut self, bitsFound: u64) {
        self.0.set_bits(2usize..9usize, bitsFound)
    }
    #[allow(dead_code)]
    pub const fn width_of_bitsFound() -> usize {
        9usize - 2usize
    }
    #[allow(dead_code)]
    fn get_lufType(&self) -> u64 {
        self.0.get_bits(0usize..2usize)
    }
    fn set_lufType(&mut self, lufType: u64) {
        self.0.set_bits(0usize..2usize, lufType)
    }
    #[allow(dead_code)]
    const fn width_of_lufType() -> usize {
        2usize - 0usize
    }
}
impl fmt::Debug for lookup_fault_guard_mismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct lookup_fault_guard_mismatch_Unpacked {
    pub guardFound: u64,
    pub bitsLeft: u64,
    pub bitsFound: u64,
}
impl lookup_fault_guard_mismatch_Unpacked {
    pub fn pack(self) -> lookup_fault_guard_mismatch {
        match self {
            Self {
                guardFound,
                bitsLeft,
                bitsFound,
            } => lookup_fault_guard_mismatch::new(guardFound, bitsLeft, bitsFound),
        }
    }
}
pub mod lookup_fault_tag {
    pub const lookup_fault_invalid_root: u64 = 0;
    pub const lookup_fault_missing_capability: u64 = 1;
    pub const lookup_fault_depth_mismatch: u64 = 2;
    pub const lookup_fault_guard_mismatch: u64 = 3;
}
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct lookup_fault(pub Bitfield<u64, 2usize>);
impl lookup_fault {
    pub fn splay(self) -> lookup_fault_Splayed {
        match self.get_tag() {
            lookup_fault_tag::lookup_fault_invalid_root => {
                lookup_fault_Splayed::invalid_root(lookup_fault_invalid_root(self.0))
            }
            lookup_fault_tag::lookup_fault_missing_capability => {
                lookup_fault_Splayed::missing_capability(lookup_fault_missing_capability(self.0))
            }
            lookup_fault_tag::lookup_fault_depth_mismatch => {
                lookup_fault_Splayed::depth_mismatch(lookup_fault_depth_mismatch(self.0))
            }
            lookup_fault_tag::lookup_fault_guard_mismatch => {
                lookup_fault_Splayed::guard_mismatch(lookup_fault_guard_mismatch(self.0))
            }
            _ => panic!(),
        }
    }
    pub fn get_tag(&self) -> u64 {
        self.0.get_bits(0usize..2usize)
    }
}
impl fmt::Debug for lookup_fault {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.clone().splay().fmt(f)?;
        write!(f, ".unsplay()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum lookup_fault_Splayed {
    invalid_root(lookup_fault_invalid_root),
    missing_capability(lookup_fault_missing_capability),
    depth_mismatch(lookup_fault_depth_mismatch),
    guard_mismatch(lookup_fault_guard_mismatch),
}
impl lookup_fault_Splayed {
    pub fn unsplay(self) -> lookup_fault {
        match self {
            lookup_fault_Splayed::invalid_root(lookup_fault_invalid_root(bitfield)) => {
                lookup_fault(bitfield)
            }
            lookup_fault_Splayed::missing_capability(lookup_fault_missing_capability(bitfield)) => {
                lookup_fault(bitfield)
            }
            lookup_fault_Splayed::depth_mismatch(lookup_fault_depth_mismatch(bitfield)) => {
                lookup_fault(bitfield)
            }
            lookup_fault_Splayed::guard_mismatch(lookup_fault_guard_mismatch(bitfield)) => {
                lookup_fault(bitfield)
            }
        }
    }
}
impl lookup_fault_invalid_root {
    pub fn unsplay(self) -> lookup_fault {
        lookup_fault(self.0)
    }
}
impl lookup_fault_invalid_root_Unpacked {
    pub fn unsplay(self) -> lookup_fault {
        self.pack().unsplay()
    }
}
impl lookup_fault_missing_capability {
    pub fn unsplay(self) -> lookup_fault {
        lookup_fault(self.0)
    }
}
impl lookup_fault_missing_capability_Unpacked {
    pub fn unsplay(self) -> lookup_fault {
        self.pack().unsplay()
    }
}
impl lookup_fault_depth_mismatch {
    pub fn unsplay(self) -> lookup_fault {
        lookup_fault(self.0)
    }
}
impl lookup_fault_depth_mismatch_Unpacked {
    pub fn unsplay(self) -> lookup_fault {
        self.pack().unsplay()
    }
}
impl lookup_fault_guard_mismatch {
    pub fn unsplay(self) -> lookup_fault {
        lookup_fault(self.0)
    }
}
impl lookup_fault_guard_mismatch_Unpacked {
    pub fn unsplay(self) -> lookup_fault {
        self.pack().unsplay()
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct cap_null_cap(pub Bitfield<u64, 2usize>);
impl cap_null_cap {
    pub fn new() -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_capType(cap_tag::cap_null_cap);
        this
    }
    pub fn unpack(&self) -> cap_null_cap_Unpacked {
        cap_null_cap_Unpacked {}
    }
    #[allow(dead_code)]
    fn get_capType(&self) -> u64 {
        self.0.get_bits(59usize..64usize)
    }
    fn set_capType(&mut self, capType: u64) {
        self.0.set_bits(59usize..64usize, capType)
    }
    #[allow(dead_code)]
    const fn width_of_capType() -> usize {
        64usize - 59usize
    }
}
impl fmt::Debug for cap_null_cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct cap_null_cap_Unpacked {}
impl cap_null_cap_Unpacked {
    pub fn pack(self) -> cap_null_cap {
        match self {
            Self {} => cap_null_cap::new(),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct cap_untyped_cap(pub Bitfield<u64, 2usize>);
impl cap_untyped_cap {
    pub fn new(capFreeIndex: u64, capIsDevice: u64, capBlockSize: u64, capPtr: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_capFreeIndex(capFreeIndex);
        this.set_capIsDevice(capIsDevice);
        this.set_capBlockSize(capBlockSize);
        this.set_capType(cap_tag::cap_untyped_cap);
        this.set_capPtr(capPtr);
        this
    }
    pub fn unpack(&self) -> cap_untyped_cap_Unpacked {
        cap_untyped_cap_Unpacked {
            capFreeIndex: self.get_capFreeIndex(),
            capIsDevice: self.get_capIsDevice(),
            capBlockSize: self.get_capBlockSize(),
            capPtr: self.get_capPtr(),
        }
    }
    #[allow(dead_code)]
    pub fn get_capFreeIndex(&self) -> u64 {
        self.0.get_bits(80usize..128usize)
    }
    pub fn set_capFreeIndex(&mut self, capFreeIndex: u64) {
        self.0.set_bits(80usize..128usize, capFreeIndex)
    }
    #[allow(dead_code)]
    pub const fn width_of_capFreeIndex() -> usize {
        128usize - 80usize
    }
    #[allow(dead_code)]
    pub fn get_capIsDevice(&self) -> u64 {
        self.0.get_bits(70usize..71usize)
    }
    pub fn set_capIsDevice(&mut self, capIsDevice: u64) {
        self.0.set_bits(70usize..71usize, capIsDevice)
    }
    #[allow(dead_code)]
    pub const fn width_of_capIsDevice() -> usize {
        71usize - 70usize
    }
    #[allow(dead_code)]
    pub fn get_capBlockSize(&self) -> u64 {
        self.0.get_bits(64usize..70usize)
    }
    pub fn set_capBlockSize(&mut self, capBlockSize: u64) {
        self.0.set_bits(64usize..70usize, capBlockSize)
    }
    #[allow(dead_code)]
    pub const fn width_of_capBlockSize() -> usize {
        70usize - 64usize
    }
    #[allow(dead_code)]
    fn get_capType(&self) -> u64 {
        self.0.get_bits(59usize..64usize)
    }
    fn set_capType(&mut self, capType: u64) {
        self.0.set_bits(59usize..64usize, capType)
    }
    #[allow(dead_code)]
    const fn width_of_capType() -> usize {
        64usize - 59usize
    }
    #[allow(dead_code)]
    pub fn get_capPtr(&self) -> u64 {
        self.0.get_bits(0usize..48usize)
    }
    pub fn set_capPtr(&mut self, capPtr: u64) {
        self.0.set_bits(0usize..48usize, capPtr)
    }
    #[allow(dead_code)]
    pub const fn width_of_capPtr() -> usize {
        48usize - 0usize
    }
}
impl fmt::Debug for cap_untyped_cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct cap_untyped_cap_Unpacked {
    pub capFreeIndex: u64,
    pub capIsDevice: u64,
    pub capBlockSize: u64,
    pub capPtr: u64,
}
impl cap_untyped_cap_Unpacked {
    pub fn pack(self) -> cap_untyped_cap {
        match self {
            Self {
                capFreeIndex,
                capIsDevice,
                capBlockSize,
                capPtr,
            } => cap_untyped_cap::new(capFreeIndex, capIsDevice, capBlockSize, capPtr),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct cap_endpoint_cap(pub Bitfield<u64, 2usize>);
impl cap_endpoint_cap {
    pub fn new(
        capEPBadge: u64,
        capCanGrantReply: u64,
        capCanGrant: u64,
        capCanReceive: u64,
        capCanSend: u64,
        capEPPtr: u64,
    ) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_capEPBadge(capEPBadge);
        this.set_capType(cap_tag::cap_endpoint_cap);
        this.set_capCanGrantReply(capCanGrantReply);
        this.set_capCanGrant(capCanGrant);
        this.set_capCanReceive(capCanReceive);
        this.set_capCanSend(capCanSend);
        this.set_capEPPtr(capEPPtr);
        this
    }
    pub fn unpack(&self) -> cap_endpoint_cap_Unpacked {
        cap_endpoint_cap_Unpacked {
            capEPBadge: self.get_capEPBadge(),
            capCanGrantReply: self.get_capCanGrantReply(),
            capCanGrant: self.get_capCanGrant(),
            capCanReceive: self.get_capCanReceive(),
            capCanSend: self.get_capCanSend(),
            capEPPtr: self.get_capEPPtr(),
        }
    }
    #[allow(dead_code)]
    pub fn get_capEPBadge(&self) -> u64 {
        self.0.get_bits(64usize..128usize)
    }
    pub fn set_capEPBadge(&mut self, capEPBadge: u64) {
        self.0.set_bits(64usize..128usize, capEPBadge)
    }
    #[allow(dead_code)]
    pub const fn width_of_capEPBadge() -> usize {
        128usize - 64usize
    }
    #[allow(dead_code)]
    fn get_capType(&self) -> u64 {
        self.0.get_bits(59usize..64usize)
    }
    fn set_capType(&mut self, capType: u64) {
        self.0.set_bits(59usize..64usize, capType)
    }
    #[allow(dead_code)]
    const fn width_of_capType() -> usize {
        64usize - 59usize
    }
    #[allow(dead_code)]
    pub fn get_capCanGrantReply(&self) -> u64 {
        self.0.get_bits(58usize..59usize)
    }
    pub fn set_capCanGrantReply(&mut self, capCanGrantReply: u64) {
        self.0.set_bits(58usize..59usize, capCanGrantReply)
    }
    #[allow(dead_code)]
    pub const fn width_of_capCanGrantReply() -> usize {
        59usize - 58usize
    }
    #[allow(dead_code)]
    pub fn get_capCanGrant(&self) -> u64 {
        self.0.get_bits(57usize..58usize)
    }
    pub fn set_capCanGrant(&mut self, capCanGrant: u64) {
        self.0.set_bits(57usize..58usize, capCanGrant)
    }
    #[allow(dead_code)]
    pub const fn width_of_capCanGrant() -> usize {
        58usize - 57usize
    }
    #[allow(dead_code)]
    pub fn get_capCanReceive(&self) -> u64 {
        self.0.get_bits(56usize..57usize)
    }
    pub fn set_capCanReceive(&mut self, capCanReceive: u64) {
        self.0.set_bits(56usize..57usize, capCanReceive)
    }
    #[allow(dead_code)]
    pub const fn width_of_capCanReceive() -> usize {
        57usize - 56usize
    }
    #[allow(dead_code)]
    pub fn get_capCanSend(&self) -> u64 {
        self.0.get_bits(55usize..56usize)
    }
    pub fn set_capCanSend(&mut self, capCanSend: u64) {
        self.0.set_bits(55usize..56usize, capCanSend)
    }
    #[allow(dead_code)]
    pub const fn width_of_capCanSend() -> usize {
        56usize - 55usize
    }
    #[allow(dead_code)]
    pub fn get_capEPPtr(&self) -> u64 {
        self.0.get_bits(0usize..48usize)
    }
    pub fn set_capEPPtr(&mut self, capEPPtr: u64) {
        self.0.set_bits(0usize..48usize, capEPPtr)
    }
    #[allow(dead_code)]
    pub const fn width_of_capEPPtr() -> usize {
        48usize - 0usize
    }
}
impl fmt::Debug for cap_endpoint_cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct cap_endpoint_cap_Unpacked {
    pub capEPBadge: u64,
    pub capCanGrantReply: u64,
    pub capCanGrant: u64,
    pub capCanReceive: u64,
    pub capCanSend: u64,
    pub capEPPtr: u64,
}
impl cap_endpoint_cap_Unpacked {
    pub fn pack(self) -> cap_endpoint_cap {
        match self {
            Self {
                capEPBadge,
                capCanGrantReply,
                capCanGrant,
                capCanReceive,
                capCanSend,
                capEPPtr,
            } => cap_endpoint_cap::new(
                capEPBadge,
                capCanGrantReply,
                capCanGrant,
                capCanReceive,
                capCanSend,
                capEPPtr,
            ),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct cap_notification_cap(pub Bitfield<u64, 2usize>);
impl cap_notification_cap {
    pub fn new(
        capNtfnBadge: u64,
        capNtfnCanReceive: u64,
        capNtfnCanSend: u64,
        capNtfnPtr: u64,
    ) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_capNtfnBadge(capNtfnBadge);
        this.set_capType(cap_tag::cap_notification_cap);
        this.set_capNtfnCanReceive(capNtfnCanReceive);
        this.set_capNtfnCanSend(capNtfnCanSend);
        this.set_capNtfnPtr(capNtfnPtr);
        this
    }
    pub fn unpack(&self) -> cap_notification_cap_Unpacked {
        cap_notification_cap_Unpacked {
            capNtfnBadge: self.get_capNtfnBadge(),
            capNtfnCanReceive: self.get_capNtfnCanReceive(),
            capNtfnCanSend: self.get_capNtfnCanSend(),
            capNtfnPtr: self.get_capNtfnPtr(),
        }
    }
    #[allow(dead_code)]
    pub fn get_capNtfnBadge(&self) -> u64 {
        self.0.get_bits(64usize..128usize)
    }
    pub fn set_capNtfnBadge(&mut self, capNtfnBadge: u64) {
        self.0.set_bits(64usize..128usize, capNtfnBadge)
    }
    #[allow(dead_code)]
    pub const fn width_of_capNtfnBadge() -> usize {
        128usize - 64usize
    }
    #[allow(dead_code)]
    fn get_capType(&self) -> u64 {
        self.0.get_bits(59usize..64usize)
    }
    fn set_capType(&mut self, capType: u64) {
        self.0.set_bits(59usize..64usize, capType)
    }
    #[allow(dead_code)]
    const fn width_of_capType() -> usize {
        64usize - 59usize
    }
    #[allow(dead_code)]
    pub fn get_capNtfnCanReceive(&self) -> u64 {
        self.0.get_bits(58usize..59usize)
    }
    pub fn set_capNtfnCanReceive(&mut self, capNtfnCanReceive: u64) {
        self.0.set_bits(58usize..59usize, capNtfnCanReceive)
    }
    #[allow(dead_code)]
    pub const fn width_of_capNtfnCanReceive() -> usize {
        59usize - 58usize
    }
    #[allow(dead_code)]
    pub fn get_capNtfnCanSend(&self) -> u64 {
        self.0.get_bits(57usize..58usize)
    }
    pub fn set_capNtfnCanSend(&mut self, capNtfnCanSend: u64) {
        self.0.set_bits(57usize..58usize, capNtfnCanSend)
    }
    #[allow(dead_code)]
    pub const fn width_of_capNtfnCanSend() -> usize {
        58usize - 57usize
    }
    #[allow(dead_code)]
    pub fn get_capNtfnPtr(&self) -> u64 {
        self.0.get_bits(0usize..48usize)
    }
    pub fn set_capNtfnPtr(&mut self, capNtfnPtr: u64) {
        self.0.set_bits(0usize..48usize, capNtfnPtr)
    }
    #[allow(dead_code)]
    pub const fn width_of_capNtfnPtr() -> usize {
        48usize - 0usize
    }
}
impl fmt::Debug for cap_notification_cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct cap_notification_cap_Unpacked {
    pub capNtfnBadge: u64,
    pub capNtfnCanReceive: u64,
    pub capNtfnCanSend: u64,
    pub capNtfnPtr: u64,
}
impl cap_notification_cap_Unpacked {
    pub fn pack(self) -> cap_notification_cap {
        match self {
            Self {
                capNtfnBadge,
                capNtfnCanReceive,
                capNtfnCanSend,
                capNtfnPtr,
            } => cap_notification_cap::new(
                capNtfnBadge,
                capNtfnCanReceive,
                capNtfnCanSend,
                capNtfnPtr,
            ),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct cap_reply_cap(pub Bitfield<u64, 2usize>);
impl cap_reply_cap {
    pub fn new(capTCBPtr: u64, capReplyCanGrant: u64, capReplyMaster: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_capTCBPtr(capTCBPtr);
        this.set_capType(cap_tag::cap_reply_cap);
        this.set_capReplyCanGrant(capReplyCanGrant);
        this.set_capReplyMaster(capReplyMaster);
        this
    }
    pub fn unpack(&self) -> cap_reply_cap_Unpacked {
        cap_reply_cap_Unpacked {
            capTCBPtr: self.get_capTCBPtr(),
            capReplyCanGrant: self.get_capReplyCanGrant(),
            capReplyMaster: self.get_capReplyMaster(),
        }
    }
    #[allow(dead_code)]
    pub fn get_capTCBPtr(&self) -> u64 {
        self.0.get_bits(64usize..128usize)
    }
    pub fn set_capTCBPtr(&mut self, capTCBPtr: u64) {
        self.0.set_bits(64usize..128usize, capTCBPtr)
    }
    #[allow(dead_code)]
    pub const fn width_of_capTCBPtr() -> usize {
        128usize - 64usize
    }
    #[allow(dead_code)]
    fn get_capType(&self) -> u64 {
        self.0.get_bits(59usize..64usize)
    }
    fn set_capType(&mut self, capType: u64) {
        self.0.set_bits(59usize..64usize, capType)
    }
    #[allow(dead_code)]
    const fn width_of_capType() -> usize {
        64usize - 59usize
    }
    #[allow(dead_code)]
    pub fn get_capReplyCanGrant(&self) -> u64 {
        self.0.get_bits(1usize..2usize)
    }
    pub fn set_capReplyCanGrant(&mut self, capReplyCanGrant: u64) {
        self.0.set_bits(1usize..2usize, capReplyCanGrant)
    }
    #[allow(dead_code)]
    pub const fn width_of_capReplyCanGrant() -> usize {
        2usize - 1usize
    }
    #[allow(dead_code)]
    pub fn get_capReplyMaster(&self) -> u64 {
        self.0.get_bits(0usize..1usize)
    }
    pub fn set_capReplyMaster(&mut self, capReplyMaster: u64) {
        self.0.set_bits(0usize..1usize, capReplyMaster)
    }
    #[allow(dead_code)]
    pub const fn width_of_capReplyMaster() -> usize {
        1usize - 0usize
    }
}
impl fmt::Debug for cap_reply_cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct cap_reply_cap_Unpacked {
    pub capTCBPtr: u64,
    pub capReplyCanGrant: u64,
    pub capReplyMaster: u64,
}
impl cap_reply_cap_Unpacked {
    pub fn pack(self) -> cap_reply_cap {
        match self {
            Self {
                capTCBPtr,
                capReplyCanGrant,
                capReplyMaster,
            } => cap_reply_cap::new(capTCBPtr, capReplyCanGrant, capReplyMaster),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct cap_cnode_cap(pub Bitfield<u64, 2usize>);
impl cap_cnode_cap {
    pub fn new(
        capCNodeGuard: u64,
        capCNodeGuardSize: u64,
        capCNodeRadix: u64,
        capCNodePtr: u64,
    ) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_capCNodeGuard(capCNodeGuard);
        this.set_capType(cap_tag::cap_cnode_cap);
        this.set_capCNodeGuardSize(capCNodeGuardSize);
        this.set_capCNodeRadix(capCNodeRadix);
        this.set_capCNodePtr(capCNodePtr);
        this
    }
    pub fn unpack(&self) -> cap_cnode_cap_Unpacked {
        cap_cnode_cap_Unpacked {
            capCNodeGuard: self.get_capCNodeGuard(),
            capCNodeGuardSize: self.get_capCNodeGuardSize(),
            capCNodeRadix: self.get_capCNodeRadix(),
            capCNodePtr: self.get_capCNodePtr(),
        }
    }
    #[allow(dead_code)]
    pub fn get_capCNodeGuard(&self) -> u64 {
        self.0.get_bits(64usize..128usize)
    }
    pub fn set_capCNodeGuard(&mut self, capCNodeGuard: u64) {
        self.0.set_bits(64usize..128usize, capCNodeGuard)
    }
    #[allow(dead_code)]
    pub const fn width_of_capCNodeGuard() -> usize {
        128usize - 64usize
    }
    #[allow(dead_code)]
    fn get_capType(&self) -> u64 {
        self.0.get_bits(59usize..64usize)
    }
    fn set_capType(&mut self, capType: u64) {
        self.0.set_bits(59usize..64usize, capType)
    }
    #[allow(dead_code)]
    const fn width_of_capType() -> usize {
        64usize - 59usize
    }
    #[allow(dead_code)]
    pub fn get_capCNodeGuardSize(&self) -> u64 {
        self.0.get_bits(53usize..59usize)
    }
    pub fn set_capCNodeGuardSize(&mut self, capCNodeGuardSize: u64) {
        self.0.set_bits(53usize..59usize, capCNodeGuardSize)
    }
    #[allow(dead_code)]
    pub const fn width_of_capCNodeGuardSize() -> usize {
        59usize - 53usize
    }
    #[allow(dead_code)]
    pub fn get_capCNodeRadix(&self) -> u64 {
        self.0.get_bits(47usize..53usize)
    }
    pub fn set_capCNodeRadix(&mut self, capCNodeRadix: u64) {
        self.0.set_bits(47usize..53usize, capCNodeRadix)
    }
    #[allow(dead_code)]
    pub const fn width_of_capCNodeRadix() -> usize {
        53usize - 47usize
    }
    #[allow(dead_code)]
    pub fn get_capCNodePtr(&self) -> u64 {
        self.0.get_bits(0usize..47usize)
    }
    pub fn set_capCNodePtr(&mut self, capCNodePtr: u64) {
        self.0.set_bits(0usize..47usize, capCNodePtr)
    }
    #[allow(dead_code)]
    pub const fn width_of_capCNodePtr() -> usize {
        47usize - 0usize
    }
}
impl fmt::Debug for cap_cnode_cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct cap_cnode_cap_Unpacked {
    pub capCNodeGuard: u64,
    pub capCNodeGuardSize: u64,
    pub capCNodeRadix: u64,
    pub capCNodePtr: u64,
}
impl cap_cnode_cap_Unpacked {
    pub fn pack(self) -> cap_cnode_cap {
        match self {
            Self {
                capCNodeGuard,
                capCNodeGuardSize,
                capCNodeRadix,
                capCNodePtr,
            } => cap_cnode_cap::new(capCNodeGuard, capCNodeGuardSize, capCNodeRadix, capCNodePtr),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct cap_thread_cap(pub Bitfield<u64, 2usize>);
impl cap_thread_cap {
    pub fn new(capTCBPtr: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_capType(cap_tag::cap_thread_cap);
        this.set_capTCBPtr(capTCBPtr);
        this
    }
    pub fn unpack(&self) -> cap_thread_cap_Unpacked {
        cap_thread_cap_Unpacked {
            capTCBPtr: self.get_capTCBPtr(),
        }
    }
    #[allow(dead_code)]
    fn get_capType(&self) -> u64 {
        self.0.get_bits(59usize..64usize)
    }
    fn set_capType(&mut self, capType: u64) {
        self.0.set_bits(59usize..64usize, capType)
    }
    #[allow(dead_code)]
    const fn width_of_capType() -> usize {
        64usize - 59usize
    }
    #[allow(dead_code)]
    pub fn get_capTCBPtr(&self) -> u64 {
        self.0.get_bits(0usize..48usize)
    }
    pub fn set_capTCBPtr(&mut self, capTCBPtr: u64) {
        self.0.set_bits(0usize..48usize, capTCBPtr)
    }
    #[allow(dead_code)]
    pub const fn width_of_capTCBPtr() -> usize {
        48usize - 0usize
    }
}
impl fmt::Debug for cap_thread_cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct cap_thread_cap_Unpacked {
    pub capTCBPtr: u64,
}
impl cap_thread_cap_Unpacked {
    pub fn pack(self) -> cap_thread_cap {
        match self {
            Self { capTCBPtr } => cap_thread_cap::new(capTCBPtr),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct cap_irq_control_cap(pub Bitfield<u64, 2usize>);
impl cap_irq_control_cap {
    pub fn new() -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_capType(cap_tag::cap_irq_control_cap);
        this
    }
    pub fn unpack(&self) -> cap_irq_control_cap_Unpacked {
        cap_irq_control_cap_Unpacked {}
    }
    #[allow(dead_code)]
    fn get_capType(&self) -> u64 {
        self.0.get_bits(59usize..64usize)
    }
    fn set_capType(&mut self, capType: u64) {
        self.0.set_bits(59usize..64usize, capType)
    }
    #[allow(dead_code)]
    const fn width_of_capType() -> usize {
        64usize - 59usize
    }
}
impl fmt::Debug for cap_irq_control_cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct cap_irq_control_cap_Unpacked {}
impl cap_irq_control_cap_Unpacked {
    pub fn pack(self) -> cap_irq_control_cap {
        match self {
            Self {} => cap_irq_control_cap::new(),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct cap_irq_handler_cap(pub Bitfield<u64, 2usize>);
impl cap_irq_handler_cap {
    pub fn new(capIRQ: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_capIRQ(capIRQ);
        this.set_capType(cap_tag::cap_irq_handler_cap);
        this
    }
    pub fn unpack(&self) -> cap_irq_handler_cap_Unpacked {
        cap_irq_handler_cap_Unpacked {
            capIRQ: self.get_capIRQ(),
        }
    }
    #[allow(dead_code)]
    pub fn get_capIRQ(&self) -> u64 {
        self.0.get_bits(64usize..76usize)
    }
    pub fn set_capIRQ(&mut self, capIRQ: u64) {
        self.0.set_bits(64usize..76usize, capIRQ)
    }
    #[allow(dead_code)]
    pub const fn width_of_capIRQ() -> usize {
        76usize - 64usize
    }
    #[allow(dead_code)]
    fn get_capType(&self) -> u64 {
        self.0.get_bits(59usize..64usize)
    }
    fn set_capType(&mut self, capType: u64) {
        self.0.set_bits(59usize..64usize, capType)
    }
    #[allow(dead_code)]
    const fn width_of_capType() -> usize {
        64usize - 59usize
    }
}
impl fmt::Debug for cap_irq_handler_cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct cap_irq_handler_cap_Unpacked {
    pub capIRQ: u64,
}
impl cap_irq_handler_cap_Unpacked {
    pub fn pack(self) -> cap_irq_handler_cap {
        match self {
            Self { capIRQ } => cap_irq_handler_cap::new(capIRQ),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct cap_zombie_cap(pub Bitfield<u64, 2usize>);
impl cap_zombie_cap {
    pub fn new(capZombieID: u64, capZombieType: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_capZombieID(capZombieID);
        this.set_capType(cap_tag::cap_zombie_cap);
        this.set_capZombieType(capZombieType);
        this
    }
    pub fn unpack(&self) -> cap_zombie_cap_Unpacked {
        cap_zombie_cap_Unpacked {
            capZombieID: self.get_capZombieID(),
            capZombieType: self.get_capZombieType(),
        }
    }
    #[allow(dead_code)]
    pub fn get_capZombieID(&self) -> u64 {
        self.0.get_bits(64usize..128usize)
    }
    pub fn set_capZombieID(&mut self, capZombieID: u64) {
        self.0.set_bits(64usize..128usize, capZombieID)
    }
    #[allow(dead_code)]
    pub const fn width_of_capZombieID() -> usize {
        128usize - 64usize
    }
    #[allow(dead_code)]
    fn get_capType(&self) -> u64 {
        self.0.get_bits(59usize..64usize)
    }
    fn set_capType(&mut self, capType: u64) {
        self.0.set_bits(59usize..64usize, capType)
    }
    #[allow(dead_code)]
    const fn width_of_capType() -> usize {
        64usize - 59usize
    }
    #[allow(dead_code)]
    pub fn get_capZombieType(&self) -> u64 {
        self.0.get_bits(0usize..7usize)
    }
    pub fn set_capZombieType(&mut self, capZombieType: u64) {
        self.0.set_bits(0usize..7usize, capZombieType)
    }
    #[allow(dead_code)]
    pub const fn width_of_capZombieType() -> usize {
        7usize - 0usize
    }
}
impl fmt::Debug for cap_zombie_cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct cap_zombie_cap_Unpacked {
    pub capZombieID: u64,
    pub capZombieType: u64,
}
impl cap_zombie_cap_Unpacked {
    pub fn pack(self) -> cap_zombie_cap {
        match self {
            Self {
                capZombieID,
                capZombieType,
            } => cap_zombie_cap::new(capZombieID, capZombieType),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct cap_domain_cap(pub Bitfield<u64, 2usize>);
impl cap_domain_cap {
    pub fn new() -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_capType(cap_tag::cap_domain_cap);
        this
    }
    pub fn unpack(&self) -> cap_domain_cap_Unpacked {
        cap_domain_cap_Unpacked {}
    }
    #[allow(dead_code)]
    fn get_capType(&self) -> u64 {
        self.0.get_bits(59usize..64usize)
    }
    fn set_capType(&mut self, capType: u64) {
        self.0.set_bits(59usize..64usize, capType)
    }
    #[allow(dead_code)]
    const fn width_of_capType() -> usize {
        64usize - 59usize
    }
}
impl fmt::Debug for cap_domain_cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct cap_domain_cap_Unpacked {}
impl cap_domain_cap_Unpacked {
    pub fn pack(self) -> cap_domain_cap {
        match self {
            Self {} => cap_domain_cap::new(),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct cap_frame_cap(pub Bitfield<u64, 2usize>);
impl cap_frame_cap {
    pub fn new(
        capFMappedASID: u64,
        capFBasePtr: u64,
        capFSize: u64,
        capFMappedAddress: u64,
        capFVMRights: u64,
        capFIsDevice: u64,
    ) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_capFMappedASID(capFMappedASID);
        this.set_capFBasePtr(capFBasePtr);
        this.set_capType(cap_tag::cap_frame_cap);
        this.set_capFSize(capFSize);
        this.set_capFMappedAddress(capFMappedAddress);
        this.set_capFVMRights(capFVMRights);
        this.set_capFIsDevice(capFIsDevice);
        this
    }
    pub fn unpack(&self) -> cap_frame_cap_Unpacked {
        cap_frame_cap_Unpacked {
            capFMappedASID: self.get_capFMappedASID(),
            capFBasePtr: self.get_capFBasePtr(),
            capFSize: self.get_capFSize(),
            capFMappedAddress: self.get_capFMappedAddress(),
            capFVMRights: self.get_capFVMRights(),
            capFIsDevice: self.get_capFIsDevice(),
        }
    }
    #[allow(dead_code)]
    pub fn get_capFMappedASID(&self) -> u64 {
        self.0.get_bits(112usize..128usize)
    }
    pub fn set_capFMappedASID(&mut self, capFMappedASID: u64) {
        self.0.set_bits(112usize..128usize, capFMappedASID)
    }
    #[allow(dead_code)]
    pub const fn width_of_capFMappedASID() -> usize {
        128usize - 112usize
    }
    #[allow(dead_code)]
    pub fn get_capFBasePtr(&self) -> u64 {
        self.0.get_bits(64usize..112usize)
    }
    pub fn set_capFBasePtr(&mut self, capFBasePtr: u64) {
        self.0.set_bits(64usize..112usize, capFBasePtr)
    }
    #[allow(dead_code)]
    pub const fn width_of_capFBasePtr() -> usize {
        112usize - 64usize
    }
    #[allow(dead_code)]
    fn get_capType(&self) -> u64 {
        self.0.get_bits(59usize..64usize)
    }
    fn set_capType(&mut self, capType: u64) {
        self.0.set_bits(59usize..64usize, capType)
    }
    #[allow(dead_code)]
    const fn width_of_capType() -> usize {
        64usize - 59usize
    }
    #[allow(dead_code)]
    pub fn get_capFSize(&self) -> u64 {
        self.0.get_bits(57usize..59usize)
    }
    pub fn set_capFSize(&mut self, capFSize: u64) {
        self.0.set_bits(57usize..59usize, capFSize)
    }
    #[allow(dead_code)]
    pub const fn width_of_capFSize() -> usize {
        59usize - 57usize
    }
    #[allow(dead_code)]
    pub fn get_capFMappedAddress(&self) -> u64 {
        self.0.get_bits(9usize..57usize)
    }
    pub fn set_capFMappedAddress(&mut self, capFMappedAddress: u64) {
        self.0.set_bits(9usize..57usize, capFMappedAddress)
    }
    #[allow(dead_code)]
    pub const fn width_of_capFMappedAddress() -> usize {
        57usize - 9usize
    }
    #[allow(dead_code)]
    pub fn get_capFVMRights(&self) -> u64 {
        self.0.get_bits(7usize..9usize)
    }
    pub fn set_capFVMRights(&mut self, capFVMRights: u64) {
        self.0.set_bits(7usize..9usize, capFVMRights)
    }
    #[allow(dead_code)]
    pub const fn width_of_capFVMRights() -> usize {
        9usize - 7usize
    }
    #[allow(dead_code)]
    pub fn get_capFIsDevice(&self) -> u64 {
        self.0.get_bits(6usize..7usize)
    }
    pub fn set_capFIsDevice(&mut self, capFIsDevice: u64) {
        self.0.set_bits(6usize..7usize, capFIsDevice)
    }
    #[allow(dead_code)]
    pub const fn width_of_capFIsDevice() -> usize {
        7usize - 6usize
    }
}
impl fmt::Debug for cap_frame_cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct cap_frame_cap_Unpacked {
    pub capFMappedASID: u64,
    pub capFBasePtr: u64,
    pub capFSize: u64,
    pub capFMappedAddress: u64,
    pub capFVMRights: u64,
    pub capFIsDevice: u64,
}
impl cap_frame_cap_Unpacked {
    pub fn pack(self) -> cap_frame_cap {
        match self {
            Self {
                capFMappedASID,
                capFBasePtr,
                capFSize,
                capFMappedAddress,
                capFVMRights,
                capFIsDevice,
            } => cap_frame_cap::new(
                capFMappedASID,
                capFBasePtr,
                capFSize,
                capFMappedAddress,
                capFVMRights,
                capFIsDevice,
            ),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct cap_page_table_cap(pub Bitfield<u64, 2usize>);
impl cap_page_table_cap {
    pub fn new(
        capPTMappedASID: u64,
        capPTBasePtr: u64,
        capPTIsMapped: u64,
        capPTMappedAddress: u64,
    ) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_capPTMappedASID(capPTMappedASID);
        this.set_capPTBasePtr(capPTBasePtr);
        this.set_capType(cap_tag::cap_page_table_cap);
        this.set_capPTIsMapped(capPTIsMapped);
        this.set_capPTMappedAddress(capPTMappedAddress);
        this
    }
    pub fn unpack(&self) -> cap_page_table_cap_Unpacked {
        cap_page_table_cap_Unpacked {
            capPTMappedASID: self.get_capPTMappedASID(),
            capPTBasePtr: self.get_capPTBasePtr(),
            capPTIsMapped: self.get_capPTIsMapped(),
            capPTMappedAddress: self.get_capPTMappedAddress(),
        }
    }
    #[allow(dead_code)]
    pub fn get_capPTMappedASID(&self) -> u64 {
        self.0.get_bits(112usize..128usize)
    }
    pub fn set_capPTMappedASID(&mut self, capPTMappedASID: u64) {
        self.0.set_bits(112usize..128usize, capPTMappedASID)
    }
    #[allow(dead_code)]
    pub const fn width_of_capPTMappedASID() -> usize {
        128usize - 112usize
    }
    #[allow(dead_code)]
    pub fn get_capPTBasePtr(&self) -> u64 {
        self.0.get_bits(64usize..112usize)
    }
    pub fn set_capPTBasePtr(&mut self, capPTBasePtr: u64) {
        self.0.set_bits(64usize..112usize, capPTBasePtr)
    }
    #[allow(dead_code)]
    pub const fn width_of_capPTBasePtr() -> usize {
        112usize - 64usize
    }
    #[allow(dead_code)]
    fn get_capType(&self) -> u64 {
        self.0.get_bits(59usize..64usize)
    }
    fn set_capType(&mut self, capType: u64) {
        self.0.set_bits(59usize..64usize, capType)
    }
    #[allow(dead_code)]
    const fn width_of_capType() -> usize {
        64usize - 59usize
    }
    #[allow(dead_code)]
    pub fn get_capPTIsMapped(&self) -> u64 {
        self.0.get_bits(48usize..49usize)
    }
    pub fn set_capPTIsMapped(&mut self, capPTIsMapped: u64) {
        self.0.set_bits(48usize..49usize, capPTIsMapped)
    }
    #[allow(dead_code)]
    pub const fn width_of_capPTIsMapped() -> usize {
        49usize - 48usize
    }
    #[allow(dead_code)]
    pub fn get_capPTMappedAddress(&self) -> u64 {
        self.0.get_bits(20usize..48usize)
    }
    pub fn set_capPTMappedAddress(&mut self, capPTMappedAddress: u64) {
        self.0.set_bits(20usize..48usize, capPTMappedAddress)
    }
    #[allow(dead_code)]
    pub const fn width_of_capPTMappedAddress() -> usize {
        48usize - 20usize
    }
}
impl fmt::Debug for cap_page_table_cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct cap_page_table_cap_Unpacked {
    pub capPTMappedASID: u64,
    pub capPTBasePtr: u64,
    pub capPTIsMapped: u64,
    pub capPTMappedAddress: u64,
}
impl cap_page_table_cap_Unpacked {
    pub fn pack(self) -> cap_page_table_cap {
        match self {
            Self {
                capPTMappedASID,
                capPTBasePtr,
                capPTIsMapped,
                capPTMappedAddress,
            } => cap_page_table_cap::new(
                capPTMappedASID,
                capPTBasePtr,
                capPTIsMapped,
                capPTMappedAddress,
            ),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct cap_vspace_cap(pub Bitfield<u64, 2usize>);
impl cap_vspace_cap {
    pub fn new(capVSMappedASID: u64, capVSBasePtr: u64, capVSIsMapped: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_capVSMappedASID(capVSMappedASID);
        this.set_capVSBasePtr(capVSBasePtr);
        this.set_capType(cap_tag::cap_vspace_cap);
        this.set_capVSIsMapped(capVSIsMapped);
        this
    }
    pub fn unpack(&self) -> cap_vspace_cap_Unpacked {
        cap_vspace_cap_Unpacked {
            capVSMappedASID: self.get_capVSMappedASID(),
            capVSBasePtr: self.get_capVSBasePtr(),
            capVSIsMapped: self.get_capVSIsMapped(),
        }
    }
    #[allow(dead_code)]
    pub fn get_capVSMappedASID(&self) -> u64 {
        self.0.get_bits(112usize..128usize)
    }
    pub fn set_capVSMappedASID(&mut self, capVSMappedASID: u64) {
        self.0.set_bits(112usize..128usize, capVSMappedASID)
    }
    #[allow(dead_code)]
    pub const fn width_of_capVSMappedASID() -> usize {
        128usize - 112usize
    }
    #[allow(dead_code)]
    pub fn get_capVSBasePtr(&self) -> u64 {
        self.0.get_bits(64usize..112usize)
    }
    pub fn set_capVSBasePtr(&mut self, capVSBasePtr: u64) {
        self.0.set_bits(64usize..112usize, capVSBasePtr)
    }
    #[allow(dead_code)]
    pub const fn width_of_capVSBasePtr() -> usize {
        112usize - 64usize
    }
    #[allow(dead_code)]
    fn get_capType(&self) -> u64 {
        self.0.get_bits(59usize..64usize)
    }
    fn set_capType(&mut self, capType: u64) {
        self.0.set_bits(59usize..64usize, capType)
    }
    #[allow(dead_code)]
    const fn width_of_capType() -> usize {
        64usize - 59usize
    }
    #[allow(dead_code)]
    pub fn get_capVSIsMapped(&self) -> u64 {
        self.0.get_bits(58usize..59usize)
    }
    pub fn set_capVSIsMapped(&mut self, capVSIsMapped: u64) {
        self.0.set_bits(58usize..59usize, capVSIsMapped)
    }
    #[allow(dead_code)]
    pub const fn width_of_capVSIsMapped() -> usize {
        59usize - 58usize
    }
}
impl fmt::Debug for cap_vspace_cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct cap_vspace_cap_Unpacked {
    pub capVSMappedASID: u64,
    pub capVSBasePtr: u64,
    pub capVSIsMapped: u64,
}
impl cap_vspace_cap_Unpacked {
    pub fn pack(self) -> cap_vspace_cap {
        match self {
            Self {
                capVSMappedASID,
                capVSBasePtr,
                capVSIsMapped,
            } => cap_vspace_cap::new(capVSMappedASID, capVSBasePtr, capVSIsMapped),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct cap_asid_control_cap(pub Bitfield<u64, 2usize>);
impl cap_asid_control_cap {
    pub fn new() -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_capType(cap_tag::cap_asid_control_cap);
        this
    }
    pub fn unpack(&self) -> cap_asid_control_cap_Unpacked {
        cap_asid_control_cap_Unpacked {}
    }
    #[allow(dead_code)]
    fn get_capType(&self) -> u64 {
        self.0.get_bits(59usize..64usize)
    }
    fn set_capType(&mut self, capType: u64) {
        self.0.set_bits(59usize..64usize, capType)
    }
    #[allow(dead_code)]
    const fn width_of_capType() -> usize {
        64usize - 59usize
    }
}
impl fmt::Debug for cap_asid_control_cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct cap_asid_control_cap_Unpacked {}
impl cap_asid_control_cap_Unpacked {
    pub fn pack(self) -> cap_asid_control_cap {
        match self {
            Self {} => cap_asid_control_cap::new(),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct cap_asid_pool_cap(pub Bitfield<u64, 2usize>);
impl cap_asid_pool_cap {
    pub fn new(capASIDBase: u64, capASIDPool: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_capType(cap_tag::cap_asid_pool_cap);
        this.set_capASIDBase(capASIDBase);
        this.set_capASIDPool(capASIDPool);
        this
    }
    pub fn unpack(&self) -> cap_asid_pool_cap_Unpacked {
        cap_asid_pool_cap_Unpacked {
            capASIDBase: self.get_capASIDBase(),
            capASIDPool: self.get_capASIDPool(),
        }
    }
    #[allow(dead_code)]
    fn get_capType(&self) -> u64 {
        self.0.get_bits(59usize..64usize)
    }
    fn set_capType(&mut self, capType: u64) {
        self.0.set_bits(59usize..64usize, capType)
    }
    #[allow(dead_code)]
    const fn width_of_capType() -> usize {
        64usize - 59usize
    }
    #[allow(dead_code)]
    pub fn get_capASIDBase(&self) -> u64 {
        self.0.get_bits(43usize..59usize)
    }
    pub fn set_capASIDBase(&mut self, capASIDBase: u64) {
        self.0.set_bits(43usize..59usize, capASIDBase)
    }
    #[allow(dead_code)]
    pub const fn width_of_capASIDBase() -> usize {
        59usize - 43usize
    }
    #[allow(dead_code)]
    pub fn get_capASIDPool(&self) -> u64 {
        self.0.get_bits(0usize..37usize)
    }
    pub fn set_capASIDPool(&mut self, capASIDPool: u64) {
        self.0.set_bits(0usize..37usize, capASIDPool)
    }
    #[allow(dead_code)]
    pub const fn width_of_capASIDPool() -> usize {
        37usize - 0usize
    }
}
impl fmt::Debug for cap_asid_pool_cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct cap_asid_pool_cap_Unpacked {
    pub capASIDBase: u64,
    pub capASIDPool: u64,
}
impl cap_asid_pool_cap_Unpacked {
    pub fn pack(self) -> cap_asid_pool_cap {
        match self {
            Self {
                capASIDBase,
                capASIDPool,
            } => cap_asid_pool_cap::new(capASIDBase, capASIDPool),
        }
    }
}
pub mod cap_tag {
    pub const cap_null_cap: u64 = 0;
    pub const cap_untyped_cap: u64 = 2;
    pub const cap_endpoint_cap: u64 = 4;
    pub const cap_notification_cap: u64 = 6;
    pub const cap_reply_cap: u64 = 8;
    pub const cap_cnode_cap: u64 = 10;
    pub const cap_thread_cap: u64 = 12;
    pub const cap_irq_control_cap: u64 = 14;
    pub const cap_irq_handler_cap: u64 = 16;
    pub const cap_zombie_cap: u64 = 18;
    pub const cap_domain_cap: u64 = 20;
    pub const cap_frame_cap: u64 = 1;
    pub const cap_page_table_cap: u64 = 3;
    pub const cap_vspace_cap: u64 = 9;
    pub const cap_asid_control_cap: u64 = 11;
    pub const cap_asid_pool_cap: u64 = 13;
}
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct cap(pub Bitfield<u64, 2usize>);
impl cap {
    pub fn splay(self) -> cap_Splayed {
        match self.get_tag() {
            cap_tag::cap_null_cap => cap_Splayed::null_cap(cap_null_cap(self.0)),
            cap_tag::cap_untyped_cap => cap_Splayed::untyped_cap(cap_untyped_cap(self.0)),
            cap_tag::cap_endpoint_cap => cap_Splayed::endpoint_cap(cap_endpoint_cap(self.0)),
            cap_tag::cap_notification_cap => {
                cap_Splayed::notification_cap(cap_notification_cap(self.0))
            }
            cap_tag::cap_reply_cap => cap_Splayed::reply_cap(cap_reply_cap(self.0)),
            cap_tag::cap_cnode_cap => cap_Splayed::cnode_cap(cap_cnode_cap(self.0)),
            cap_tag::cap_thread_cap => cap_Splayed::thread_cap(cap_thread_cap(self.0)),
            cap_tag::cap_irq_control_cap => {
                cap_Splayed::irq_control_cap(cap_irq_control_cap(self.0))
            }
            cap_tag::cap_irq_handler_cap => {
                cap_Splayed::irq_handler_cap(cap_irq_handler_cap(self.0))
            }
            cap_tag::cap_zombie_cap => cap_Splayed::zombie_cap(cap_zombie_cap(self.0)),
            cap_tag::cap_domain_cap => cap_Splayed::domain_cap(cap_domain_cap(self.0)),
            cap_tag::cap_frame_cap => cap_Splayed::frame_cap(cap_frame_cap(self.0)),
            cap_tag::cap_page_table_cap => cap_Splayed::page_table_cap(cap_page_table_cap(self.0)),
            cap_tag::cap_vspace_cap => cap_Splayed::vspace_cap(cap_vspace_cap(self.0)),
            cap_tag::cap_asid_control_cap => {
                cap_Splayed::asid_control_cap(cap_asid_control_cap(self.0))
            }
            cap_tag::cap_asid_pool_cap => cap_Splayed::asid_pool_cap(cap_asid_pool_cap(self.0)),
            _ => panic!(),
        }
    }
    pub fn get_tag(&self) -> u64 {
        self.0.get_bits(59usize..64usize)
    }
}
impl fmt::Debug for cap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.clone().splay().fmt(f)?;
        write!(f, ".unsplay()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum cap_Splayed {
    null_cap(cap_null_cap),
    untyped_cap(cap_untyped_cap),
    endpoint_cap(cap_endpoint_cap),
    notification_cap(cap_notification_cap),
    reply_cap(cap_reply_cap),
    cnode_cap(cap_cnode_cap),
    thread_cap(cap_thread_cap),
    irq_control_cap(cap_irq_control_cap),
    irq_handler_cap(cap_irq_handler_cap),
    zombie_cap(cap_zombie_cap),
    domain_cap(cap_domain_cap),
    frame_cap(cap_frame_cap),
    page_table_cap(cap_page_table_cap),
    vspace_cap(cap_vspace_cap),
    asid_control_cap(cap_asid_control_cap),
    asid_pool_cap(cap_asid_pool_cap),
}
impl cap_Splayed {
    pub fn unsplay(self) -> cap {
        match self {
            cap_Splayed::null_cap(cap_null_cap(bitfield)) => cap(bitfield),
            cap_Splayed::untyped_cap(cap_untyped_cap(bitfield)) => cap(bitfield),
            cap_Splayed::endpoint_cap(cap_endpoint_cap(bitfield)) => cap(bitfield),
            cap_Splayed::notification_cap(cap_notification_cap(bitfield)) => cap(bitfield),
            cap_Splayed::reply_cap(cap_reply_cap(bitfield)) => cap(bitfield),
            cap_Splayed::cnode_cap(cap_cnode_cap(bitfield)) => cap(bitfield),
            cap_Splayed::thread_cap(cap_thread_cap(bitfield)) => cap(bitfield),
            cap_Splayed::irq_control_cap(cap_irq_control_cap(bitfield)) => cap(bitfield),
            cap_Splayed::irq_handler_cap(cap_irq_handler_cap(bitfield)) => cap(bitfield),
            cap_Splayed::zombie_cap(cap_zombie_cap(bitfield)) => cap(bitfield),
            cap_Splayed::domain_cap(cap_domain_cap(bitfield)) => cap(bitfield),
            cap_Splayed::frame_cap(cap_frame_cap(bitfield)) => cap(bitfield),
            cap_Splayed::page_table_cap(cap_page_table_cap(bitfield)) => cap(bitfield),
            cap_Splayed::vspace_cap(cap_vspace_cap(bitfield)) => cap(bitfield),
            cap_Splayed::asid_control_cap(cap_asid_control_cap(bitfield)) => cap(bitfield),
            cap_Splayed::asid_pool_cap(cap_asid_pool_cap(bitfield)) => cap(bitfield),
        }
    }
}
impl cap_null_cap {
    pub fn unsplay(self) -> cap {
        cap(self.0)
    }
}
impl cap_null_cap_Unpacked {
    pub fn unsplay(self) -> cap {
        self.pack().unsplay()
    }
}
impl cap_untyped_cap {
    pub fn unsplay(self) -> cap {
        cap(self.0)
    }
}
impl cap_untyped_cap_Unpacked {
    pub fn unsplay(self) -> cap {
        self.pack().unsplay()
    }
}
impl cap_endpoint_cap {
    pub fn unsplay(self) -> cap {
        cap(self.0)
    }
}
impl cap_endpoint_cap_Unpacked {
    pub fn unsplay(self) -> cap {
        self.pack().unsplay()
    }
}
impl cap_notification_cap {
    pub fn unsplay(self) -> cap {
        cap(self.0)
    }
}
impl cap_notification_cap_Unpacked {
    pub fn unsplay(self) -> cap {
        self.pack().unsplay()
    }
}
impl cap_reply_cap {
    pub fn unsplay(self) -> cap {
        cap(self.0)
    }
}
impl cap_reply_cap_Unpacked {
    pub fn unsplay(self) -> cap {
        self.pack().unsplay()
    }
}
impl cap_cnode_cap {
    pub fn unsplay(self) -> cap {
        cap(self.0)
    }
}
impl cap_cnode_cap_Unpacked {
    pub fn unsplay(self) -> cap {
        self.pack().unsplay()
    }
}
impl cap_thread_cap {
    pub fn unsplay(self) -> cap {
        cap(self.0)
    }
}
impl cap_thread_cap_Unpacked {
    pub fn unsplay(self) -> cap {
        self.pack().unsplay()
    }
}
impl cap_irq_control_cap {
    pub fn unsplay(self) -> cap {
        cap(self.0)
    }
}
impl cap_irq_control_cap_Unpacked {
    pub fn unsplay(self) -> cap {
        self.pack().unsplay()
    }
}
impl cap_irq_handler_cap {
    pub fn unsplay(self) -> cap {
        cap(self.0)
    }
}
impl cap_irq_handler_cap_Unpacked {
    pub fn unsplay(self) -> cap {
        self.pack().unsplay()
    }
}
impl cap_zombie_cap {
    pub fn unsplay(self) -> cap {
        cap(self.0)
    }
}
impl cap_zombie_cap_Unpacked {
    pub fn unsplay(self) -> cap {
        self.pack().unsplay()
    }
}
impl cap_domain_cap {
    pub fn unsplay(self) -> cap {
        cap(self.0)
    }
}
impl cap_domain_cap_Unpacked {
    pub fn unsplay(self) -> cap {
        self.pack().unsplay()
    }
}
impl cap_frame_cap {
    pub fn unsplay(self) -> cap {
        cap(self.0)
    }
}
impl cap_frame_cap_Unpacked {
    pub fn unsplay(self) -> cap {
        self.pack().unsplay()
    }
}
impl cap_page_table_cap {
    pub fn unsplay(self) -> cap {
        cap(self.0)
    }
}
impl cap_page_table_cap_Unpacked {
    pub fn unsplay(self) -> cap {
        self.pack().unsplay()
    }
}
impl cap_vspace_cap {
    pub fn unsplay(self) -> cap {
        cap(self.0)
    }
}
impl cap_vspace_cap_Unpacked {
    pub fn unsplay(self) -> cap {
        self.pack().unsplay()
    }
}
impl cap_asid_control_cap {
    pub fn unsplay(self) -> cap {
        cap(self.0)
    }
}
impl cap_asid_control_cap_Unpacked {
    pub fn unsplay(self) -> cap {
        self.pack().unsplay()
    }
}
impl cap_asid_pool_cap {
    pub fn unsplay(self) -> cap {
        cap(self.0)
    }
}
impl cap_asid_pool_cap_Unpacked {
    pub fn unsplay(self) -> cap {
        self.pack().unsplay()
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct asid_map_asid_map_none(pub Bitfield<u64, 1usize>);
impl asid_map_asid_map_none {
    pub fn new() -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_types(asid_map_tag::asid_map_asid_map_none);
        this
    }
    pub fn unpack(&self) -> asid_map_asid_map_none_Unpacked {
        asid_map_asid_map_none_Unpacked {}
    }
    #[allow(dead_code)]
    fn get_types(&self) -> u64 {
        self.0.get_bits(0usize..1usize)
    }
    fn set_types(&mut self, types: u64) {
        self.0.set_bits(0usize..1usize, types)
    }
    #[allow(dead_code)]
    const fn width_of_types() -> usize {
        1usize - 0usize
    }
}
impl fmt::Debug for asid_map_asid_map_none {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct asid_map_asid_map_none_Unpacked {}
impl asid_map_asid_map_none_Unpacked {
    pub fn pack(self) -> asid_map_asid_map_none {
        match self {
            Self {} => asid_map_asid_map_none::new(),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct asid_map_asid_map_vspace(pub Bitfield<u64, 1usize>);
impl asid_map_asid_map_vspace {
    pub fn new(vspace_root: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_vspace_root(vspace_root);
        this.set_types(asid_map_tag::asid_map_asid_map_vspace);
        this
    }
    pub fn unpack(&self) -> asid_map_asid_map_vspace_Unpacked {
        asid_map_asid_map_vspace_Unpacked {
            vspace_root: self.get_vspace_root(),
        }
    }
    #[allow(dead_code)]
    pub fn get_vspace_root(&self) -> u64 {
        self.0.get_bits(12usize..48usize)
    }
    pub fn set_vspace_root(&mut self, vspace_root: u64) {
        self.0.set_bits(12usize..48usize, vspace_root)
    }
    #[allow(dead_code)]
    pub const fn width_of_vspace_root() -> usize {
        48usize - 12usize
    }
    #[allow(dead_code)]
    fn get_types(&self) -> u64 {
        self.0.get_bits(0usize..1usize)
    }
    fn set_types(&mut self, types: u64) {
        self.0.set_bits(0usize..1usize, types)
    }
    #[allow(dead_code)]
    const fn width_of_types() -> usize {
        1usize - 0usize
    }
}
impl fmt::Debug for asid_map_asid_map_vspace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct asid_map_asid_map_vspace_Unpacked {
    pub vspace_root: u64,
}
impl asid_map_asid_map_vspace_Unpacked {
    pub fn pack(self) -> asid_map_asid_map_vspace {
        match self {
            Self { vspace_root } => asid_map_asid_map_vspace::new(vspace_root),
        }
    }
}
pub mod asid_map_tag {
    pub const asid_map_asid_map_none: u64 = 0;
    pub const asid_map_asid_map_vspace: u64 = 1;
}
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct asid_map(pub Bitfield<u64, 1usize>);
impl asid_map {
    pub fn splay(self) -> asid_map_Splayed {
        match self.get_tag() {
            asid_map_tag::asid_map_asid_map_none => {
                asid_map_Splayed::asid_map_none(asid_map_asid_map_none(self.0))
            }
            asid_map_tag::asid_map_asid_map_vspace => {
                asid_map_Splayed::asid_map_vspace(asid_map_asid_map_vspace(self.0))
            }
            _ => panic!(),
        }
    }
    pub fn get_tag(&self) -> u64 {
        self.0.get_bits(0usize..1usize)
    }
}
impl fmt::Debug for asid_map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.clone().splay().fmt(f)?;
        write!(f, ".unsplay()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum asid_map_Splayed {
    asid_map_none(asid_map_asid_map_none),
    asid_map_vspace(asid_map_asid_map_vspace),
}
impl asid_map_Splayed {
    pub fn unsplay(self) -> asid_map {
        match self {
            asid_map_Splayed::asid_map_none(asid_map_asid_map_none(bitfield)) => asid_map(bitfield),
            asid_map_Splayed::asid_map_vspace(asid_map_asid_map_vspace(bitfield)) => {
                asid_map(bitfield)
            }
        }
    }
}
impl asid_map_asid_map_none {
    pub fn unsplay(self) -> asid_map {
        asid_map(self.0)
    }
}
impl asid_map_asid_map_none_Unpacked {
    pub fn unsplay(self) -> asid_map {
        self.pack().unsplay()
    }
}
impl asid_map_asid_map_vspace {
    pub fn unsplay(self) -> asid_map {
        asid_map(self.0)
    }
}
impl asid_map_asid_map_vspace_Unpacked {
    pub fn unsplay(self) -> asid_map {
        self.pack().unsplay()
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct seL4_Fault_NullFault(pub Bitfield<u64, 2usize>);
impl seL4_Fault_NullFault {
    pub fn new() -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_seL4_FaultType(seL4_Fault_tag::seL4_Fault_NullFault);
        this
    }
    pub fn unpack(&self) -> seL4_Fault_NullFault_Unpacked {
        seL4_Fault_NullFault_Unpacked {}
    }
    #[allow(dead_code)]
    fn get_seL4_FaultType(&self) -> u64 {
        self.0.get_bits(0usize..4usize)
    }
    fn set_seL4_FaultType(&mut self, seL4_FaultType: u64) {
        self.0.set_bits(0usize..4usize, seL4_FaultType)
    }
    #[allow(dead_code)]
    const fn width_of_seL4_FaultType() -> usize {
        4usize - 0usize
    }
}
impl fmt::Debug for seL4_Fault_NullFault {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct seL4_Fault_NullFault_Unpacked {}
impl seL4_Fault_NullFault_Unpacked {
    pub fn pack(self) -> seL4_Fault_NullFault {
        match self {
            Self {} => seL4_Fault_NullFault::new(),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct seL4_Fault_CapFault(pub Bitfield<u64, 2usize>);
impl seL4_Fault_CapFault {
    pub fn new(address: u64, inReceivePhase: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_address(address);
        this.set_inReceivePhase(inReceivePhase);
        this.set_seL4_FaultType(seL4_Fault_tag::seL4_Fault_CapFault);
        this
    }
    pub fn unpack(&self) -> seL4_Fault_CapFault_Unpacked {
        seL4_Fault_CapFault_Unpacked {
            address: self.get_address(),
            inReceivePhase: self.get_inReceivePhase(),
        }
    }
    #[allow(dead_code)]
    pub fn get_address(&self) -> u64 {
        self.0.get_bits(64usize..128usize)
    }
    pub fn set_address(&mut self, address: u64) {
        self.0.set_bits(64usize..128usize, address)
    }
    #[allow(dead_code)]
    pub const fn width_of_address() -> usize {
        128usize - 64usize
    }
    #[allow(dead_code)]
    pub fn get_inReceivePhase(&self) -> u64 {
        self.0.get_bits(63usize..64usize)
    }
    pub fn set_inReceivePhase(&mut self, inReceivePhase: u64) {
        self.0.set_bits(63usize..64usize, inReceivePhase)
    }
    #[allow(dead_code)]
    pub const fn width_of_inReceivePhase() -> usize {
        64usize - 63usize
    }
    #[allow(dead_code)]
    fn get_seL4_FaultType(&self) -> u64 {
        self.0.get_bits(0usize..4usize)
    }
    fn set_seL4_FaultType(&mut self, seL4_FaultType: u64) {
        self.0.set_bits(0usize..4usize, seL4_FaultType)
    }
    #[allow(dead_code)]
    const fn width_of_seL4_FaultType() -> usize {
        4usize - 0usize
    }
}
impl fmt::Debug for seL4_Fault_CapFault {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct seL4_Fault_CapFault_Unpacked {
    pub address: u64,
    pub inReceivePhase: u64,
}
impl seL4_Fault_CapFault_Unpacked {
    pub fn pack(self) -> seL4_Fault_CapFault {
        match self {
            Self {
                address,
                inReceivePhase,
            } => seL4_Fault_CapFault::new(address, inReceivePhase),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct seL4_Fault_UnknownSyscall(pub Bitfield<u64, 2usize>);
impl seL4_Fault_UnknownSyscall {
    pub fn new(syscallNumber: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_syscallNumber(syscallNumber);
        this.set_seL4_FaultType(seL4_Fault_tag::seL4_Fault_UnknownSyscall);
        this
    }
    pub fn unpack(&self) -> seL4_Fault_UnknownSyscall_Unpacked {
        seL4_Fault_UnknownSyscall_Unpacked {
            syscallNumber: self.get_syscallNumber(),
        }
    }
    #[allow(dead_code)]
    pub fn get_syscallNumber(&self) -> u64 {
        self.0.get_bits(64usize..128usize)
    }
    pub fn set_syscallNumber(&mut self, syscallNumber: u64) {
        self.0.set_bits(64usize..128usize, syscallNumber)
    }
    #[allow(dead_code)]
    pub const fn width_of_syscallNumber() -> usize {
        128usize - 64usize
    }
    #[allow(dead_code)]
    fn get_seL4_FaultType(&self) -> u64 {
        self.0.get_bits(0usize..4usize)
    }
    fn set_seL4_FaultType(&mut self, seL4_FaultType: u64) {
        self.0.set_bits(0usize..4usize, seL4_FaultType)
    }
    #[allow(dead_code)]
    const fn width_of_seL4_FaultType() -> usize {
        4usize - 0usize
    }
}
impl fmt::Debug for seL4_Fault_UnknownSyscall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct seL4_Fault_UnknownSyscall_Unpacked {
    pub syscallNumber: u64,
}
impl seL4_Fault_UnknownSyscall_Unpacked {
    pub fn pack(self) -> seL4_Fault_UnknownSyscall {
        match self {
            Self { syscallNumber } => seL4_Fault_UnknownSyscall::new(syscallNumber),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct seL4_Fault_UserException(pub Bitfield<u64, 2usize>);
impl seL4_Fault_UserException {
    pub fn new(number: u64, code: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_number(number);
        this.set_code(code);
        this.set_seL4_FaultType(seL4_Fault_tag::seL4_Fault_UserException);
        this
    }
    pub fn unpack(&self) -> seL4_Fault_UserException_Unpacked {
        seL4_Fault_UserException_Unpacked {
            number: self.get_number(),
            code: self.get_code(),
        }
    }
    #[allow(dead_code)]
    pub fn get_number(&self) -> u64 {
        self.0.get_bits(32usize..64usize)
    }
    pub fn set_number(&mut self, number: u64) {
        self.0.set_bits(32usize..64usize, number)
    }
    #[allow(dead_code)]
    pub const fn width_of_number() -> usize {
        64usize - 32usize
    }
    #[allow(dead_code)]
    pub fn get_code(&self) -> u64 {
        self.0.get_bits(4usize..32usize)
    }
    pub fn set_code(&mut self, code: u64) {
        self.0.set_bits(4usize..32usize, code)
    }
    #[allow(dead_code)]
    pub const fn width_of_code() -> usize {
        32usize - 4usize
    }
    #[allow(dead_code)]
    fn get_seL4_FaultType(&self) -> u64 {
        self.0.get_bits(0usize..4usize)
    }
    fn set_seL4_FaultType(&mut self, seL4_FaultType: u64) {
        self.0.set_bits(0usize..4usize, seL4_FaultType)
    }
    #[allow(dead_code)]
    const fn width_of_seL4_FaultType() -> usize {
        4usize - 0usize
    }
}
impl fmt::Debug for seL4_Fault_UserException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct seL4_Fault_UserException_Unpacked {
    pub number: u64,
    pub code: u64,
}
impl seL4_Fault_UserException_Unpacked {
    pub fn pack(self) -> seL4_Fault_UserException {
        match self {
            Self { number, code } => seL4_Fault_UserException::new(number, code),
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq)]
pub struct seL4_Fault_VMFault(pub Bitfield<u64, 2usize>);
impl seL4_Fault_VMFault {
    pub fn new(address: u64, FSR: u64, instructionFault: u64) -> Self {
        let mut this = Self(Bitfield::zeroed());
        this.set_address(address);
        this.set_FSR(FSR);
        this.set_instructionFault(instructionFault);
        this.set_seL4_FaultType(seL4_Fault_tag::seL4_Fault_VMFault);
        this
    }
    pub fn unpack(&self) -> seL4_Fault_VMFault_Unpacked {
        seL4_Fault_VMFault_Unpacked {
            address: self.get_address(),
            FSR: self.get_FSR(),
            instructionFault: self.get_instructionFault(),
        }
    }
    #[allow(dead_code)]
    pub fn get_address(&self) -> u64 {
        self.0.get_bits(64usize..128usize)
    }
    pub fn set_address(&mut self, address: u64) {
        self.0.set_bits(64usize..128usize, address)
    }
    #[allow(dead_code)]
    pub const fn width_of_address() -> usize {
        128usize - 64usize
    }
    #[allow(dead_code)]
    pub fn get_FSR(&self) -> u64 {
        self.0.get_bits(32usize..64usize)
    }
    pub fn set_FSR(&mut self, FSR: u64) {
        self.0.set_bits(32usize..64usize, FSR)
    }
    #[allow(dead_code)]
    pub const fn width_of_FSR() -> usize {
        64usize - 32usize
    }
    #[allow(dead_code)]
    pub fn get_instructionFault(&self) -> u64 {
        self.0.get_bits(31usize..32usize)
    }
    pub fn set_instructionFault(&mut self, instructionFault: u64) {
        self.0.set_bits(31usize..32usize, instructionFault)
    }
    #[allow(dead_code)]
    pub const fn width_of_instructionFault() -> usize {
        32usize - 31usize
    }
    #[allow(dead_code)]
    fn get_seL4_FaultType(&self) -> u64 {
        self.0.get_bits(0usize..4usize)
    }
    fn set_seL4_FaultType(&mut self, seL4_FaultType: u64) {
        self.0.set_bits(0usize..4usize, seL4_FaultType)
    }
    #[allow(dead_code)]
    const fn width_of_seL4_FaultType() -> usize {
        4usize - 0usize
    }
}
impl fmt::Debug for seL4_Fault_VMFault {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)?;
        write!(f, ".pack()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct seL4_Fault_VMFault_Unpacked {
    pub address: u64,
    pub FSR: u64,
    pub instructionFault: u64,
}
impl seL4_Fault_VMFault_Unpacked {
    pub fn pack(self) -> seL4_Fault_VMFault {
        match self {
            Self {
                address,
                FSR,
                instructionFault,
            } => seL4_Fault_VMFault::new(address, FSR, instructionFault),
        }
    }
}
pub mod seL4_Fault_tag {
    pub const seL4_Fault_NullFault: u64 = 0;
    pub const seL4_Fault_CapFault: u64 = 1;
    pub const seL4_Fault_UnknownSyscall: u64 = 2;
    pub const seL4_Fault_UserException: u64 = 3;
    pub const seL4_Fault_VMFault: u64 = 5;
}
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct seL4_Fault(pub Bitfield<u64, 2usize>);
impl seL4_Fault {
    pub fn splay(self) -> seL4_Fault_Splayed {
        match self.get_tag() {
            seL4_Fault_tag::seL4_Fault_NullFault => {
                seL4_Fault_Splayed::NullFault(seL4_Fault_NullFault(self.0))
            }
            seL4_Fault_tag::seL4_Fault_CapFault => {
                seL4_Fault_Splayed::CapFault(seL4_Fault_CapFault(self.0))
            }
            seL4_Fault_tag::seL4_Fault_UnknownSyscall => {
                seL4_Fault_Splayed::UnknownSyscall(seL4_Fault_UnknownSyscall(self.0))
            }
            seL4_Fault_tag::seL4_Fault_UserException => {
                seL4_Fault_Splayed::UserException(seL4_Fault_UserException(self.0))
            }
            seL4_Fault_tag::seL4_Fault_VMFault => {
                seL4_Fault_Splayed::VMFault(seL4_Fault_VMFault(self.0))
            }
            _ => panic!(),
        }
    }
    pub fn get_tag(&self) -> u64 {
        self.0.get_bits(0usize..4usize)
    }
}
impl fmt::Debug for seL4_Fault {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.clone().splay().fmt(f)?;
        write!(f, ".unsplay()")?;
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum seL4_Fault_Splayed {
    NullFault(seL4_Fault_NullFault),
    CapFault(seL4_Fault_CapFault),
    UnknownSyscall(seL4_Fault_UnknownSyscall),
    UserException(seL4_Fault_UserException),
    VMFault(seL4_Fault_VMFault),
}
impl seL4_Fault_Splayed {
    pub fn unsplay(self) -> seL4_Fault {
        match self {
            seL4_Fault_Splayed::NullFault(seL4_Fault_NullFault(bitfield)) => seL4_Fault(bitfield),
            seL4_Fault_Splayed::CapFault(seL4_Fault_CapFault(bitfield)) => seL4_Fault(bitfield),
            seL4_Fault_Splayed::UnknownSyscall(seL4_Fault_UnknownSyscall(bitfield)) => {
                seL4_Fault(bitfield)
            }
            seL4_Fault_Splayed::UserException(seL4_Fault_UserException(bitfield)) => {
                seL4_Fault(bitfield)
            }
            seL4_Fault_Splayed::VMFault(seL4_Fault_VMFault(bitfield)) => seL4_Fault(bitfield),
        }
    }
}
impl seL4_Fault_NullFault {
    pub fn unsplay(self) -> seL4_Fault {
        seL4_Fault(self.0)
    }
}
impl seL4_Fault_NullFault_Unpacked {
    pub fn unsplay(self) -> seL4_Fault {
        self.pack().unsplay()
    }
}
impl seL4_Fault_CapFault {
    pub fn unsplay(self) -> seL4_Fault {
        seL4_Fault(self.0)
    }
}
impl seL4_Fault_CapFault_Unpacked {
    pub fn unsplay(self) -> seL4_Fault {
        self.pack().unsplay()
    }
}
impl seL4_Fault_UnknownSyscall {
    pub fn unsplay(self) -> seL4_Fault {
        seL4_Fault(self.0)
    }
}
impl seL4_Fault_UnknownSyscall_Unpacked {
    pub fn unsplay(self) -> seL4_Fault {
        self.pack().unsplay()
    }
}
impl seL4_Fault_UserException {
    pub fn unsplay(self) -> seL4_Fault {
        seL4_Fault(self.0)
    }
}
impl seL4_Fault_UserException_Unpacked {
    pub fn unsplay(self) -> seL4_Fault {
        self.pack().unsplay()
    }
}
impl seL4_Fault_VMFault {
    pub fn unsplay(self) -> seL4_Fault {
        seL4_Fault(self.0)
    }
}
impl seL4_Fault_VMFault_Unpacked {
    pub fn unsplay(self) -> seL4_Fault {
        self.pack().unsplay()
    }
}
