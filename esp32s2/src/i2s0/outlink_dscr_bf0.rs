#[doc = "Register `OUTLINK_DSCR_BF0` reader"]
pub struct R(crate::R<OUTLINK_DSCR_BF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTLINK_DSCR_BF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTLINK_DSCR_BF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTLINK_DSCR_BF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUTLINK_DSCR_BF0` reader - The address of next outlink descriptor."]
pub type OUTLINK_DSCR_BF0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The address of next outlink descriptor."]
    #[inline(always)]
    pub fn outlink_dscr_bf0(&self) -> OUTLINK_DSCR_BF0_R {
        OUTLINK_DSCR_BF0_R::new(self.bits)
    }
}
#[doc = "Address of next outlink descriptor\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outlink_dscr_bf0](index.html) module"]
pub struct OUTLINK_DSCR_BF0_SPEC;
impl crate::RegisterSpec for OUTLINK_DSCR_BF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outlink_dscr_bf0::R](R) reader structure"]
impl crate::Readable for OUTLINK_DSCR_BF0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUTLINK_DSCR_BF0 to value 0"]
impl crate::Resettable for OUTLINK_DSCR_BF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}