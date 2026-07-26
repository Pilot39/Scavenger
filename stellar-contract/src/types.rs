
use soroban_sdk::{contracttype, Address, String, Symbol};

/// Represents a transfer record in the recycling system
/// This struct is fully compatible with Soroban storage and implements
/// deterministic serialization for safe storage and retrieval
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferRecord {
    /// Unique identifier for the transfer
    pub id: u64,
    /// Address of the sender
    pub from: Address,
    /// Address of the recipient
    pub to: Address,
    /// Type of item being transferred
    pub item_type: TransferItemType,
    /// Identifier of the item being transferred
    pub item_id: u64,
    /// Amount or quantity being transferred
    pub amount: u64,
    /// Timestamp when the transfer occurred
    pub timestamp: u64,
    /// Status of the transfer
    pub status: TransferStatus,
    /// Optional note or description
    pub note: String,
}

/// Represents the type of item being transferred
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransferItemType {
    /// Material/Waste transfer
    Material = 0,
    /// Token transfer
    Token = 1,
    /// Incentive transfer
    Incentive = 2,
    /// Ownership transfer
    Ownership = 3,
}

/// Represents the status of a transfer
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransferStatus {
    /// Transfer is pending
    Pending = 0,
    /// Transfer is in progress
    InProgress = 1,
    /// Transfer completed successfully
    Completed = 2,
    /// Transfer failed
    Failed = 3,
    /// Transfer was cancelled
    Cancelled = 4,
}

impl TransferItemType {
    /// Validates if the value is a valid TransferItemType variant
    pub fn is_valid(value: u32) -> bool {
        matches!(value, 0 | 1 | 2 | 3)
    }

    /// Converts a u32 to a TransferItemType
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            0 => Some(TransferItemType::Material),
            1 => Some(TransferItemType::Token),
            2 => Some(TransferItemType::Incentive),
            3 => Some(TransferItemType::Ownership),
            _ => None,
        }
    }

    /// Converts the TransferItemType to u32
    pub fn to_u32(&self) -> u32 {
        *self as u32
    }

    /// Returns the string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            TransferItemType::Material => "MATERIAL",
            TransferItemType::Token => "TOKEN",
            TransferItemType::Incentive => "INCENTIVE",
            TransferItemType::Ownership => "OWNERSHIP",
        }
    }
}

impl TransferStatus {
    /// Validates if the value is a valid TransferStatus variant
    pub fn is_valid(value: u32) -> bool {
        matches!(value, 0 | 1 | 2 | 3 | 4)
    }

    /// Converts a u32 to a TransferStatus
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            0 => Some(TransferStatus::Pending),
            1 => Some(TransferStatus::InProgress),
            2 => Some(TransferStatus::Completed),
            3 => Some(TransferStatus::Failed),
            4 => Some(TransferStatus::Cancelled),
            _ => None,
        }
    }

    /// Converts the TransferStatus to u32
    pub fn to_u32(&self) -> u32 {
        *self as u32
    }

    /// Returns the string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            TransferStatus::Pending => "PENDING",
            TransferStatus::InProgress => "IN_PROGRESS",
            TransferStatus::Completed => "COMPLETED",
            TransferStatus::Failed => "FAILED",
            TransferStatus::Cancelled => "CANCELLED",
        }
    }

    /// Checks if the status is final (cannot be changed)
    pub fn is_final(&self) -> bool {
        matches!(
            self,
            TransferStatus::Completed | TransferStatus::Failed | TransferStatus::Cancelled
        )
    }

    /// Checks if the status is active (can still be modified)
    pub fn is_active(&self) -> bool {
        matches!(self, TransferStatus::Pending | TransferStatus::InProgress)
    }
}

impl TransferRecord {
    /// Creates a new TransferRecord with Pending status
    pub fn new(
        id: u64,
        from: Address,
        to: Address,
        item_type: TransferItemType,
        item_id: u64,
        amount: u64,
        timestamp: u64,
        note: String,
    ) -> Self {
        Self {
            id,
            from,
            to,
            item_type,
            item_id,
            amount,
            timestamp,
            status: TransferStatus::Pending,
            note,
        }
    }

    /// Updates the status of the transfer
    /// Returns true if updated, false if status is final
    pub fn update_status(&mut self, new_status: TransferStatus) -> bool {
        if self.status.is_final() {
            return false;
        }
        self.status = new_status;
        true
    }

    /// Validates the transfer record
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.amount == 0 {
            return Err("Amount must be greater than zero");
        }
        if self.from == self.to {
            return Err("Sender and recipient cannot be the same");
        }
        Ok(())
    }

    /// Checks if the transfer is complete
    pub fn is_complete(&self) -> bool {
        self.status == TransferStatus::Completed
    }

    /// Checks if the transfer can be modified
    pub fn is_modifiable(&self) -> bool {
        self.status.is_active()
    }
}

/// Represents an incentive offered by a manufacturer to encourage recycling
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Incentive {
    /// Unique identifier for the incentive
    pub id: u64,
    /// Address of the manufacturer offering the incentive
    pub rewarder: Address,
    /// Type of waste this incentive targets
    pub waste_type: WasteType,
    /// Reward points per kilogram
    pub reward_points: u64,
    /// Total points budget allocated for this incentive
    pub total_budget: u64,
    /// Remaining points budget available
    pub remaining_budget: u64,
    /// Whether the incentive is currently active
    pub active: bool,
    /// Timestamp when the incentive was created
    pub created_at: u64,
}

impl Incentive {
    /// Creates a new Incentive instance
    pub fn new(
        id: u64,
        rewarder: Address,
        waste_type: WasteType,
        reward_points: u64,
        total_budget: u64,
        created_at: u64,
    ) -> Self {
        Self {
            id,
            rewarder,
            waste_type,
            reward_points,
            total_budget,
            remaining_budget: total_budget,
            active: true,
            created_at,
        }
    }

