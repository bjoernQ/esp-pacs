#[doc = "Register `L1_DBUS2_ACS_NXTLVL_CNT` reader"]
pub struct R(crate::R<L1_DBUS2_ACS_NXTLVL_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_DBUS2_ACS_NXTLVL_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_DBUS2_ACS_NXTLVL_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_DBUS2_ACS_NXTLVL_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L1_DBUS2_NXTLVL_CNT` reader - The register records the number of times that L1-DCache accesses L2-Cache due to bus2 accessing L1-DCache."]
pub type L1_DBUS2_NXTLVL_CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of times that L1-DCache accesses L2-Cache due to bus2 accessing L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus2_nxtlvl_cnt(&self) -> L1_DBUS2_NXTLVL_CNT_R {
        L1_DBUS2_NXTLVL_CNT_R::new(self.bits)
    }
}
#[doc = "L1-DCache bus2 Next-Level-Access Counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_dbus2_acs_nxtlvl_cnt](index.html) module"]
pub struct L1_DBUS2_ACS_NXTLVL_CNT_SPEC;
impl crate::RegisterSpec for L1_DBUS2_ACS_NXTLVL_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_dbus2_acs_nxtlvl_cnt::R](R) reader structure"]
impl crate::Readable for L1_DBUS2_ACS_NXTLVL_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L1_DBUS2_ACS_NXTLVL_CNT to value 0"]
impl crate::Resettable for L1_DBUS2_ACS_NXTLVL_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
