use crate::abi::curve::child_registries::{
    crv_usd_pool_factory, pool_registry_v1, stable_swap_factory_ng,
};

mod sealed {
    // The `PlainPoolDeployedEventSealed` trait is private and acts as a seal to restrict
    // external implementations of the `PlainPoolDeployedEvent` trait, ensuring that only
    // specific event types within this module can implement it.
    pub trait PlainPoolDeployedEventSealed {}
}

// The `PlainPoolDeployedEvent` trait is designed to handle events related to the deployment
// of plain pools. By being bounded by the sealed trait `PlainPoolDeployedEventSealed`,
// it ensures that only the intended event types from specific ABIs can implement it.
// This pattern allows for a unified processing logic for similar events from different ABIs
// while maintaining strict type safety.
pub trait PlainPoolDeployedEvent: sealed::PlainPoolDeployedEventSealed {}

// Implementation of the sealed trait for specific event types, effectively allowing them
// to implement the `PlainPoolDeployedEvent` trait and be processed using a common logic.
impl sealed::PlainPoolDeployedEventSealed for crv_usd_pool_factory::events::PlainPoolDeployed {}
impl PlainPoolDeployedEvent for crv_usd_pool_factory::events::PlainPoolDeployed {}

impl sealed::PlainPoolDeployedEventSealed for stable_swap_factory_ng::events::PlainPoolDeployed {}
impl PlainPoolDeployedEvent for stable_swap_factory_ng::events::PlainPoolDeployed {}

impl sealed::PlainPoolDeployedEventSealed for pool_registry_v1::events::PlainPoolDeployed {}
impl PlainPoolDeployedEvent for pool_registry_v1::events::PlainPoolDeployed {}