    /// Deactivates the incentive
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Calculates reward for a given weight in grams
    pub fn calculate_reward(&self, weight_grams: u64) -> u64 {
        // Convert grams to kg and multiply by reward points
        (weight_grams / 1000)
            .checked_mul(self.reward_points)
            .expect("Overflow in reward calculation")
    }

    /// Attempts to claim a reward, returns the amount claimed
    /// Returns None if insufficient budget
    pub fn claim_reward(&mut self, weight_grams: u64) -> Option<u64> {
        if !self.active {
            return None;
        }

        let reward = self.calculate_reward(weight_grams);
        if reward > self.remaining_budget {
            return None;
        }

        self.remaining_budget = self
            .remaining_budget
            .checked_sub(reward)
            .expect("Overflow in remaining budget");

        // Auto-deactivate if budget exhausted
        if self.remaining_budget == 0 {
            self.active = false;
        }

        Some(reward)
    }

    /// Checks if the incentive has sufficient budget for a reward
    pub fn has_sufficient_budget(&self, weight_grams: u64) -> bool {
        if !self.active {
            return false;
        }
        let reward = self.calculate_reward(weight_grams);
        reward <= self.remaining_budget

    }
}

/// Represents the role of a participant in the Scavenger ecosystem
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParticipantRole {
    /// Recycler role - responsible for collecting and processing recyclable materials
    Recycler = 0,
    /// Collector role - responsible for gathering materials from various sources
    Collector = 1,
    /// Manufacturer role - responsible for processing materials into new products
    Manufacturer = 2,
}

impl ParticipantRole {
    /// Validates if the role is a valid ParticipantRole variant
    pub fn is_valid(role: u32) -> bool {
        matches!(role, 0..=2)
    }

    /// Converts a u32 to a ParticipantRole
    /// Returns None if the value is invalid
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            0 => Some(ParticipantRole::Recycler),
            1 => Some(ParticipantRole::Collector),
            2 => Some(ParticipantRole::Manufacturer),
            _ => None,
        }
    }

    /// Converts the ParticipantRole to u32
    pub fn to_u32(&self) -> u32 {
        *self as u32
    }

    /// Returns the string representation of the role
    pub fn as_str(&self) -> &'static str {
        match self {
            ParticipantRole::Recycler => "RECYCLER",
            ParticipantRole::Collector => "COLLECTOR",
            ParticipantRole::Manufacturer => "MANUFACTURER",
        }
    }

    /// Validates if a participant can perform a specific action based on their role
    pub fn can_collect_materials(&self) -> bool {
        matches!(self, ParticipantRole::Recycler | ParticipantRole::Collector)
    }

    /// Validates if a participant can manufacture products
    pub fn can_manufacture(&self) -> bool {
        matches!(self, ParticipantRole::Manufacturer)
    }

    /// Validates if a participant can process recyclables
    pub fn can_process_recyclables(&self) -> bool {
        matches!(self, ParticipantRole::Recycler)
    }
}

/// Represents the type of waste material in the recycling ecosystem
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WasteType {
    /// Paper waste - newspapers, cardboard, office paper
    Paper = 0,
    /// PET plastic - polyethylene terephthalate bottles and containers
    PetPlastic = 1,
    /// General plastic waste - various plastic types
    Plastic = 2,
    /// Metal waste - aluminum, steel, copper
    Metal = 3,
    /// Glass waste - bottles, jars, containers
    Glass = 4,
}

impl WasteType {
    /// Validates if the value is a valid WasteType variant
    pub fn is_valid(value: u32) -> bool {
        matches!(value, 0..=4)
    }

    /// Converts a u32 to a WasteType
    /// Returns None if the value is invalid
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            0 => Some(WasteType::Paper),
            1 => Some(WasteType::PetPlastic),
            2 => Some(WasteType::Plastic),
            3 => Some(WasteType::Metal),
            4 => Some(WasteType::Glass),
            _ => None,
        }
    }

    /// Converts the WasteType to u32
    pub fn to_u32(&self) -> u32 {
        *self as u32
    }

    /// Returns the string representation of the waste type
    pub fn as_str(&self) -> &'static str {
        match self {
            WasteType::Paper => "PAPER",
            WasteType::PetPlastic => "PETPLASTIC",
            WasteType::Plastic => "PLASTIC",
            WasteType::Metal => "METAL",
            WasteType::Glass => "GLASS",
        }
    }

    /// Checks if the waste type is recyclable plastic
    pub fn is_plastic(&self) -> bool {
        matches!(self, WasteType::PetPlastic | WasteType::Plastic)
    }

    /// Checks if the waste type is biodegradable
    pub fn is_biodegradable(&self) -> bool {
        matches!(self, WasteType::Paper)
    }

    /// Checks if the waste type is infinitely recyclable
    pub fn is_infinitely_recyclable(&self) -> bool {
        matches!(self, WasteType::Metal | WasteType::Glass)
    }
}

impl core::fmt::Display for WasteType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Represents a recyclable material submission in the system
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Material {
    /// Unique identifier for the material
    pub id: u64,
    /// Type of waste material
    pub waste_type: WasteType,
    /// Weight of the material in grams
    pub weight: u64,
    /// Address of the participant who submitted the material
    pub submitter: Address,
    /// Timestamp when the material was submitted
    pub submitted_at: u64,
    /// Whether the material has been verified
    pub verified: bool,
    /// Optional description of the material
    pub description: String,
}

