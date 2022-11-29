#[doc = "Register `L2_CACHE_AUTOLOAD_SCT1_SIZE` reader"]
pub struct R(crate::R<L2_CACHE_AUTOLOAD_SCT1_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_AUTOLOAD_SCT1_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_AUTOLOAD_SCT1_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_AUTOLOAD_SCT1_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_AUTOLOAD_SCT1_SIZE` reader - Those bits are used to configure the size of the second section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT1_ADDR and L2_CACHE_AUTOLOAD_SCT1_ENA."]
pub type L2_CACHE_AUTOLOAD_SCT1_SIZE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the second section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT1_ADDR and L2_CACHE_AUTOLOAD_SCT1_ENA."]
    #[inline(always)]
    pub fn l2_cache_autoload_sct1_size(&self) -> L2_CACHE_AUTOLOAD_SCT1_SIZE_R {
        L2_CACHE_AUTOLOAD_SCT1_SIZE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[doc = "L2 Cache autoload section 1 size configure register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_autoload_sct1_size](index.html) module"]
pub struct L2_CACHE_AUTOLOAD_SCT1_SIZE_SPEC;
impl crate::RegisterSpec for L2_CACHE_AUTOLOAD_SCT1_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_autoload_sct1_size::R](R) reader structure"]
impl crate::Readable for L2_CACHE_AUTOLOAD_SCT1_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_AUTOLOAD_SCT1_SIZE to value 0"]
impl crate::Resettable for L2_CACHE_AUTOLOAD_SCT1_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}