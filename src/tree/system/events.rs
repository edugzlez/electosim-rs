#[allow(dead_code)]
pub enum ElectionEvent {
    Create,
    
    IncreaseVotes {
        region: u32,
        candidacy: u32,
        votes: i32,
    },
    
    AfterSetParent {
        region: u32,
        parent: u32,
        last_parent: Option<u32>,
    },
    
    BeforeSetParent {
        region: u32,
        parent: u32,
    },
    
    AfterRemoveParent(u32),
    
    BeforeRemoveParent(u32),
    
    CandidacyRemoved(u32),
    
    RegionCreated(u32),
    
    RegionModified(u32),
    
    RegionRemoved { region: u32, remove_children: bool },
}