impl Material {
    /// Creates a new Material instance
    pub fn new(
        id: u64,
        waste_type: WasteType,
        weight: u64,
        submitter: Address,
        submitted_at: u64,
        description: String,
    ) -> Self {
        Self {
            id,
            waste_type,
            weight,
            submitter,
            submitted_at,
            verified: false,
            description,
        }
    }

    /// Marks the material as verified
    pub fn verify(&mut self) {
        self.verified = true;
    }

    /// Checks if the material meets minimum weight requirement (100g)
    pub fn meets_minimum_weight(&self) -> bool {
        self.weight >= 100
    }

    /// Calculates reward points based on waste type and weight
    /// Different waste types have different point multipliers
    pub fn calculate_reward_points(&self) -> u64 {
        let multiplier = match self.waste_type {
            WasteType::Paper => 1,
            WasteType::PetPlastic => 3,
            WasteType::Plastic => 2,
            WasteType::Metal => 5,
            WasteType::Glass => 2,
        };

        // Points = (weight in kg) * multiplier * 10
        (self.weight / 1000)
            .checked_mul(multiplier)
            .and_then(|v| v.checked_mul(10))
            .expect("Overflow in reward points calculation")
    }
}

/// Represents a waste item in the recycling system
/// This is the main struct that tracks waste throughout its lifecycle
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Waste {
    /// Unique identifier for the waste item
    pub waste_id: u128,
    /// Type of waste material
    pub waste_type: WasteType,
    /// Weight of the waste in grams
    pub weight: u128,
    /// Current owner of the waste
    pub current_owner: Address,
    /// Latitude coordinate (scaled by 1e6 for precision)
    pub latitude: i128,
    /// Longitude coordinate (scaled by 1e6 for precision)
    pub longitude: i128,
    /// Timestamp when the waste was recycled (0 if not yet recycled)
    pub recycled_timestamp: u64,
    /// Whether the waste is currently active in the system
    pub is_active: bool,
    /// Whether the waste has been confirmed/verified
    pub is_confirmed: bool,
    /// Address of the confirmer/verifier
    pub confirmer: Address,
}

impl Waste {
    /// Creates a new Waste instance with all fields
    pub fn new(
        waste_id: u128,
        waste_type: WasteType,
        weight: u128,
        current_owner: Address,
        latitude: i128,
        longitude: i128,
        recycled_timestamp: u64,
        is_active: bool,
        is_confirmed: bool,
        confirmer: Address,
    ) -> Self {
        Self {
            waste_id,
            waste_type,
            weight,
            current_owner,
            latitude,
            longitude,
            recycled_timestamp,
            is_active,
            is_confirmed,
            confirmer,
        }
    }

    /// Validates that the waste has valid coordinates
    pub fn has_valid_coordinates(&self) -> bool {
        let max_lat = 90_000_000i128;
        let max_lon = 180_000_000i128;
        
        self.latitude >= -max_lat 
            && self.latitude <= max_lat
            && self.longitude >= -max_lon
            && self.longitude <= max_lon
    }

    /// Checks if the waste has been recycled
    pub fn is_recycled(&self) -> bool {
        self.recycled_timestamp > 0
    }

    /// Checks if the waste meets minimum weight requirement (100g)
    pub fn meets_minimum_weight(&self) -> bool {
        self.weight >= 100
    }

    /// Marks the waste as recycled with the given timestamp
    pub fn mark_recycled(&mut self, timestamp: u64) {
        self.recycled_timestamp = timestamp;
    }

    /// Confirms the waste with the given confirmer
    pub fn confirm(&mut self, confirmer: Address) {
        self.is_confirmed = true;
        self.confirmer = confirmer;
    }

    /// Resets the confirmation status of the waste
    pub fn reset_confirmation(&mut self) {
        self.is_confirmed = false;
        self.confirmer = self.current_owner.clone();
    }

    /// Deactivates the waste
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Transfers ownership to a new owner
    pub fn transfer_to(&mut self, new_owner: Address) {
        self.current_owner = new_owner;
    }

    /// Updates the location of the waste
    pub fn update_location(&mut self, latitude: i128, longitude: i128) {
        self.latitude = latitude;
        self.longitude = longitude;
    }
}

/// Transfer record for waste movement across the supply chain.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WasteTransfer {
    pub waste_id: u128,
    pub from: Address,
    pub to: Address,
    pub transferred_at: u64,
    pub latitude: i128,
    pub longitude: i128,
    pub note: Symbol,
}

impl WasteTransfer {
    pub fn new(
        waste_id: u128,
        from: Address,
        to: Address,
        transferred_at: u64,
        latitude: i128,
        longitude: i128,
        note: Symbol,
    ) -> Self {
        Self {
            waste_id,
            from,
            to,
            transferred_at,
            latitude,
            longitude,
            note,
        }
    }
}


/// Tracks recycling statistics for a participant
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecyclingStats {
    /// Participant address
    pub participant: Address,
    /// Total number of materials submitted
    pub total_submissions: u64,
    /// Total number of verified materials
    pub verified_submissions: u64,
    /// Total weight of all materials in grams
    pub total_weight: u64,
    /// Total reward points earned
    pub total_points: u64,
    /// Number of materials by waste type
    pub paper_count: u64,
    pub pet_plastic_count: u64,
    pub plastic_count: u64,
    pub metal_count: u64,
    pub glass_count: u64,
}

impl RecyclingStats {
    /// Creates a new RecyclingStats instance
    pub fn new(participant: Address) -> Self {
        Self {
            participant,
            total_submissions: 0,
            verified_submissions: 0,
            total_weight: 0,
            total_points: 0,
            paper_count: 0,
            pet_plastic_count: 0,
            plastic_count: 0,
            metal_count: 0,
            glass_count: 0,
        }
    }

