dharitri_sc::imports!();

#[dharitri_sc::module]
pub trait OnlyAdminDerivedTestModule {
    #[view]
    fn call_derived_not_admin_only(&self) {}
}
