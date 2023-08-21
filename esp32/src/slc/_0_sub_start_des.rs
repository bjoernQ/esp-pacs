#[doc = "Register `_0_SUB_START_DES` reader"]
pub type R = crate::R<_0_SUB_START_DES_SPEC>;
#[doc = "Field `SLC0_SUB_PAC_START_DSCR_ADDR` reader - "]
pub type SLC0_SUB_PAC_START_DSCR_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc0_sub_pac_start_dscr_addr(&self) -> SLC0_SUB_PAC_START_DSCR_ADDR_R {
        SLC0_SUB_PAC_START_DSCR_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0_SUB_START_DES")
            .field(
                "slc0_sub_pac_start_dscr_addr",
                &format_args!("{}", self.slc0_sub_pac_start_dscr_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_0_SUB_START_DES_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_sub_start_des::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _0_SUB_START_DES_SPEC;
impl crate::RegisterSpec for _0_SUB_START_DES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_0_sub_start_des::R`](R) reader structure"]
impl crate::Readable for _0_SUB_START_DES_SPEC {}
#[doc = "`reset()` method sets _0_SUB_START_DES to value 0"]
impl crate::Resettable for _0_SUB_START_DES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