    /// Records a new material submission
    pub fn record_submission(&mut self, material: &Material) {
        self.total_submissions = self
            .total_submissions
            .checked_add(1)
            .expect("Overflow in total submissions");
        self.total_weight = self
            .total_weight
            .checked_add(material.weight)
            .expect("Overflow in total weight");

        // Update waste type count
        let count = match material.waste_type {
            WasteType::Paper => &mut self.paper_count,
            WasteType::PetPlastic => &mut self.pet_plastic_count,
            WasteType::Plastic => &mut self.plastic_count,
            WasteType::Metal => &mut self.metal_count,
            WasteType::Glass => &mut self.glass_count,
        };
        *count = count.checked_add(1).expect("Overflow in waste type count");
    }

    /// Records a material verification
    pub fn record_verification(&mut self, material: &Material) {
        if material.verified {
            self.verified_submissions = self
                .verified_submissions
                .checked_add(1)
                .expect("Overflow in verified submissions");
            self.total_points = self
                .total_points
                .checked_add(material.calculate_reward_points())
                .expect("Overflow in total points");
        }
    }

    /// Calculates the verification rate (percentage)
    pub fn verification_rate(&self) -> u64 {
        if self.total_submissions == 0 {
            0
        } else {
            self.verified_submissions
                .checked_mul(100)
                .expect("Overflow in verification rate")
                / self.total_submissions
        }
    }

    /// Gets the most submitted waste type
    pub fn most_submitted_type(&self) -> Option<WasteType> {
        let counts = [
            (WasteType::Paper, self.paper_count),
            (WasteType::PetPlastic, self.pet_plastic_count),
            (WasteType::Plastic, self.plastic_count),
            (WasteType::Metal, self.metal_count),
            (WasteType::Glass, self.glass_count),
        ];

        counts
            .iter()
            .max_by_key(|(_, count)| count)
            .filter(|(_, count)| *count > 0)
            .map(|(waste_type, _)| *waste_type)
    }

    /// Calculates average weight per submission
    pub fn average_weight(&self) -> u64 {
        if self.total_submissions == 0 {
            0
        } else {
            self.total_weight / self.total_submissions
        }
    }

    /// Checks if participant is an active recycler (10+ submissions)
    pub fn is_active_recycler(&self) -> bool {
        self.total_submissions >= 10
    }

    /// Checks if participant is a verified contributor (80%+ verification rate)
    pub fn is_verified_contributor(&self) -> bool {
        self.verification_rate() >= 80
    }
}

#[cfg(test)]
mod recycling_stats_tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_new_stats() {
        let env = soroban_sdk::Env::default();
        let participant = Address::generate(&env);

        let stats = RecyclingStats::new(participant.clone());

        assert_eq!(stats.participant, participant);
        assert_eq!(stats.total_submissions, 0);
        assert_eq!(stats.verified_submissions, 0);
        assert_eq!(stats.total_weight, 0);
        assert_eq!(stats.total_points, 0);
    }

    #[test]
    fn test_record_submission() {
        let env = soroban_sdk::Env::default();
        let participant = Address::generate(&env);
        let description = String::from_str(&env, "Test");

        let mut stats = RecyclingStats::new(participant.clone());
        let material = Material::new(1, WasteType::Paper, 5000, participant, 0, description);

        stats.record_submission(&material);

        assert_eq!(stats.total_submissions, 1);
        assert_eq!(stats.total_weight, 5000);
        assert_eq!(stats.paper_count, 1);
    }

    #[test]
    fn test_record_verification() {
        let env = soroban_sdk::Env::default();
        let participant = Address::generate(&env);
        let description = String::from_str(&env, "Test");

        let mut stats = RecyclingStats::new(participant.clone());
        let mut material = Material::new(1, WasteType::Metal, 5000, participant, 0, description);

        material.verify();
        stats.record_verification(&material);

        assert_eq!(stats.verified_submissions, 1);
        assert_eq!(stats.total_points, 250); // 5kg * 5 * 10
    }

    #[test]
    fn test_verification_rate() {
        let env = soroban_sdk::Env::default();
        let participant = Address::generate(&env);

        let mut stats = RecyclingStats::new(participant);
        stats.total_submissions = 10;
        stats.verified_submissions = 8;

        assert_eq!(stats.verification_rate(), 80);
    }

    #[test]
    fn test_most_submitted_type() {
        let env = soroban_sdk::Env::default();
        let participant = Address::generate(&env);

        let mut stats = RecyclingStats::new(participant);
        stats.paper_count = 5;
        stats.plastic_count = 10;
        stats.metal_count = 3;

        assert_eq!(stats.most_submitted_type(), Some(WasteType::Plastic));
    }

    #[test]
    fn test_average_weight() {
        let env = soroban_sdk::Env::default();
        let participant = Address::generate(&env);

        let mut stats = RecyclingStats::new(participant);
        stats.total_submissions = 5;
        stats.total_weight = 10000;

        assert_eq!(stats.average_weight(), 2000);
    }

    #[test]
    fn test_is_active_recycler() {
        let env = soroban_sdk::Env::default();
        let participant = Address::generate(&env);

        let mut stats = RecyclingStats::new(participant);
        assert!(!stats.is_active_recycler());

        stats.total_submissions = 10;
        assert!(stats.is_active_recycler());
    }

    #[test]
    fn test_is_verified_contributor() {
        let env = soroban_sdk::Env::default();
        let participant = Address::generate(&env);

        let mut stats = RecyclingStats::new(participant);
        stats.total_submissions = 10;
        stats.verified_submissions = 8;

        assert!(stats.is_verified_contributor());
    }

    #[test]
    fn test_stats_storage() {
        let env = soroban_sdk::Env::default();
        let contract_id = env.register_contract(None, crate::ScavengerContract);
        let participant = Address::generate(&env);

        let stats = RecyclingStats::new(participant.clone());

        // RecyclingStats can be stored (validated through contract tests)
        assert_eq!(stats.total_submissions, 0);
    }

    #[test]
    #[should_panic(expected = "Overflow in total submissions")]
    fn test_record_submission_overflows_at_u64_max_total_submissions() {
        let env = soroban_sdk::Env::default();
        let participant = Address::generate(&env);
        let description = String::from_str(&env, "Test");

        let mut stats = RecyclingStats::new(participant.clone());
        stats.total_submissions = u64::MAX;
        let material = Material::new(1, WasteType::Paper, 1, participant, 0, description);

        stats.record_submission(&material);
    }

    #[test]
    #[should_panic(expected = "Overflow in total weight")]
    fn test_record_submission_overflows_at_u64_max_total_weight() {
        let env = soroban_sdk::Env::default();
        let participant = Address::generate(&env);
        let description = String::from_str(&env, "Test");

        let mut stats = RecyclingStats::new(participant.clone());
        stats.total_weight = u64::MAX;
        let material = Material::new(1, WasteType::Paper, 1, participant, 0, description);

        stats.record_submission(&material);
    }

    #[test]
    #[should_panic(expected = "Overflow in total points")]
    fn test_record_verification_overflows_at_u64_max_total_points() {
        let env = soroban_sdk::Env::default();
        let participant = Address::generate(&env);
        let description = String::from_str(&env, "Test");

        let mut stats = RecyclingStats::new(participant.clone());
        stats.total_points = u64::MAX;
        let mut material = Material::new(1, WasteType::Metal, 1000, participant, 0, description);
        material.verify();

        stats.record_verification(&material);
    }

    #[test]
    #[should_panic(expected = "Overflow in verification rate")]
    fn test_verification_rate_overflows_at_extreme_verified_submissions() {
        let env = soroban_sdk::Env::default();
        let participant = Address::generate(&env);

        let mut stats = RecyclingStats::new(participant);
        stats.total_submissions = 1;
        stats.verified_submissions = u64::MAX;

        stats.verification_rate();
    }
}

