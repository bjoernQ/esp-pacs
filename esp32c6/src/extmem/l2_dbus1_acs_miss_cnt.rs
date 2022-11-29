#[doc = "Register `L2_DBUS1_ACS_MISS_CNT` reader"]
pub struct R(crate::R<L2_DBUS1_ACS_MISS_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_DBUS1_ACS_MISS_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_DBUS1_ACS_MISS_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_DBUS1_ACS_MISS_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_DBUS1_MISS_CNT` reader - The register records the number of missing when L1-DCache accesses L2-Cache due to bus1 accessing L1-DCache."]
pub type L2_DBUS1_MISS_CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of missing when L1-DCache accesses L2-Cache due to bus1 accessing L1-DCache."]
    #[inline(always)]
    pub fn l2_dbus1_miss_cnt(&self) -> L2_DBUS1_MISS_CNT_R {
        L2_DBUS1_MISS_CNT_R::new(self.bits)
    }
}
#[doc = "L2-Cache bus1 Miss-Access Counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_dbus1_acs_miss_cnt](index.html) module"]
pub struct L2_DBUS1_ACS_MISS_CNT_SPEC;
impl crate::RegisterSpec for L2_DBUS1_ACS_MISS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_dbus1_acs_miss_cnt::R](R) reader structure"]
impl crate::Readable for L2_DBUS1_ACS_MISS_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_DBUS1_ACS_MISS_CNT to value 0"]
impl crate::Resettable for L2_DBUS1_ACS_MISS_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}