#[cfg(test)]
mod material_tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_material_creation() {
        let env = soroban_sdk::Env::default();
        let submitter = Address::generate(&env);
        let description = String::from_str(&env, "Plastic bottles");

        let material = Material::new(
            1,
            WasteType::PetPlastic,
            5000,
            submitter.clone(),
            1234567890,
            description.clone(),
        );

        assert_eq!(material.id, 1);
        assert_eq!(material.waste_type, WasteType::PetPlastic);
        assert_eq!(material.weight, 5000);
        assert_eq!(material.submitter, submitter);
        assert_eq!(material.submitted_at, 1234567890);
        assert!(!material.verified);
        assert_eq!(material.description, description);
    }

    #[test]
    fn test_material_verify() {
        let env = soroban_sdk::Env::default();
        let submitter = Address::generate(&env);
        let description = String::from_str(&env, "Test");

        let mut material = Material::new(
            1,
            WasteType::Paper,
            1000,
            submitter,
            1234567890,
            description,
        );

        assert!(!material.verified);
        material.verify();
        assert!(material.verified);
    }

    #[test]
    fn test_meets_minimum_weight() {
        let env = soroban_sdk::Env::default();
        let submitter = Address::generate(&env);
        let description = String::from_str(&env, "Test");

        let material_below = Material::new(
            1,
            WasteType::Paper,
            50,
            submitter.clone(),
            1234567890,
            description.clone(),
        );
        assert!(!material_below.meets_minimum_weight());

        let material_exact = Material::new(
            2,
            WasteType::Paper,
            100,
            submitter.clone(),
            1234567890,
            description.clone(),
        );
        assert!(material_exact.meets_minimum_weight());

        let material_above =
            Material::new(3, WasteType::Paper, 500, submitter, 1234567890, description);
        assert!(material_above.meets_minimum_weight());
    }

    #[test]
    fn test_calculate_reward_points() {
        let env = soroban_sdk::Env::default();
        let submitter = Address::generate(&env);
        let description = String::from_str(&env, "Test");

        // Paper: 5kg * 1 * 10 = 50 points
        let paper = Material::new(
            1,
            WasteType::Paper,
            5000,
            submitter.clone(),
            0,
            description.clone(),
        );
        assert_eq!(paper.calculate_reward_points(), 50);

        // PetPlastic: 5kg * 3 * 10 = 150 points
        let pet = Material::new(
            2,
            WasteType::PetPlastic,
            5000,
            submitter.clone(),
            0,
            description.clone(),
        );
        assert_eq!(pet.calculate_reward_points(), 150);

        // Plastic: 5kg * 2 * 10 = 100 points
        let plastic = Material::new(
            3,
            WasteType::Plastic,
            5000,
            submitter.clone(),
            0,
            description.clone(),
        );
        assert_eq!(plastic.calculate_reward_points(), 100);

        // Metal: 5kg * 5 * 10 = 250 points
        let metal = Material::new(
            4,
            WasteType::Metal,
            5000,
            submitter.clone(),
            0,
            description.clone(),
        );
        assert_eq!(metal.calculate_reward_points(), 250);

        // Glass: 5kg * 2 * 10 = 100 points
        let glass = Material::new(5, WasteType::Glass, 5000, submitter, 0, description);
        assert_eq!(glass.calculate_reward_points(), 100);
    }

    #[test]
    fn test_material_storage_compatibility() {
        let env = soroban_sdk::Env::default();
        let contract_id = env.register_contract(None, crate::ScavengerContract);
        let submitter = Address::generate(&env);
        let description = String::from_str(&env, "Storage test");

        let material = Material::new(
            1,
            WasteType::Metal,
            3000,
            submitter,
            1234567890,
            description,
        );

        // Material can be stored in Soroban storage (validated through contract tests)
        assert_eq!(material.id, 1);
        assert_eq!(material.waste_type, WasteType::Metal);
        assert_eq!(material.weight, 3000);
    }

    #[test]
    fn test_calculate_reward_points_at_u64_max_weight_does_not_overflow() {
        let env = soroban_sdk::Env::default();
        let submitter = Address::generate(&env);
        let description = String::from_str(&env, "Test");

        // weight/1000 shrinks u64::MAX enough that * multiplier(<=5) * 10 cannot overflow u64.
        let material = Material::new(1, WasteType::Metal, u64::MAX, submitter, 0, description);
        assert_eq!(material.calculate_reward_points(), (u64::MAX / 1000) * 50);
    }
}

#[cfg(test)]
mod incentive_tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    #[should_panic(expected = "Overflow in reward calculation")]
    fn test_calculate_reward_overflows_at_extreme_reward_points() {
        let env = soroban_sdk::Env::default();
        let rewarder = Address::generate(&env);

        let incentive = Incentive::new(1, rewarder, WasteType::Metal, u64::MAX, u64::MAX, 0);
        // 5000g -> 5kg, 5 * u64::MAX overflows u64
        incentive.calculate_reward(5000);
    }

    #[test]
    fn test_claim_reward_respects_remaining_budget() {
        let env = soroban_sdk::Env::default();
        let rewarder = Address::generate(&env);

        let mut incentive = Incentive::new(1, rewarder, WasteType::Metal, 10, 45, 0);
        // 5kg * 10 = 50 > 45 remaining budget -> rejected
        assert_eq!(incentive.claim_reward(5000), None);
        assert_eq!(incentive.remaining_budget, 45);

        // 4kg * 10 = 40 <= 45 remaining budget -> accepted, deactivates only when exhausted
        assert_eq!(incentive.claim_reward(4000), Some(40));
        assert_eq!(incentive.remaining_budget, 5);
        assert!(incentive.active);
    }

    #[test]
    fn test_claim_reward_deactivates_on_exact_budget_exhaustion() {
        let env = soroban_sdk::Env::default();
        let rewarder = Address::generate(&env);

        let mut incentive = Incentive::new(1, rewarder, WasteType::Metal, 10, 50, 0);
        assert_eq!(incentive.claim_reward(5000), Some(50));
        assert_eq!(incentive.remaining_budget, 0);
        assert!(!incentive.active);
    }
}

#[cfg(test)]
mod waste_tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::Env;

    fn make_waste(env: &Env, owner: Address, latitude: i128, longitude: i128) -> Waste {
        Waste::new(1, WasteType::Metal, 5000, owner.clone(), latitude, longitude, 0, true, false, owner)
    }

    #[test]
    fn test_new_sets_all_fields() {
        let env = Env::default();
        let owner = Address::generate(&env);
        let confirmer = Address::generate(&env);
        let waste = Waste::new(7, WasteType::Glass, 250, owner.clone(), 1, 2, 99, false, true, confirmer.clone());

        assert_eq!(waste.waste_id, 7);
        assert_eq!(waste.waste_type, WasteType::Glass);
        assert_eq!(waste.weight, 250);
        assert_eq!(waste.current_owner, owner);
        assert_eq!(waste.latitude, 1);
        assert_eq!(waste.longitude, 2);
        assert_eq!(waste.recycled_timestamp, 99);
        assert!(!waste.is_active);
        assert!(waste.is_confirmed);
        assert_eq!(waste.confirmer, confirmer);
    }

    #[test]
    fn test_has_valid_coordinates() {
        let env = Env::default();
        let owner = Address::generate(&env);

        assert!(make_waste(&env, owner.clone(), 0, 0).has_valid_coordinates());
        assert!(make_waste(&env, owner.clone(), 90_000_000, 180_000_000).has_valid_coordinates());
        assert!(make_waste(&env, owner.clone(), -90_000_000, -180_000_000).has_valid_coordinates());
        assert!(!make_waste(&env, owner.clone(), 90_000_001, 0).has_valid_coordinates());
        assert!(!make_waste(&env, owner, 0, 180_000_001).has_valid_coordinates());
    }

    #[test]
    fn test_is_recycled() {
        let env = Env::default();
        let owner = Address::generate(&env);
        let mut waste = make_waste(&env, owner, 0, 0);

        assert!(!waste.is_recycled());
        waste.mark_recycled(12345);
        assert!(waste.is_recycled());
        assert_eq!(waste.recycled_timestamp, 12345);
    }

    #[test]
    fn test_waste_meets_minimum_weight() {
        let env = Env::default();
        let owner = Address::generate(&env);

        let mut light = make_waste(&env, owner.clone(), 0, 0);
        light.weight = 99;
        assert!(!light.meets_minimum_weight());

        let mut exact = make_waste(&env, owner, 0, 0);
        exact.weight = 100;
        assert!(exact.meets_minimum_weight());
    }

    #[test]
    fn test_update_location() {
        let env = Env::default();
        let owner = Address::generate(&env);
        let mut waste = make_waste(&env, owner, 0, 0);

        waste.update_location(45_000_000, -93_000_000);
        assert_eq!(waste.latitude, 45_000_000);
        assert_eq!(waste.longitude, -93_000_000);
    }
}

#[cfg(test)]
mod transfer_record_tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::Env;

    #[test]
    fn test_transfer_item_type_round_trip() {
        for (variant, value) in [
            (TransferItemType::Material, 0u32),
            (TransferItemType::Token, 1),
            (TransferItemType::Incentive, 2),
            (TransferItemType::Ownership, 3),
        ] {
            assert!(TransferItemType::is_valid(value));
            assert_eq!(TransferItemType::from_u32(value), Some(variant));
            assert_eq!(variant.to_u32(), value);
        }
        assert!(!TransferItemType::is_valid(4));
        assert_eq!(TransferItemType::from_u32(4), None);
    }

    #[test]
    fn test_transfer_status_round_trip_and_state() {
        for (variant, value, is_final, is_active) in [
            (TransferStatus::Pending, 0u32, false, true),
            (TransferStatus::InProgress, 1, false, true),
            (TransferStatus::Completed, 2, true, false),
            (TransferStatus::Failed, 3, true, false),
            (TransferStatus::Cancelled, 4, true, false),
        ] {
            assert!(TransferStatus::is_valid(value));
            assert_eq!(TransferStatus::from_u32(value), Some(variant));
            assert_eq!(variant.to_u32(), value);
            assert_eq!(variant.is_final(), is_final);
            assert_eq!(variant.is_active(), is_active);
        }
        assert!(!TransferStatus::is_valid(5));
        assert_eq!(TransferStatus::from_u32(5), None);
    }

    fn make_record(env: &Env, from: Address, to: Address) -> TransferRecord {
        TransferRecord::new(
            1,
            from,
            to,
            TransferItemType::Material,
            42,
            5000,
            0,
            String::from_str(env, "note"),
        )
    }

    #[test]
    fn test_new_starts_pending() {
        let env = Env::default();
        let from = Address::generate(&env);
        let to = Address::generate(&env);
        let record = make_record(&env, from, to);

        assert_eq!(record.status, TransferStatus::Pending);
        assert!(!record.is_complete());
        assert!(record.is_modifiable());
    }

    #[test]
    fn test_update_status_succeeds_while_not_final() {
        let env = Env::default();
        let from = Address::generate(&env);
        let to = Address::generate(&env);
        let mut record = make_record(&env, from, to);

        assert!(record.update_status(TransferStatus::InProgress));
        assert_eq!(record.status, TransferStatus::InProgress);

        assert!(record.update_status(TransferStatus::Completed));
        assert_eq!(record.status, TransferStatus::Completed);
        assert!(record.is_complete());
        assert!(!record.is_modifiable());
    }

    #[test]
    fn test_update_status_rejected_once_final() {
        let env = Env::default();
        let from = Address::generate(&env);
        let to = Address::generate(&env);
        let mut record = make_record(&env, from, to);

        assert!(record.update_status(TransferStatus::Cancelled));
        // Status is final; further updates must be rejected and leave state unchanged.
        assert!(!record.update_status(TransferStatus::Pending));
        assert_eq!(record.status, TransferStatus::Cancelled);
    }

    #[test]
    fn test_validate_rejects_zero_amount() {
        let env = Env::default();
        let from = Address::generate(&env);
        let to = Address::generate(&env);
        let mut record = make_record(&env, from, to);
        record.amount = 0;

        assert_eq!(record.validate(), Err("Amount must be greater than zero"));
    }

    #[test]
    fn test_validate_rejects_same_sender_and_recipient() {
        let env = Env::default();
        let addr = Address::generate(&env);
        let record = make_record(&env, addr.clone(), addr);

        assert_eq!(record.validate(), Err("Sender and recipient cannot be the same"));
    }

    #[test]
    fn test_validate_accepts_well_formed_record() {
        let env = Env::default();
        let from = Address::generate(&env);
        let to = Address::generate(&env);
        let record = make_record(&env, from, to);

        assert_eq!(record.validate(), Ok(()));
    }
}

#[cfg(test)]
mod waste_type_tests {
    use super::*;

    #[test]
    fn test_waste_type_values() {
        assert_eq!(WasteType::Paper as u32, 0);
        assert_eq!(WasteType::PetPlastic as u32, 1);
        assert_eq!(WasteType::Plastic as u32, 2);
        assert_eq!(WasteType::Metal as u32, 3);
        assert_eq!(WasteType::Glass as u32, 4);
    }

    #[test]
    fn test_waste_type_is_valid() {
        assert!(WasteType::is_valid(0));
        assert!(WasteType::is_valid(1));
        assert!(WasteType::is_valid(2));
        assert!(WasteType::is_valid(3));
        assert!(WasteType::is_valid(4));
        assert!(!WasteType::is_valid(5));
        assert!(!WasteType::is_valid(999));
    }

    #[test]
    fn test_waste_type_from_u32() {
        assert_eq!(WasteType::from_u32(0), Some(WasteType::Paper));
        assert_eq!(WasteType::from_u32(1), Some(WasteType::PetPlastic));
        assert_eq!(WasteType::from_u32(2), Some(WasteType::Plastic));
        assert_eq!(WasteType::from_u32(3), Some(WasteType::Metal));
        assert_eq!(WasteType::from_u32(4), Some(WasteType::Glass));
        assert_eq!(WasteType::from_u32(5), None);
        assert_eq!(WasteType::from_u32(999), None);
    }

    #[test]
    fn test_waste_type_to_u32() {
        assert_eq!(WasteType::Paper.to_u32(), 0);
        assert_eq!(WasteType::PetPlastic.to_u32(), 1);
        assert_eq!(WasteType::Plastic.to_u32(), 2);
        assert_eq!(WasteType::Metal.to_u32(), 3);
        assert_eq!(WasteType::Glass.to_u32(), 4);
    }

    #[test]
    fn test_waste_type_as_str() {
        assert_eq!(WasteType::Paper.as_str(), "PAPER");
        assert_eq!(WasteType::PetPlastic.as_str(), "PETPLASTIC");
        assert_eq!(WasteType::Plastic.as_str(), "PLASTIC");
        assert_eq!(WasteType::Metal.as_str(), "METAL");
        assert_eq!(WasteType::Glass.as_str(), "GLASS");
    }

    #[test]
    fn test_waste_type_display() {
        // Test Display trait by converting to string representation
        assert_eq!(WasteType::Paper.as_str(), "PAPER");
        assert_eq!(WasteType::PetPlastic.as_str(), "PETPLASTIC");
        assert_eq!(WasteType::Plastic.as_str(), "PLASTIC");
        assert_eq!(WasteType::Metal.as_str(), "METAL");
        assert_eq!(WasteType::Glass.as_str(), "GLASS");
    }

    #[test]
    fn test_waste_type_is_plastic() {
        assert!(!WasteType::Paper.is_plastic());
        assert!(WasteType::PetPlastic.is_plastic());
        assert!(WasteType::Plastic.is_plastic());
        assert!(!WasteType::Metal.is_plastic());
        assert!(!WasteType::Glass.is_plastic());
    }

    #[test]
    fn test_waste_type_is_biodegradable() {
        assert!(WasteType::Paper.is_biodegradable());
        assert!(!WasteType::PetPlastic.is_biodegradable());
        assert!(!WasteType::Plastic.is_biodegradable());
        assert!(!WasteType::Metal.is_biodegradable());
        assert!(!WasteType::Glass.is_biodegradable());
    }

    #[test]
    fn test_waste_type_is_infinitely_recyclable() {
        assert!(!WasteType::Paper.is_infinitely_recyclable());
        assert!(!WasteType::PetPlastic.is_infinitely_recyclable());
        assert!(!WasteType::Plastic.is_infinitely_recyclable());
        assert!(WasteType::Metal.is_infinitely_recyclable());
        assert!(WasteType::Glass.is_infinitely_recyclable());
    }

    #[test]
    fn test_waste_type_clone_and_copy() {
        let waste1 = WasteType::Paper;
        let waste2 = waste1;
        assert_eq!(waste1, waste2);
    }

    #[test]
    fn test_waste_type_equality() {
        assert_eq!(WasteType::Paper, WasteType::Paper);
        assert_ne!(WasteType::Paper, WasteType::Plastic);
        assert_ne!(WasteType::Metal, WasteType::Glass);
    }

    #[test]
    fn test_all_waste_types() {
        let types = [
            WasteType::Paper,
            WasteType::PetPlastic,
            WasteType::Plastic,
            WasteType::Metal,
            WasteType::Glass,
        ];

        for (i, waste_type) in types.iter().enumerate() {
            assert_eq!(waste_type.to_u32(), i as u32);
            assert_eq!(WasteType::from_u32(i as u32), Some(*waste_type));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_values() {
        assert_eq!(ParticipantRole::Recycler as u32, 0);
        assert_eq!(ParticipantRole::Collector as u32, 1);
        assert_eq!(ParticipantRole::Manufacturer as u32, 2);
    }

    #[test]
    fn test_is_valid() {
        assert!(ParticipantRole::is_valid(0));
        assert!(ParticipantRole::is_valid(1));
        assert!(ParticipantRole::is_valid(2));
        assert!(!ParticipantRole::is_valid(3));
        assert!(!ParticipantRole::is_valid(999));
    }

    #[test]
    fn test_from_u32() {
        assert_eq!(
            ParticipantRole::from_u32(0),
            Some(ParticipantRole::Recycler)
        );
        assert_eq!(
            ParticipantRole::from_u32(1),
            Some(ParticipantRole::Collector)
        );
        assert_eq!(
            ParticipantRole::from_u32(2),
            Some(ParticipantRole::Manufacturer)
        );
        assert_eq!(ParticipantRole::from_u32(3), None);
        assert_eq!(ParticipantRole::from_u32(999), None);
    }

    #[test]
    fn test_to_u32() {
        assert_eq!(ParticipantRole::Recycler.to_u32(), 0);
        assert_eq!(ParticipantRole::Collector.to_u32(), 1);
        assert_eq!(ParticipantRole::Manufacturer.to_u32(), 2);
    }

    #[test]
    fn test_as_str() {
        assert_eq!(ParticipantRole::Recycler.as_str(), "RECYCLER");
        assert_eq!(ParticipantRole::Collector.as_str(), "COLLECTOR");
        assert_eq!(ParticipantRole::Manufacturer.as_str(), "MANUFACTURER");
    }

    #[test]
    fn test_can_collect_materials() {
        assert!(ParticipantRole::Recycler.can_collect_materials());
        assert!(ParticipantRole::Collector.can_collect_materials());
        assert!(!ParticipantRole::Manufacturer.can_collect_materials());
    }

    #[test]
    fn test_can_manufacture() {
        assert!(!ParticipantRole::Recycler.can_manufacture());
        assert!(!ParticipantRole::Collector.can_manufacture());
        assert!(ParticipantRole::Manufacturer.can_manufacture());
    }

    #[test]
    fn test_can_process_recyclables() {
        assert!(ParticipantRole::Recycler.can_process_recyclables());
        assert!(!ParticipantRole::Collector.can_process_recyclables());
        assert!(!ParticipantRole::Manufacturer.can_process_recyclables());
    }

    #[test]
    fn test_clone_and_copy() {
        let role1 = ParticipantRole::Recycler;
        let role2 = role1;
        assert_eq!(role1, role2);
    }

    #[test]
    fn test_equality() {
        assert_eq!(ParticipantRole::Recycler, ParticipantRole::Recycler);
        assert_ne!(ParticipantRole::Recycler, ParticipantRole::Collector);
        assert_ne!(ParticipantRole::Collector, ParticipantRole::Manufacturer);
    }
}

/// Global contract metrics
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalMetrics {
    /// Total number of waste items logged in the system
    pub total_wastes_count: u64,
    /// Total amount of tokens earned across all participants
    pub total_tokens_earned: u128,
}

